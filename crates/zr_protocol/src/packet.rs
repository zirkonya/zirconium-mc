use crate::{packet::{direction::PacketDirection, state::PacketState}, parser::binary::ToBytes};

pub mod state;
pub mod direction;
mod packet;

pub use packet::Packet;

pub trait PacketData {
    const ID: i32;
    const STATE: PacketState;
    const DIRECTION: PacketDirection;

    fn to_packet(self) -> Packet<Self> where Self: ToBytes + Sized {
        Packet::new(Self::ID, self)
    }

    fn packet_name() -> &'static str;
}