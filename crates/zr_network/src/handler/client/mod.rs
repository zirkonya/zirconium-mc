use std::sync::{Arc, Mutex};

use receiver::Receiver;
use sender::Sender;
use tokio::net::TcpStream;
use zr_binary::binary::Binary;
use crate::packet::{self, Packet, PacketData};

use super::options::HandlerOptions;

pub mod sender;
pub mod receiver;

pub struct ClientNetworkHandler<S: Sender<TcpStream>, R: Receiver<TcpStream>> {
    sender: S,
    receiver: R,
    options: HandlerOptions,
}

impl <S: Sender<TcpStream>, R: Receiver<TcpStream>> ClientNetworkHandler<S, R> {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self::with_options(tcp_stream, HandlerOptions::default())
    }

    pub fn with_options(tcp_stream: TcpStream, options: HandlerOptions) -> Self {
        let stream = Arc::new(Mutex::new(tcp_stream));
        let sender = S::new(stream.clone());
        let receiver = R::new(stream.clone());
        Self { sender, receiver, options }
    }

    pub fn get_options_mut(&mut self) -> &mut HandlerOptions {
        &mut self.options
    }

    pub fn get_options(&mut self) -> &HandlerOptions {
        &self.options
    }

    pub fn read_packet(&mut self) -> Option<Packet> {
        self.receiver.read_packet()
    }

    // return the first packet found with `P::ID`
    pub fn read<P: PacketData + Binary>(&mut self) -> Option<P> {
        todo!()
    }

    pub fn write_packet(&mut self, packet: Packet) {
        self.sender.send_packet(packet)
    }
}