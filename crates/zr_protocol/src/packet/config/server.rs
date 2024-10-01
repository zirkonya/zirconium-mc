use std::collections::HashMap;

use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_nbt::Nbt;
use zr_network_macros::Packet;

use super::registry::Registry;

#[derive(Binary, Packet)]
#[id = 0x07]
pub struct RegistryData {
    pub(crate) registry_codec: Nbt<HashMap<String, Registry>>,
}

#[derive(Binary, Clone)]
pub struct Pack {
    pub(crate) namespace: String,
    pub(crate) id: String,
    pub(crate) version: String,
}

#[derive(Binary, Packet)]
#[id = 0x0E]
pub struct KnownPacks {
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) known_pack: Vec<Pack>,
}
