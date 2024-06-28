use std::future::Future;
use std::sync::Arc;
use std::{collections::VecDeque, sync::Mutex};

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use zr_binary::binary::Binary;
use crate::handler::error::{NetworkError, Result};
use crate::handler::Handler;
use crate::packet::Packet;

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

pub trait Receiver<T> {
    fn new(stream: Arc<Mutex<T>>, alive: Arc<Mutex<bool>>) -> Self;
    fn reveive_packet(&mut self) -> Option<Packet>;
    fn read_packet(&mut self) -> impl Future<Output = Result<Packet>>;
}

pub struct ClientReveiverThread {
    tcp_stream: Arc<Mutex<TcpStream>>,
    alive: Arc<Mutex<bool>>,
    pool: VecDeque<Packet>,
}

unsafe impl Send for ClientReveiverThread {}

impl Receiver<TcpStream> for ClientReveiverThread {
    fn new(stream: Arc<Mutex<TcpStream>>, alive: Arc<Mutex<bool>>) -> Self {
        Self { tcp_stream: stream, pool: VecDeque::new(), alive }
    }

    fn reveive_packet(&mut self) -> Option<Packet> {
        self.pool.pop_back()
    }

    async fn read_packet(&mut self) -> Result<Packet> {
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

impl Handler for ClientReveiverThread {
    fn is_alive(&mut self) -> bool {
        *self.alive.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = false;
            self.alive.clear_poison();
            e.into_inner()
        })
    }

    async fn init(&mut self) -> Result<()> {
        Ok(())
    }

    async fn inner(&mut self) -> Result<()> {
        let packet = self.read_packet().await?;
        self.pool.push_front(packet);
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        *self.alive.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = false;
            self.alive.clear_poison();
            e.into_inner()
        }) = false;
        Ok(())
    }
}