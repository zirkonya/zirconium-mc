use packet::Packet;
use zr_binary::binary::Binary;

use crate::error::packet::PacketError;

pub mod compressed;
pub mod packet;

pub trait PacketData {
    const ID: i32;
    fn id() -> i32 {
        Self::ID
    }

    fn to_packet(self) -> Packet
    where
        Self: Sized + Binary,
    {
        Packet {
            id: Self::id().into(),
            data: self.to_binary(),
        }
    }

    fn from_packet(packet: Packet) -> Result<Self, PacketError>
    where
        Self: Sized + Binary,
    {
        let id = packet.id();
        if id != Self::ID {
            return Err(PacketError::WrongId {
                expected: Self::ID,
                found: id,
            });
        }
        Self::from_binary(packet.data).map_err(PacketError::DataError)
    }
}
