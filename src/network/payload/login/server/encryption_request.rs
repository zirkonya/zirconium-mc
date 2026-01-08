use std::sync::Arc;

use zr_protocol::macros::Serialize;
use zr_protocol::message::size::PrefixSize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct EncryptionRequest {
    server_id: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    public_key: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    verify_token: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    should_authenticate: bool,
}
