use std::future::Future;
use std::sync::Arc;
use std::{collections::VecDeque, sync::Mutex};

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use zr_binary::binary::Binary;
use crate::handler::error::{NetworkError, Result};
use crate::packet::Packet;

pub trait Receiver<T> {
    fn new(stream: Arc<Mutex<T>>) -> Self;
    fn read_packet(&mut self) -> Option<Packet>;
    fn start(&mut self) -> impl Future<Output = Result<()>>;
}

pub struct ClientReveiverThread {
    tcp_stream: Arc<Mutex<TcpStream>>,
    pool: VecDeque<Packet>,
}

impl ClientReveiverThread {
    pub async fn write_packet(&mut self, packet: Packet) -> Result<()> {
        let mut stream = self.tcp_stream
            .lock()
            .map_err(|error| NetworkError::Custom(error.to_string()))?;
        stream.write_all(&packet.to_binary()).await.map_err(|error| NetworkError::IOError(error))
    }
}

impl Receiver<TcpStream> for ClientReveiverThread {
    fn new(stream: Arc<Mutex<TcpStream>>) -> Self {
        Self { tcp_stream: stream, pool: VecDeque::new() }
    }

    fn read_packet(&mut self) -> Option<Packet> {
        self.pool.pop_back()
    }

    async fn start(&mut self) -> Result<()> {
        loop {
            if let Some(packet) = self.pool.pop_back() {
                self.write_packet(packet).await?;
            }
        }
    }
}