use super::packets::*;
use crate::network::*;
use crate::packet::PacketContainer;
use basin2_lib::result::*;
use basin2_lib::McProtoBase;
use bytes::buf::Buf;
use bytes::BytesMut;
use flate2::{write::DeflateDecoder, write::DeflateEncoder, Compression};
use futures::future::FutureExt;
use futures::pin_mut;
use futures::select;
use futures::stream::StreamExt;
use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use log::*;
use openssl::symm::{Cipher, Crypter, Mode};
use std::fmt;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::{Mutex, RwLock};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::oneshot;
use tokio_util::codec::{Decoder, Encoder, Framed};

pub struct McCodec {
    read_cipher: Option<Mutex<(Crypter, BytesMut)>>,
    write_cipher: Option<Mutex<Crypter>>,
    pub threshold: Option<u32>,
    pub state: ConnectionProtocol,
    pub is_server: bool,
}

#[derive(Clone)]
pub struct WrappedMcCodec(pub Arc<RwLock<McCodec>>);

pub struct Connection {
    pub incoming: Mutex<Option<Receiver<(Option<oneshot::Sender<()>>, Packet)>>>,
    pub outgoing: Mutex<Sender<(Option<oneshot::Sender<()>>, Option<Packet>)>>,
    pub address: SocketAddr,
    pub codec: WrappedMcCodec,
}

pub type WrappedConnection = Arc<Connection>;

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Connection {{ address: {:?} }}", self.address)
    }
}

impl McCodec {
    fn finish_decode(&self, buf: &mut BytesMut) -> Result<Packet> {
        let packet_id = buf.get_mc_var_int()?;
        let decoded = self.finish_decode_packet(buf, packet_id);
        match decoded {
            Ok(packet) => Ok(packet),
            Err(e) => Err(basin_err!("failed to decode packet id {} from {}<{:?}>: {:?}", packet_id, if self.is_server { "server" } else { "client" }, self.state, e)),
        }
    }

    fn finish_decode_packet(&self, buf: &mut BytesMut, packet_id: i32) -> Result<Packet> {
        Ok(match (self.state, self.is_server) {
            (ConnectionProtocol::Handshake, true) => Packet::Handshake(
                Handshake::HandshakeServerbound(HandshakeServerbound::decode(packet_id, buf)?),
            ),
            (ConnectionProtocol::Handshake, false) => Packet::Handshake(
                Handshake::HandshakeClientbound(HandshakeClientbound::decode(packet_id, buf)?),
            ),
            (ConnectionProtocol::Status, true) => Packet::Status(Status::StatusServerbound(
                StatusServerbound::decode(packet_id, buf)?,
            )),
            (ConnectionProtocol::Status, false) => Packet::Status(Status::StatusClientbound(
                StatusClientbound::decode(packet_id, buf)?,
            )),
            (ConnectionProtocol::Login, true) => Packet::Login(Login::LoginServerbound(
                LoginServerbound::decode(packet_id, buf)?,
            )),
            (ConnectionProtocol::Login, false) => Packet::Login(Login::LoginClientbound(
                LoginClientbound::decode(packet_id, buf)?,
            )),
            (ConnectionProtocol::Game, true) => Packet::Game(Game::GameServerbound(
                GameServerbound::decode(packet_id, buf)?,
            )),
            (ConnectionProtocol::Game, false) => Packet::Game(Game::GameClientbound(
                GameClientbound::decode(packet_id, buf)?,
            )),
        })
    }

    pub fn init_encryption(&mut self, key: &[u8]) -> Result<()> {
        self.read_cipher = Some(Mutex::new((
            Crypter::new(Cipher::aes_128_cfb8(), Mode::Decrypt, &key, Some(&key))?,
            BytesMut::new(),
        )));
        self.write_cipher = Some(Mutex::new(Crypter::new(
            Cipher::aes_128_cfb8(),
            Mode::Encrypt,
            &key,
            Some(&key),
        )?));
        Ok(())
    }

