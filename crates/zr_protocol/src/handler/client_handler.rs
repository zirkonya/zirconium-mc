use std::{collections::HashMap, sync::mpsc::Receiver};
use zr_network::{client::Client, packet::packet::Packet};

pub mod handshake;

pub enum State {
    Handshake,
    Status,
    Config,
    Login,
    Play,
}

pub struct ClientHandler {
    client_state: HashMap<u32, State>,
    clients: HashMap<u32, Client>,
    receiver: Receiver<(u32, Packet)>,
}

impl ClientHandler {
    pub fn new(receiver: Receiver<(u32, Packet)>) -> Self {
        Self {
            client_state: HashMap::new(),
            clients: HashMap::new(),
            receiver,
        }
    }

    pub fn add_client(&mut self, id: u32, client: Client) {
        self.clients.insert(id, client);
    }

    fn handle_packet(&mut self, client: u32, packet: Packet) {
        todo!()
    }

    pub fn handle(&mut self) {
        loop {
            let (id, packet) = self.receiver.recv().unwrap();
            self.handle_packet(id, packet);
        }
    }
}
