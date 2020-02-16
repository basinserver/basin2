use super::packets::*;
use crate::network::*;
use crate::packet::PacketContainer;
use crate::result::*;
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
use std::io::Write;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_util::codec::{Decoder, Encoder, Framed};

pub struct Connection {
    incoming: Receiver<Packet>,
    outgoing: Sender<Packet>,
    address: SocketAddr,
    codec: McCodec,
}

struct McCodec {
    read_cipher: Option<Crypter>,
    decrypted: BytesMut,
    threshold: Option<u32>,
    write_cipher: Option<Crypter>,
    state: ConnectionProtocol,
    is_server: bool,
}

impl McCodec {
    fn finish_decode(&mut self, buf: &mut BytesMut) -> Result<Packet> {
        let packet_id = buf.get_mc_var_int()?;
        Ok(match (self.state, self.is_server) {
            (ConnectionProtocol::Handshake, true) => {
                Packet::PacketHandshake(PacketHandshake::PacketHandshakeServerbound(
                    PacketHandshakeServerbound::decode(packet_id, buf)?,
                ))
            }
            (ConnectionProtocol::Handshake, false) => {
                Packet::PacketHandshake(PacketHandshake::PacketHandshakeClientbound(
                    PacketHandshakeClientbound::decode(packet_id, buf)?,
                ))
            }
            (ConnectionProtocol::Status, true) => {
                Packet::PacketStatus(PacketStatus::PacketStatusServerbound(
                    PacketStatusServerbound::decode(packet_id, buf)?,
                ))
            }
            (ConnectionProtocol::Status, false) => {
                Packet::PacketStatus(PacketStatus::PacketStatusClientbound(
                    PacketStatusClientbound::decode(packet_id, buf)?,
                ))
            }
            (ConnectionProtocol::Login, true) => {
                Packet::PacketLogin(PacketLogin::PacketLoginServerbound(
                    PacketLoginServerbound::decode(packet_id, buf)?,
                ))
            }
            (ConnectionProtocol::Login, false) => {
                Packet::PacketLogin(PacketLogin::PacketLoginClientbound(
                    PacketLoginClientbound::decode(packet_id, buf)?,
                ))
            }
            (ConnectionProtocol::Game, true) => Packet::PacketGame(
                PacketGame::PacketGameServerbound(PacketGameServerbound::decode(packet_id, buf)?),
            ),
            (ConnectionProtocol::Game, false) => Packet::PacketGame(
                PacketGame::PacketGameClientbound(PacketGameClientbound::decode(packet_id, buf)?),
            ),
        })
    }

    fn init_encryption(&mut self, key: &[u8]) -> Result<()> {
        self.read_cipher = Some(Crypter::new(
            Cipher::aes_128_cfb8(),
            Mode::Decrypt,
            &key,
            Some(&key),
        )?);
        self.write_cipher = Some(Crypter::new(
            Cipher::aes_128_cfb8(),
            Mode::Encrypt,
            &key,
            Some(&key),
        )?);
        Ok(())
    }
}

impl Decoder for &mut McCodec {
    type Item = Packet;
    type Error = IoError;

    fn decode(&mut self, buf: &mut BytesMut) -> std::result::Result<Option<Packet>, IoError> {
        if buf.len() < 4 {
            return Ok(None);
        }
        let mut post_cipher = buf;
        match &mut self.read_cipher {
            Some(cipher) => {
                let mut ciphertext = vec![0; post_cipher.len() + 16];
                let count = cipher.update(post_cipher, &mut ciphertext).unwrap();
                self.decrypted.extend_from_slice(&ciphertext[..count]);
                post_cipher.advance(post_cipher.len());
                post_cipher = &mut self.decrypted;
            }
            _ => (),
        };
        let header_len = if post_cipher.len() > 16 {
            16
        } else {
            post_cipher.len()
        };
        let header_split = post_cipher.split_off(header_len);
        let mut header = post_cipher.clone();
        post_cipher.unsplit(header_split);
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
        if post_cipher.len() - header_size < packet_length {
            return Ok(None);
        }
        post_cipher.advance(header_size);
        let mut packet_data = post_cipher.split_to(packet_length);
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
            error!("Invalid packet: {:?}", packet.err().unwrap());
            return Err(IoError::from(ErrorKind::InvalidData));
        }
        let packet = packet.unwrap();
        Ok(Some(packet))
    }
}

impl Encoder for &mut McCodec {
    type Item = Packet;
    type Error = IoError;