    fn internal_decipher(
        &self,
        buf: &mut BytesMut,
    ) -> std::result::Result<Option<Packet>, IoError> {
        match &self.read_cipher {
            Some(cipher) => {
                let mut ciphertext = vec![0; buf.len() + 16];
                let mut cipher = cipher.lock().unwrap();
                let count = cipher.0.update(buf, &mut ciphertext).unwrap();
                cipher.1.extend_from_slice(&ciphertext[..count]);
                buf.advance(buf.len());
                self.internal_decode(&mut cipher.1)
            }
            _ => self.internal_decode(buf),
        }
    }

    fn internal_decode(&self, buf: &mut BytesMut) -> std::result::Result<Option<Packet>, IoError> {
        let header_len = if buf.len() > 16 { 16 } else { buf.len() };
        let header_split = buf.split_off(header_len);
        let mut header = buf.clone();
        buf.unsplit(header_split);
        let packet_length = header.get_mc_var_int();
        if packet_length.is_err() {
            return Ok(None);
        }
        let packet_length = packet_length.unwrap();
        if packet_length < 0 {
            return Err(IoError::from(ErrorKind::InvalidData));
        }
        let packet_length = packet_length as usize;
        let header_size = header_len - header.len();
        if buf.len() - header_size < packet_length {
            return Ok(None);
        }
        buf.advance(header_size);
        let mut packet_data = buf.split_to(packet_length);
        if self.threshold.is_some() {
            let decompressed_length = packet_data.get_mc_var_int();
            if decompressed_length.is_err() {
                return Err(IoError::from(ErrorKind::InvalidData));
            }
            let decompressed_length = decompressed_length.unwrap() as usize;
            if decompressed_length > 2097152 {
                return Err(IoError::from(ErrorKind::InvalidData));
            }
            if decompressed_length > 0 {
                let compressed = packet_data;
                let mut deflater = DeflateDecoder::new(vec![]);
                deflater.write_all(&compressed[..])?;
                packet_data = BytesMut::from(&deflater.finish()?[..]);
            }
        }
        let packet = self.finish_decode(&mut packet_data);
        if packet.is_err() {
            error!("Invalid packet for state {:?}: {:?}", self.state, packet.err().unwrap());
            return Err(IoError::from(ErrorKind::InvalidData));
        }
        let packet = packet.unwrap();
        Ok(Some(packet))
    }

