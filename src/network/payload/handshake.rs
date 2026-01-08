use std::sync::Arc;

use zr_protocol::{macros::Serialize, message::size::PrefixSize};

use crate::utils::varint::VarInt;

#[derive(Clone, Debug, Serialize)]
pub struct Handshake {
    pub protocol_version: VarInt<i32>,
    pub server_addr: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    pub server_port: u16,
    pub intent: VarInt<i32>,
}