    fn encode(
        &mut self,
        packet: Packet,
        output: &mut BytesMut,
    ) -> std::result::Result<(), IoError> {
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
        match &mut self.write_cipher {
            Some(cipher) => {
                output.reserve(encoded.len() + 16);
                let mut indirect_output: Vec<u8> = vec![0; encoded.len() + 16];
                let outlen = cipher.update(&encoded, &mut indirect_output).unwrap();
                output.extend_from_slice(&indirect_output[0..outlen]);
            }
            None => {
                output.unsplit(encoded);
            }
        };
        Ok(())
    }
}

async fn writeForward(
    mut incoming: Receiver<Packet>,
    mut to: SplitSink<Framed<TcpStream, &mut McCodec>, Packet>,
) -> Result<()> {
    while let Some(packet) = incoming.recv().await {
        if to.send(packet).await.is_err() {
            break;
        }
    }
    Ok(())
}

async fn readForward(
    mut outgoing: Sender<Packet>,
    mut from: SplitStream<Framed<TcpStream, &mut McCodec>>,
) -> Result<()> {
    while let Some(Ok(request)) = from.next().await {
        if outgoing.send(request).await.is_err() {
            break;
        }
    }
    Ok(())
}

impl Connection {
    pub async fn spawn(socket: TcpStream, is_server: bool) -> Result<()> {
        let address = socket.peer_addr().unwrap();

        let (in_outgoing, in_incoming) = channel::<Packet>(256);
        let (out_outgoing, out_incoming) = channel::<Packet>(256);
        let mut connection = Connection {
            incoming: in_incoming,
            outgoing: out_outgoing,
            address,
            codec: McCodec {
                read_cipher: None,
                decrypted: BytesMut::new(),
                threshold: None,
                write_cipher: None,
                state: ConnectionProtocol::Handshake,
                is_server,
            },
        };
        let framed_stream = Framed::new(socket, &mut connection.codec);
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
            decrypted: BytesMut::new(),
            write_cipher: None,
            threshold: None,
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        let test_packet = Packet::PacketHandshake(PacketHandshake::PacketHandshakeServerbound(
            PacketHandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        let mut codec_borrow = &mut codec;
        codec_borrow.encode(test_packet.clone(), &mut output)?;
        codec_borrow.is_server = true;

        let decoded = codec_borrow.decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_compressed() -> Result<()> {
        let mut codec = McCodec {
            read_cipher: None,
            decrypted: BytesMut::new(),
            write_cipher: None,
            threshold: Some(0),
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        let test_packet = Packet::PacketHandshake(PacketHandshake::PacketHandshakeServerbound(
            PacketHandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        let mut codec_borrow = &mut codec;
        codec_borrow.encode(test_packet.clone(), &mut output)?;
        codec_borrow.is_server = true;

        let decoded = codec_borrow.decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_compressible() -> Result<()> {
        let mut codec = McCodec {
            read_cipher: None,
            decrypted: BytesMut::new(),
            write_cipher: None,
            threshold: Some(256),
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        let test_packet = Packet::PacketHandshake(PacketHandshake::PacketHandshakeServerbound(
            PacketHandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        let mut codec_borrow = &mut codec;
        codec_borrow.encode(test_packet.clone(), &mut output)?;
        codec_borrow.is_server = true;

        let decoded = codec_borrow.decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_encrypted() -> Result<()> {
        let key = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut codec = McCodec {
            read_cipher: None,
            decrypted: BytesMut::new(),
            write_cipher: None,
            threshold: None,
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        codec.init_encryption(&key)?;
        let test_packet = Packet::PacketHandshake(PacketHandshake::PacketHandshakeServerbound(
            PacketHandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        let mut codec_borrow = &mut codec;
        codec_borrow.encode(test_packet.clone(), &mut output)?;
        codec_borrow.is_server = true;

        let decoded = codec_borrow.decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }

    #[test]
    fn test_packet_serialization_encrypted_compressed() -> Result<()> {
        let key = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut codec = McCodec {
            read_cipher: None,
            decrypted: BytesMut::new(),
            write_cipher: None,
            threshold: Some(0),
            state: ConnectionProtocol::Handshake,
            is_server: false,
        };
        codec.init_encryption(&key)?;
        let test_packet = Packet::PacketHandshake(PacketHandshake::PacketHandshakeServerbound(
            PacketHandshakeServerbound::ClientIntentionPacket(ClientIntentionPacket {
                protocolVersion: 1234,
                hostName: "testHost".to_string(),
                port: 54321,
                intention: ConnectionProtocol::Login,
            }),
        ));
        let mut output = BytesMut::new();
        let mut codec_borrow = &mut codec;
        codec_borrow.encode(test_packet.clone(), &mut output)?;
        codec_borrow.is_server = true;

        let decoded = codec_borrow.decode(&mut output)?;
        assert!(decoded.is_some());
        assert_eq!(decoded.unwrap(), test_packet);
        Ok(())
    }
}
