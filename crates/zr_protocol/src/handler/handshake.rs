use crate::packet::handshake::Handshake;

use super::handler::Next;
use super::handler::PacketHandler;
use super::protocol_handler::Client;
use super::protocol_handler::State;
use zr_network::packet::packet::Packet;
use zr_network::packet::PacketData;

pub struct HandshakeProtocol;

impl PacketHandler for HandshakeProtocol {
    fn handle_packet(&mut self, _: &mut Client, packet: Packet) -> Next {
        if packet.id() != Handshake::ID {
            return Next::Disconnect;
        }
        if let Ok(Handshake {
            protocol_version: _,
            server_address: _,
            server_port: _,
            next_state,
        }) = Handshake::from_packet(packet)
        {
            // TODO : check protocol version
            match next_state.0 {
                1 => Next::UpdateClient(|client| client.change_state(State::Status)),
                2 => Next::UpdateClient(|client| client.change_state(State::Login)),
                3 => Next::Disconnect,
                _ => Next::Disconnect,
            }
        } else {
            Next::Disconnect
        }
    }
}
