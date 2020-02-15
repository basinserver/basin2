use super::packets::Packet;
use crate::network::*;
use crate::result::*;
use bytes::buf::Buf;
use bytes::BytesMut;
use flate2::{Compress, Compression, Decompress, FlushCompress, FlushDecompress};
use futures::future::FutureExt;
use futures::pin_mut;
use futures::select;
use futures::stream::StreamExt;
use futures_util::sink::SinkExt;
use futures_util::stream::{SplitSink, SplitStream};
use log::*;
use openssl::symm::{Cipher, Crypter};
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
        let packet_length = post_cipher.read_var_int();
        if packet_length.is_none() {
            return Ok(None);
        }
        let mut count = 0;
        let (packet_length, packet_length_count) = packet_length.unwrap();
        if packet_length < 0 {
            return Err(IoError::from(ErrorKind::InvalidData));
        }
        let packet_length = packet_length as usize;
        count += packet_length_count;
        if post_cipher.len() - count < packet_length {
            return Ok(None);
        }
        post_cipher.advance(packet_length_count);
        let mut packet_data = post_cipher.split_to(packet_length);
        if self.threshold.is_some() {
            let decompressed_length = packet_data.read_var_int();
            if decompressed_length.is_none() {
                return Err(IoError::from(ErrorKind::InvalidData));
            }
            let (decompressed_length, _decompressed_length_length) = decompressed_length.unwrap();
            if decompressed_length < 0 || decompressed_length as usize > 2097152 {
                return Err(IoError::from(ErrorKind::InvalidData));
            }
            let decompressed_length = decompressed_length as usize;
            packet_data.advance(decompressed_length);
            if decompressed_length > 0 {
                let mut decompressor = Decompress::new(false);
                let compressed_packet = packet_data;
                packet_data = BytesMut::with_capacity(decompressed_length);
                decompressor.decompress(
                    &compressed_packet,
                    &mut packet_data,
                    FlushDecompress::Finish,
                )?;
            }
        }
        Ok(None) // TODO: remove
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
        // compress
        match self.threshold {
            Some(threshold) => {
                let raw_length = get_var_int_len(encoded.len() as i32) + encoded.len();
                if threshold > 0 && raw_length > threshold as usize + 1 && raw_length <= 2097152 {
                    let mut compressor = Compress::new(Compression::default(), false);
                    let mut decompressed = encoded;
                    let mut compressed = BytesMut::with_capacity(raw_length);
                    compressor.compress(
                        &mut decompressed,
                        &mut compressed,
                        FlushCompress::Finish,
                    )?;
                    encoded = BytesMut::with_capacity(compressed.len() + 10);
                    encoded.write_var_int(
                        (get_var_int_len(decompressed.len() as i32) + compressed.len()) as i32,
                    );
                    encoded.write_var_int(decompressed.len() as i32);
                    encoded.unsplit(compressed);
                } else {
                    let decompressed = encoded;
                    encoded = BytesMut::with_capacity(raw_length + 10);
                    encoded.write_var_int((get_var_int_len(0) + decompressed.len()) as i32);
                    encoded.write_var_int(0);
                    encoded.unsplit(decompressed);
                }
            }
            None => {
                let decompressed = encoded;
                encoded = BytesMut::with_capacity(decompressed.len() + 10);
                encoded.write_var_int(encoded.len() as i32);
                encoded.unsplit(decompressed);
            }
        };
        // encrypt
        match &mut self.write_cipher {
            Some(cipher) => {
                output.reserve(encoded.len() + 10);
                cipher.update(&encoded, output).unwrap();
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
    pub async fn spawn(socket: TcpStream) -> Result<()> {
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
