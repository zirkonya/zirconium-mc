pub mod error;
pub mod network;
pub mod utils;
// TODO : See DashMap ?
// TODO : tests folder & better tests case
// TODO : Bundle in packet implementation
#[cfg(test)]
mod tests {
    use crate::{
        network::{
            context::{ClientContext, State},
            packet::Packet,
        },
        utils::varint::VarInt,
    };
    use std::sync::Arc;
    use zr_protocol::{
        message::size::PrefixSize,
        serialization::{Serialize, SerializeWithContext},
    };

    #[test]
    pub fn write_packet() {
        let packet: Packet = Packet::new(0x03, VarInt::new(256).to_bytes().unwrap()).into();
        let context = ClientContext {
            state: State::Login,
            compression_set: false,
            compression_threshold: Some(256),
        };
        let set_compression: PrefixSize<VarInt<i32>, Arc<[u8]>> =
            PrefixSize::new(packet.to_bytes_with_context(&context).unwrap());

        let array: &[u8] = &set_compression.to_bytes().unwrap();
        assert_eq!(array, &[0x03, 0x03, 0x80, 0x02]);
    }

    #[test]
    pub fn write_uncompressed() {
        const RAW_LOGIN_SUCCESS: [u8; 29] = [
            0x1c, 0x0, 0x2, 0xec, 0xc9, 0xcc, 0xb9, 0xa4, 0x8c, 0x3a, 0x77, 0xa9, 0xff, 0x41, 0x13,
            0x0, 0xe5, 0x93, 0xa2, 0x8, 0x7a, 0x69, 0x72, 0x6b, 0x6f, 0x6e, 0x79, 0x61, 0x0,
        ];
        const PAYLOAD_LOGIN_SUCCESS: [u8; 26] = [
            0xec, 0xc9, 0xcc, 0xb9, 0xa4, 0x8c, 0x3a, 0x77, 0xa9, 0xff, 0x41, 0x13, 0x0, 0xe5,
            0x93, 0xa2, 0x8, 0x7a, 0x69, 0x72, 0x6b, 0x6f, 0x6e, 0x79, 0x61, 0x0,
        ];
        let compressed: Packet = Packet::new(0x02, PAYLOAD_LOGIN_SUCCESS.into()).into();

        let context = ClientContext {
            state: State::Login,
            compression_set: true,
            compression_threshold: Some(256),
        };

        let login_success: PrefixSize<VarInt<i32>, Arc<[u8]>> =
            PrefixSize::new(compressed.to_bytes_with_context(&context).unwrap());
        let array: &[u8] = &login_success.to_bytes().unwrap();
        assert_eq!(array, &RAW_LOGIN_SUCCESS);
    }
}
