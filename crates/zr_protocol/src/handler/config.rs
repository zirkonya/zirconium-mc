use zr_network::packet::packet::Packet;

use super::{
    handler::{Next, PacketHandler},
    protocol_handler::Client,
};

/// Minimal implementation
pub struct ConfigProtocol;
impl ConfigProtocol {}

impl PacketHandler for ConfigProtocol {
    fn handle_packet(&mut self, client: &mut Client, packet: Packet) -> Next {
        match packet.id() {
            _ => Next::Disconnect,
        }
    }
}
