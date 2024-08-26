use std::{
    collections::HashMap,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::JoinHandle,
};
use zr_network::{
    client::Client,
    packet::{packet::Packet, PacketData},
};

use crate::protocol::status;

use super::status::StatusProtocol;

pub enum State {
    Handshake,
    Status,
    Config,
    Login,
    Play,
}

pub struct ClientHandler {
    status_protocol: StatusProtocol,
    client_state: HashMap<u32, State>,
    clients: HashMap<u32, Client>,
    receiver: Receiver<(u32, Packet)>,
}

impl ClientHandler {
    pub fn new(receiver: Receiver<(u32, Packet)>) -> Self {
        Self {
            status_protocol: StatusProtocol,
            client_state: HashMap::new(),
            clients: HashMap::new(),
            receiver,
        }
    }

    pub fn add_client(&mut self, id: u32, client: Client) {
        println!("Add client : {id} {client:?}");
        self.clients.insert(id, client);
        self.client_state.insert(id, State::Handshake);
    }

    fn handle_packet(&mut self, client_id: u32, packet: Packet) {
        if let Some(client) = self.clients.get_mut(&client_id) {
            let state = &self.client_state[&client_id];
            let response = match state {
                State::Handshake => {
                    self.client_state
                        .entry(client_id)
                        .and_modify(|state| *state = State::Status);
                    None
                }
                State::Status => Some(match packet.id() {
                    status::client::Ping::ID => self
                        .status_protocol
                        .ping(status::client::Ping::from_packet(packet).unwrap())
                        .to_packet(),
                    status::client::StatusRequest::ID => self
                        .status_protocol
                        .status(status::client::StatusRequest::from_packet(packet).unwrap())
                        .to_packet(),
                    _ => unreachable!(),
                }),
                _ => todo!(),
            };
            if let Some(response) = response {
                client.write_packet(response).unwrap()
            }
        }
    }

    pub fn handle(handler: Arc<Mutex<Self>>) -> JoinHandle<()> {
        std::thread::spawn(move || loop {
            std::thread::yield_now();
            match handler.try_lock() {
                Ok(mut handler) => match handler.receiver.try_recv() {
                    Ok((id, packet)) => {
                        println!("receive from [{id:08x}]");
                        handler.handle_packet(id, packet);
                    }
                    Err(err) => match err {
                        std::sync::mpsc::TryRecvError::Empty => std::thread::yield_now(),
                        _ => break,
                    },
                },
                Err(err) => match err {
                    std::sync::TryLockError::WouldBlock => std::thread::yield_now(),
                    _ => break,
                },
            }
        })
    }
}
