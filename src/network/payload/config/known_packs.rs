use std::sync::Arc;
use zr_protocol::{macros::Serialize, message::size::PrefixSize};
use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
pub struct KnownPack {
    pub namespace: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    pub id: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    pub version: PrefixSize<VarInt<i32>, Arc<[u8]>>,
}

#[derive(Serialize, Debug)]
pub struct KnownPacks {
    pub known_packs: PrefixSize<VarInt<i32>, Arc<[KnownPack]>>,
}
