use crate::{packet::{direction::PacketDirection, state::PacketState}, parser::binary::ToBytes, varint::VarInt};

pub mod state;
pub mod direction;
mod packet;

pub use packet::Packet;

pub trait PacketData {
    const ID: i32;
    const STATE: PacketState;
    const DIRECTION: PacketDirection;

    fn serialize(self) -> Packet<Self> where Self: Sized + ToBytes {
        todo!()
    }

    fn deserialize(bytes: &[u8]) -> Self {
        todo!()
    }

    fn packet_name() -> &'static str;
}