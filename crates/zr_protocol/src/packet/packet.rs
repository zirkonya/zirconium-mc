use crate::{packet::PacketData, parser::binary::ToBytes, varint::VarInt};

pub struct Packet<D> where D: PacketData + ToBytes  {
    id: VarInt<i32>,
    payload: D,
}