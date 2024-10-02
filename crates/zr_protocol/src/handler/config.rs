use zr_network::packet::{packet::Packet, PacketData};

use crate::packet::config::client;

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
            client::AcknowledgeFinishConfiguration::ID => {
                println!("OK");
                Next::Wait
            }
            _ => Next::Disconnect,
        }
    }
}
