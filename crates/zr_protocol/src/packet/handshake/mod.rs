use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_network_macros::Packet;

#[derive(Binary, Debug, Packet)]
#[id = 0x00]
pub struct Handshake {
    pub(crate) protocol_version: VarInt<i32>,
    pub(crate) server_address: String,
    pub(crate) server_port: u16,
    pub(crate) next_state: VarInt<i32>,
}
