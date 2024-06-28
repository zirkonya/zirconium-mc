use std::collections::VecDeque;
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use zr_binary::binary::Binary;
use crate::handler::error::{NetworkError, Result};
use crate::handler::Handler;
use crate::packet::{Packet, PacketData};

pub trait Sender<T> {
    fn new(stream: Arc<Mutex<TcpStream>>, alive: Arc<Mutex<bool>>) -> Self where Self: Send;
    fn write_packet(&mut self, packet: Packet) -> impl Future<Output = Result<()>>;
    fn send_packet(&mut self, packet: Packet);
    fn send<P: PacketData + Binary>(&mut self, packet: P) {
        self.send_packet(packet.to_packet())
    }
}

pub struct ClientSenderThread {
    tcp_stream: Arc<Mutex<TcpStream>>,
    alive: Arc<Mutex<bool>>,
    pool: VecDeque<Packet>,
}

unsafe impl Send for ClientSenderThread {}

impl Sender<TcpStream> for ClientSenderThread {
    fn new(stream: Arc<Mutex<TcpStream>>, alive: Arc<Mutex<bool>>) -> Self {
        Self { tcp_stream: stream, pool: VecDeque::new(), alive }
    }

    fn send_packet(&mut self, packet: Packet) {
        self.pool.push_front(packet);
    }

    async fn write_packet(&mut self, packet: Packet) -> Result<()> {
        let mut stream = self.tcp_stream
            .lock()
            .map_err(|error| NetworkError::Custom(error.to_string()))?;
        stream.write_all(&packet.to_binary()).await.map_err(|error| NetworkError::IOError(error))
    }
}

impl Handler for ClientSenderThread {
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
        if let Some(packet) = self.pool.pop_back() {
            self.write_packet(packet).await?;
        }
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