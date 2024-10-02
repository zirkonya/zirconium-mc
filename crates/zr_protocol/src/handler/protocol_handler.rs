// Client <=> ID <=> UUID <=> Player <=> Pseudo
use std::{
    collections::HashMap,
    sync::{mpsc::Receiver, Arc, RwLock},
};
use zr_core::{entity::player::Player, handler::Handler};
use zr_network::{error::network::NetworkError, packet::packet::Packet};

use super::{
    config::ConfigProtocol,
    handler::{Next, PacketHandler},
    handshake::HandshakeProtocol,
    login::LoginProtocol,
    status::{Description, JsonResponse, Players, StatusProtocol, Version},
};

#[derive(Debug)]
pub enum State {
    Handshake,
    Status,
    Play,
    Login,
    Config,
}

// TODO : found better name
pub struct Client {
    client: zr_network::client::client::Client,
    pub(crate) player: Option<Player>,
    state: State,
}

impl Client {
    fn new(client: zr_network::client::client::Client) -> Self {
        Client {
            client,
            player: None,
            state: State::Handshake,
        }
    }

    pub fn change_state(&mut self, state: State) {
        println!("[!] {state:?}");
        self.state = state;
    }

    pub fn player(&self) -> &Option<Player> {
        &self.player
    }

    pub fn player_name(&self) -> Option<String> {
        self.player.as_ref().map(|player| player.name())
    }
}

pub struct Protocols {
    handshake: HandshakeProtocol,
    status: StatusProtocol,
    login: LoginProtocol,
    config: ConfigProtocol,
    _play: (),
}

impl Default for Protocols {
    fn default() -> Self {
        let json_response = JsonResponse {
            version: Version {
                name: "1.21".to_string(),
                protocol: 767,
            },
            players: Players {
                max: 666,
                online: 0,
                sample: Vec::with_capacity(0),
            },
            description: Description {
                text: "§fMon serveur en §6§lRust".to_string(),
            },
            favicon: None,
            enforces_secure_chat: true,
        };
        Self {
            handshake: HandshakeProtocol,
            status: StatusProtocol::new(Arc::new(RwLock::new(json_response))),
            login: LoginProtocol::new().unwrap(),
            config: ConfigProtocol,
            _play: (),
        }
    }
}

pub struct ProtocolHandler {
    // packet receiver
    receiver: Receiver<(u32, Packet)>,
    clients: HashMap<u32, Client>,
    protocols: Protocols,
}

impl ProtocolHandler {
    pub fn new(receiver: Receiver<(u32, Packet)>) -> Self {
        Self {
            protocols: Protocols::default(),
            receiver,
            clients: HashMap::new(),
        }
    }

    pub fn add_client(&mut self, id: u32, client: zr_network::client::client::Client) {
        self.clients.insert(id, Client::new(client));
    }

    fn handle_packet(&mut self, client_id: u32, packet: Packet) -> Result<(), NetworkError> {
        let client = self.clients.get_mut(&client_id).unwrap();
        let next = match client.state {
            State::Handshake => self.protocols.handshake.handle_packet(client, packet),
            State::Status => self.protocols.status.handle_packet(client, packet),
            State::Config => self.protocols.config.handle_packet(client, packet),
            State::Login => self.protocols.login.handle_packet(client, packet),
            State::Play => {
                todo!()
            }
        };
        match next {
            Next::Disconnect => {
                self.clients
                    .get_mut(&client_id)
                    .unwrap()
                    .client
                    .shutdown()
                    .map_err(|err| NetworkError::IOError(err))?;
                // TODO : remove client
            }
            Next::Wait => {}
            Next::SendPacket(packet) => self
                .clients
                .get_mut(&client_id)
                .unwrap()
                .client
                .write_packet(packet)?,
            Next::SendPackets(packets) => {
                for packet in packets {
                    self.clients
                        .get_mut(&client_id)
                        .unwrap()
                        .client
                        .write_packet(packet)?;
                }
            }
            Next::UpdateClient(update) => update(self.clients.get_mut(&client_id).unwrap()),
        }
        Ok(())
    }
}

impl Handler for ProtocolHandler {
    type Return = ();

    fn handle(
        handler: std::sync::Arc<std::sync::Mutex<Self>>,
    ) -> std::thread::JoinHandle<Self::Return>
    where
        Self: Sized,
    {
        std::thread::spawn(move || loop {
            match handler.try_lock() {
                Ok(mut handler) => match handler.receiver.try_recv() {
                    Ok((client_id, packet)) => {
                        handler
                            .handle_packet(client_id, packet)
                            .expect("flemme de gerer l'erreur"); // TODO : ne plus avoir la flemme
                    }
                    Err(_) => continue,
                },
                Err(err) => match err {
                    std::sync::TryLockError::Poisoned(_) => break,
                    std::sync::TryLockError::WouldBlock => continue,
                },
            }
        })
    }
}
