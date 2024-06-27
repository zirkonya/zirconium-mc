use std::collections::VecDeque;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use zr_binary::binary::Binary;
use crate::handler::error::{NetworkError, Result};
use crate::packet::{Packet, PacketData};

macro_rules! varint_from_reader {
    ($reader: ident) => {
        {
            const SEGMENT_BITS: u8 = 0x7F;
            const CONTINUE_BIT: u8 = 0x80;
            let mut value = 0;
            let mut position = 0;
            loop {
                let read_byte = $reader.read_u8().await.map_err(|error| NetworkError::IOError(error))?;
                value |= ((read_byte & SEGMENT_BITS) as i32) << position;
                if read_byte & CONTINUE_BIT == 0 {
                    break Ok(zr_binary::varint::VarInt::new(value));
                }
                position += 7;
                if position >= 32 {
                    break Err(NetworkError::ParseError(zr_binary::error::BinaryError::LengthError))
                }
            }
        }
    };
}

pub trait Sender<T> {
    fn new(stream: Arc<Mutex<TcpStream>>) -> Self;
    fn send_packet(&mut self, packet: Packet);
    fn send<P: PacketData + Binary>(&mut self, packet: P) {
        self.send_packet(packet.to_packet())
    }
    fn start(&mut self) -> impl Future<Output = Result<()>>;
}

pub struct ClientSenderThread {
    tcp_stream: Arc<Mutex<TcpStream>>,
    /// Warning : no limit may be memory problem
    pool: VecDeque<Packet>,
}

impl ClientSenderThread {
    /// Read a packet from tcp stream
    pub async fn read_packet(&mut self) -> Result<Packet> {
        let mut stream = self.tcp_stream
            .lock()
            .map_err(|error| NetworkError::Custom(error.to_string()))?;

        // read packet length
        let length = varint_from_reader!(stream)?;        
        // read `length` next bytes to format the packet 
        let mut buf = vec![0u8; length.into()];
        stream.read_exact(&mut buf).await.map_err(|error| NetworkError::IOError(error))?;
        Packet::from_binary(buf)
            .map_err(|error| NetworkError::ParseError(error))
    }
}

impl Sender<TcpStream> for ClientSenderThread {
    fn new(stream: Arc<Mutex<TcpStream>>) -> Self {
        Self { tcp_stream: stream, pool: VecDeque::new() }
    }

    fn send_packet(&mut self, packet: Packet) {
        self.pool.push_front(packet);
    }

    async fn start(&mut self) -> Result<()> {
        loop {
            let packet = self.read_packet().await?;
            self.pool.push_front(packet);
        }
    }
}