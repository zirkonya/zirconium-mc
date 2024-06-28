use std::{sync::{Arc, Mutex}, thread};

use receiver::{ClientReveiverThread, Receiver};
use sender::{ClientSenderThread, Sender};
use tokio::net::TcpStream;
use zr_binary::binary::Binary;
use crate::packet::{Packet, PacketData};

use super::{options::HandlerOptions, Handler};

pub mod sender;
pub mod receiver;

pub struct ClientNetworkHandler<S: Sender<TcpStream>, R: Receiver<TcpStream>>
    where
        S: Sender<TcpStream> + std::marker::Send + 'static,
        R: Receiver<TcpStream> + std::marker::Send + 'static 
{
    sender: Arc<Mutex<S>>,
    receiver: Arc<Mutex<R>>,
    alive: Arc<Mutex<bool>>,
    options: HandlerOptions,
}

impl ClientNetworkHandler<ClientSenderThread, ClientReveiverThread> {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self::with_options(tcp_stream, HandlerOptions::default())
    }

    pub fn with_options(tcp_stream: TcpStream, options: HandlerOptions) -> Self {
        let stream = Arc::new(Mutex::new(tcp_stream));
        let alive = Arc::new(Mutex::new(true));

        let sender   = Arc::new(Mutex::new(ClientSenderThread::new(stream.clone(), alive.clone())));
        let receiver = Arc::new(Mutex::new(ClientReveiverThread::new(stream.clone(), alive.clone())));

        Self { sender: sender.clone(), receiver: receiver.clone(), options, alive: alive.clone() }
    }

    pub fn get_options_mut(&mut self) -> &mut HandlerOptions {
        &mut self.options
    }

    pub fn get_options(&mut self) -> &HandlerOptions {
        &self.options
    }

    pub fn read_packet(&mut self) -> Option<Packet> {
        match self.receiver.lock() {
            Ok(mut receiver) => {
                receiver.reveive_packet()
            }
            Err(_) => {
                panic!("PANIK")
            }
        }
    }

    // return the first packet found with `P::ID`
    pub fn read<P: PacketData + Binary>(&mut self) -> Option<P> {
        todo!("read first packet with same ID as P")
    }

    pub fn write_packet(&mut self, packet: Packet) {
        match self.sender.lock() {
            Ok(mut sender) => {
                sender.send_packet(packet);
            },
            Err(e) => {
                eprintln!("{e:?}");
                panic!("PANIK")
            },
        }
        
    }

    pub fn write<P: PacketData + Binary>(&mut self, packet: P) {
        self.write_packet(packet.to_packet());
    }
}

impl <S: Handler + Sender<TcpStream> + Send, R: Handler + Receiver<TcpStream> + Send> Handler for ClientNetworkHandler<S, R> {
    fn is_alive(&mut self) -> bool {
        *self.alive.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = false;
            self.alive.clear_poison();
            e.into_inner()
        })
    }

    async fn init(&mut self) -> super::error::Result<()> {
        Ok(())
    }

    async fn inner(&mut self) -> super::error::Result<()> {
        thread::yield_now();
        Ok(())
    }

    async fn stop(&mut self) -> super::error::Result<()> {
        *self.alive.lock().unwrap_or_else(|mut e| {
            **e.get_mut() = false;
            self.alive.clear_poison();
            e.into_inner()
        }) = false;
        Ok(())
    }
}