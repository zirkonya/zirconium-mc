use std::sync::Arc;

use zr_protocol::macros::Serialize;
use zr_protocol::message::size::PrefixSize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct EncryptionResponse {
    shared_secret: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    verify_token: PrefixSize<VarInt<i32>, Arc<[u8]>>,
}