    fn internal_encode(
        &self,
        packet: Packet,
        output: &mut BytesMut,
    ) -> std::result::Result<(), IoError> {
        match (self.state, self.is_server, &packet) {
            (ConnectionProtocol::Handshake, true, Packet::Handshake(Handshake::HandshakeClientbound(..))) => (),
            (ConnectionProtocol::Handshake, false, Packet::Handshake(Handshake::HandshakeServerbound(..))) => (),
            (ConnectionProtocol::Status, true, Packet::Status(Status::StatusClientbound(..))) => (),
            (ConnectionProtocol::Status, false, Packet::Status(Status::StatusServerbound(..))) => (),
            (ConnectionProtocol::Login, true, Packet::Login(Login::LoginClientbound(..))) => (),
            (ConnectionProtocol::Login, false, Packet::Login(Login::LoginServerbound(..))) => (),
            (ConnectionProtocol::Game, true, Packet::Game(Game::GameClientbound(..))) => (),
            (ConnectionProtocol::Game, false, Packet::Game(Game::GameServerbound(..))) => (),
            _ => return Err(IoError::from(ErrorKind::InvalidData)),
        }
        let mut encoded = BytesMut::new();
        packet.encode(&mut encoded);
        // compress
        match self.threshold {
            Some(threshold) => {
                let raw_length = get_var_int_len(encoded.len() as i32) + encoded.len();
                if raw_length > threshold as usize + 1 && raw_length <= 2097152 {
                    let decompressed = encoded;
                    let mut deflater = DeflateEncoder::new(vec![], Compression::default());
                    deflater.write_all(&decompressed[..])?;
                    let compressed = BytesMut::from(&deflater.finish()?[..]);
                    encoded = BytesMut::with_capacity(compressed.len() + 10);
                    encoded.set_mc_var_int(
                        (get_var_int_len(decompressed.len() as i32) + compressed.len()) as i32,
                    );
                    encoded.set_mc_var_int(decompressed.len() as i32);
                    encoded.unsplit(compressed);
                } else {
                    let decompressed = encoded;
                    encoded = BytesMut::with_capacity(raw_length + 10);
                    encoded.set_mc_var_int((get_var_int_len(0) + decompressed.len()) as i32);
                    encoded.set_mc_var_int(0);
                    encoded.unsplit(decompressed);
                }
            }
            None => {
                let decompressed = encoded;
                encoded = BytesMut::with_capacity(decompressed.len() + 10);
                encoded.set_mc_var_int(decompressed.len() as i32);
                encoded.unsplit(decompressed);
            }
        };
        // encrypt
        match &self.write_cipher {
            Some(cipher) => {
                output.reserve(encoded.len() + 16);
                let mut indirect_output: Vec<u8> = vec![0; encoded.len() + 16];
                let outlen = cipher
                    .lock()
                    .unwrap()
                    .update(&encoded, &mut indirect_output)
                    .unwrap();
                output.extend_from_slice(&indirect_output[0..outlen]);
            }
            None => {
                output.unsplit(encoded);
            }
        };
        Ok(())
    }
}

impl Decoder for WrappedMcCodec {
    type Item = Packet;
    type Error = IoError;

    fn decode(&mut self, buf: &mut BytesMut) -> std::result::Result<Option<Packet>, IoError> {
        return self.0.read().unwrap().internal_decipher(buf);
    }
}

impl Encoder for WrappedMcCodec {
    type Item = Packet;
    type Error = IoError;

    fn encode(
        &mut self,
        packet: Packet,
        output: &mut BytesMut,
    ) -> std::result::Result<(), IoError> {
        return self.0.read().unwrap().internal_encode(packet, output);
    }
}

async fn writeForward(
    mut incoming: Receiver<(Option<oneshot::Sender<()>>, Option<Packet>)>,
    mut to: SplitSink<Framed<TcpStream, WrappedMcCodec>, Packet>,
) -> Result<()> {
    loop {
        match incoming.recv().await {
            Some((responder, Some(packet))) => {
                //TODO: stop cloning for debug here
                let result = to.send(packet.clone()).await;
                if let Some(responder) = responder {
                    responder.send(()).unwrap_or(());
                }
                if result.is_err() {
                    error!("error sending packet: {:?}, data: {:?}", result, packet);
                    break;
                }
            },
            Some((_, None)) | None => { break; },
        }
    }
    Ok(())
}

async fn readForward(
    mut outgoing: Sender<(Option<oneshot::Sender<()>>, Packet)>,
    mut from: SplitStream<Framed<TcpStream, WrappedMcCodec>>,
) -> Result<()> {
    loop {
        match from.next().await {
            Some(Ok(request)) => {
                let should_block = match &request { // packets that directly affect encoding/decoding need to be processed before we continue
                    Packet::Handshake(Handshake::HandshakeServerbound(HandshakeServerbound::ClientIntentionPacket(_))) => true,
                    Packet::Login(Login::LoginServerbound(LoginServerbound::KeyPacket(_))) => true,
                    Packet::Login(Login::LoginServerbound(LoginServerbound::HelloPacket(_))) => true,
                    _ => false,
                };
                let (sender, receiver) = if should_block {
                    let (sender, receiver) = oneshot::channel::<()>();
                    (Some(sender), Some(receiver))
                } else {
                    (None, None)
                };
                if outgoing.send((sender, request)).await.is_err() {
                    break;
                }
                if receiver.is_some() {
                    receiver.unwrap().await.unwrap_or(());
                }
            },
            Some(Err(e)) => {
                error!("error decoding packet: {:?}", e);
                break;
            },
            None => { break; },
        }
    }
    Ok(())
}

