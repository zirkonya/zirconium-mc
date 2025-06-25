use zr_protocol_macros::Packet;

use crate::{
    parser::binary::{PrefixedLen, ToBytes},
    varint::VarInt,
};

#[derive(Packet)]
#[packet(id = 0x00, state = Handshake, direction = ServerBound)]
pub struct Handshake {
    pub protocol_version: VarInt<i32>,
    pub server_address: PrefixedLen<VarInt<i32>, String>,
    pub server_port: u16,
    pub intent: VarInt<i32>, // 1: Status, 2: Login, 3: Transfer
}

impl ToBytes for Handshake {
    fn bytes_len(&self) -> usize {
        self.protocol_version.bytes_len()
            + self.server_address.bytes_len()
            + self.server_port.bytes_len()
            + self.intent.bytes_len()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        let len = self.bytes_len();
        let mut buffer = Vec::with_capacity(len);
        let (_, mut bytes): (_, Vec<u8>) = self.protocol_version.to_bytes()?;
        buffer.append(&mut bytes);
        let (_, mut bytes): (_, Vec<u8>) = self.server_address.to_bytes()?;
        buffer.append(&mut bytes);
        let (_, mut bytes): (_, Vec<u8>) = self.server_port.to_bytes()?;
        buffer.append(&mut bytes);
        let (_, mut bytes): (_, Vec<u8>) = self.intent.to_bytes()?;
        buffer.append(&mut bytes);
        Ok((len, B::from(buffer)))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let bytes = bytes.into();
        let mut cursor = 0;
        let (len, protocol_version) = VarInt::<i32>::from_bytes(&bytes[cursor..])?;
        cursor += len;
        let (len, server_address) =
            PrefixedLen::<VarInt<i32>, String>::from_bytes(&bytes[cursor..])?;
        cursor += len;
        let (len, server_port) = u16::from_bytes(&bytes[cursor..])?;
        cursor += len;
        let (len, intent) = VarInt::<i32>::from_bytes(&bytes[cursor..])?;
        cursor += len;
        Ok((
            cursor,
            Self {
                protocol_version,
                server_address,
                server_port,
                intent,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        packet::{direction::PacketDirection, state::PacketState, PacketData},
        parser::binary::{PrefixedLen, ToBytes},
        protocol::handshake::Handshake,
        varint::VarInt,
    };

    #[test]
    fn test_packet_macro() {
        assert_eq!(Handshake::ID, 0x00);
        assert_eq!(Handshake::STATE, PacketState::Handshake);
        assert_eq!(Handshake::DIRECTION, PacketDirection::ServerBound)
    }

    #[test]
    fn test_packet_parsing() {
        let handshake = Handshake {
            protocol_version: VarInt::new(770),
            server_address: PrefixedLen::new("localhost"),
            server_port: 25565,
            intent: VarInt::new(1),
        };
        let len = handshake.bytes_len();
        let packet = handshake.to_packet();
        let (size, bytes): (_, Vec<u8>) = packet.to_bytes().unwrap();
        assert_eq!(size, 1 + len);
        assert_eq!(size, packet.bytes_len());
        assert_eq!(
            bytes,
            vec![
                0x00, 0x82, 0x06, 0x09, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x68, 0x6f, 0x73, 0x74, 0x63,
                0xdd, 0x01
            ]
        )
    }
}
