use super::protocol_handler::Client;
use zr_network::packet::packet::Packet;

pub enum Next {
    SendPacket(Packet),
    SendPackets(Vec<Packet>),
    UpdateClient(fn(&mut Client)),
    Disconnect,
    Wait,
}

pub trait PacketHandler {
    fn handle_packet(&mut self, client: &mut Client, packet: Packet) -> Next;
}