impl Connection {
    pub async fn spawn(
        socket: TcpStream,
        is_server: bool,
        mut handler: Sender<WrappedConnection>,
    ) -> Result<()> {
        let address = socket.peer_addr().unwrap();

        let (in_outgoing, in_incoming) = channel::<(Option<oneshot::Sender<()>>, Packet)>(256);
        let (out_outgoing, out_incoming) = channel::<(Option<oneshot::Sender<()>>, Option<Packet>)>(256);

        let connection = Connection {
            incoming: Mutex::new(Some(in_incoming)),
            outgoing: Mutex::new(out_outgoing),
            address,
            codec: WrappedMcCodec(Arc::new(RwLock::new(McCodec {
                read_cipher: None,
                threshold: None,
                write_cipher: None,
                state: ConnectionProtocol::Handshake,
                is_server,
            }))),
        };
        let framed_stream = Framed::new(socket, connection.codec.clone());
        handler.send(Arc::new(connection)).await?;

        let (writer, reader) = framed_stream.split();

        let write_future = writeForward(out_incoming, writer).fuse();
        let read_future = readForward(in_outgoing, reader).fuse();
        pin_mut!(write_future);
        pin_mut!(read_future);
        let res = select! {
            write_res = write_future => write_res,
            read_res = read_future => read_res,
        };
        return res.and(Ok(()));
    }

    pub fn set_state(&self, state: ConnectionProtocol) {
        self.codec.0.write().unwrap().state = state;
    }

    pub fn init_encryption(&self, key: &[u8]) -> Result<()> {
        self.codec.0.write().unwrap().init_encryption(key)
    }

    pub fn set_compression(&self, threshold: u32) {
        self.codec.0.write().unwrap().threshold = Some(threshold);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_serialization_plain() -> Result<()> {
        let mut codec = McCodec {
            read_cipher: None,
            write_cipher: None,
            threshold: None,
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        let test_packet = Packet::Handshake(Handshake::HandshakeServerbound(
            HandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        codec.internal_encode(test_packet.clone(), &mut output)?;
        codec.is_server = true;

        let decoded = codec.internal_decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_compressed() -> Result<()> {
        let mut codec = McCodec {
            read_cipher: None,
            write_cipher: None,
            threshold: Some(0),
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        let test_packet = Packet::Handshake(Handshake::HandshakeServerbound(
            HandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        codec.internal_encode(test_packet.clone(), &mut output)?;
        codec.is_server = true;

        let decoded = codec.internal_decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_compressible() -> Result<()> {
        let mut codec = McCodec {
            read_cipher: None,
            write_cipher: None,
            threshold: Some(256),
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        let test_packet = Packet::Handshake(Handshake::HandshakeServerbound(
            HandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        codec.internal_encode(test_packet.clone(), &mut output)?;
        codec.is_server = true;

        let decoded = codec.internal_decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_encrypted() -> Result<()> {
        let key = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut codec = McCodec {
            read_cipher: None,
            write_cipher: None,
            threshold: None,
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        codec.init_encryption(&key)?;
        let test_packet = Packet::Handshake(Handshake::HandshakeServerbound(
            HandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        codec.internal_encode(test_packet.clone(), &mut output)?;
        codec.is_server = true;

        let decoded = codec.internal_decipher(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_encrypted_compressed() -> Result<()> {
        let key = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut codec = McCodec {
            read_cipher: None,
            write_cipher: None,
            threshold: Some(0),
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        codec.init_encryption(&key)?;
        let test_packet = Packet::Handshake(Handshake::HandshakeServerbound(
            HandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        codec.internal_encode(test_packet.clone(), &mut output)?;
        codec.is_server = true;

        let decoded = codec.internal_decipher(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }
}
