use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_network_macros::Packet;

use super::server::Pack;

#[derive(Binary, Packet)]
#[id = 0x03]
pub struct AcknowledgeFinishConfiguration;

#[derive(Binary, Packet)]
#[id = 0x07]
pub struct KnownPacks {
    #[prefixed_length = "VarInt<i32>"]
    known_packs: Vec<Pack>,
}
