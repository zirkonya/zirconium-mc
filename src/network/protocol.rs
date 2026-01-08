use crate::network::{
    context::{ServerContext, State},
    packet::Bundle,
    payload::Payload,
    protocol::{
        action::Action, config::ConfigProtocol, handshake::HandshakeProtocol, login::LoginProtocol,
        play::PlayProtocol, status::StatusProtocol,
    },
};
use std::{net::SocketAddr, sync::Arc};
use zr_protocol::{
    message::size::PrefixSize, serialization::SerializeWithContext as _, traits::Protocol,
};

pub mod action;
pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

pub trait InnerProtocol {
    fn handle_message(&self, payload: Payload) -> Arc<[Action]>;
}

#[derive(Default, Clone)]
pub struct MinecraftProtocol<C: Default + Clone> {
    context: C,
    hanshake_protocol: HandshakeProtocol,
    status_protocol: StatusProtocol,
    login_protocol: LoginProtocol,
    config_protocol: ConfigProtocol,
    play_protocol: PlayProtocol,
}

impl<C: Default + Clone> MinecraftProtocol<C> {
    pub fn update_context(&mut self, update: fn(&mut C)) {
        (update)(&mut self.context)
    }
}

impl<C: Default + Clone> Protocol for MinecraftProtocol<C> {
    type Message = Bundle;
}

impl MinecraftProtocol<ServerContext> {
    pub async fn handle_message(
        &mut self,
        payload: Payload,
        addr: SocketAddr,
        context: &zr_protocol::context::Context<Self>,
    ) {
        let state = self.context.clients[&addr].state;
        let actions = match state {
            State::Handshaking => self.hanshake_protocol.handle_message(payload),
            State::Login => self.login_protocol.handle_message(payload),
            State::Status => self.status_protocol.handle_message(payload),
            State::Config => self.config_protocol.handle_message(payload),
            State::Play => self.play_protocol.handle_message(payload),
        };
        for action in actions.iter() {
            self.run_action(addr, action, context).await
        }
    }

    pub async fn run_action(
        &mut self,
        client: SocketAddr,
        action: &Action,
        context: &zr_protocol::context::Context<Self>,
    ) {
        // TODO : Error
        let client_context = self.context.clients.get(&client).unwrap();
        match action {
            Action::SendBundle(packets) => {
                let mut packets_buffer = Vec::with_capacity(packets.len());
                for packet in packets.iter() {
                    packets_buffer.push(PrefixSize::new(
                        packet
                            .to_packet()
                            .unwrap()
                            .to_bytes_with_context(client_context)
                            .unwrap(),
                    ));
                }
                context
                    .send(Bundle {
                        packets: packets_buffer.into(),
                    })
                    .await
                    .unwrap()
            }
            Action::SendMessage(payload) => {
                let bytes = payload
                    .to_packet()
                    .unwrap()
                    .to_bytes_with_context(client_context)
                    .unwrap();
                // println!(
                //     "\x1b[90m{}\x1b[m",
                //     bytes
                //         .iter()
                //         .map(|b| format!("{b:02x}"))
                //         .collect::<Vec<_>>()
                //         .concat()
                // );
                context
                    .send(Bundle {
                        packets: [PrefixSize::new(bytes)].into(),
                    })
                    .await
                    .unwrap()
            }
            Action::BroadcastMessage(_) => {}
            Action::BroadcastExceptSelfMessage(_) => {}
            Action::SetCompression(threshold) => {
                self.context.clients.entry(client).and_modify(|context| {
                    context.compression_set = true;
                    context.compression_threshold = Some(*threshold);
                });
            }
            Action::UpdateState(state) => {
                self.context
                    .clients
                    .entry(client)
                    .and_modify(|context| context.state = *state);
            }
            Action::Log(log) => println!("# {log}"),
        }
    }

    pub async fn on_message(&mut self, ctx: zr_protocol::context::Context<Self>) {
        let Some(message) = ctx.message() else {
            return;
        };
        let Some(addr) = ctx.peer_addr() else {
            return;
        };
        let Some(context) = self.context.clients.get(addr) else {
            return;
        };

        for packet in message.clone().into_packets(context) {
            match packet {
                Ok(packet) => {
                    match Payload::from_packet(&packet) {
                        Ok(payload) => self.handle_message(payload, *addr, &ctx).await, // UPDATE : spawn task ? (on_message)
                        Err(err) => eprintln!("[warning] - {err:?}"),
                    }
                }
                Err(err) => eprintln!("[warning] - {err:?}"),
            }
        }
    }

    pub fn add_client(&mut self, ctx: zr_protocol::context::Context<Self>) {
        let Some(addr) = ctx.peer_addr() else {
            return;
        };
        self.context.clients.entry(*addr).or_default();
    }

    pub fn remove_client(&mut self, ctx: zr_protocol::context::Context<Self>) {
        let Some(addr) = ctx.peer_addr() else {
            return;
        };
        self.context.clients.remove(addr);
    }
}
