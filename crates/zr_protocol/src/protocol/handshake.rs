use zr_protocol_macros::{Packet, ToBytes};

use crate::{
    parser::binary::{PrefixedLen},
    varint::VarInt,
};

#[derive(Packet, ToBytes)]
#[packet(id = 0x00, state = Handshake, direction = ServerBound)]
pub struct Handshake {
    pub protocol_version: VarInt<i32>,
    pub server_address: PrefixedLen<VarInt<i32>, String>,
    pub server_port: u16,
    pub intent: VarInt<i32>, // 1: Status, 2: Login, 3: Transfer
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
