use zr_protocol_macros::Packet;

use crate::varint::VarInt;

#[derive(Packet)]
#[packet(id = 0x00, state = Handshake, direction = ServerBound)]
pub struct Handshake {
    protocol_version: VarInt<i32>,
    server_host: String,
    server_port: u16,
    intent: VarInt<i32>, // 1: Status, 2: Login, 3: Transfer
}