use std::sync::Arc;

use zr_protocol::macros::Serialize;
use zr_protocol::message::size::PrefixSize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct LoginPluginRequest {
    message_id: VarInt<i32>,
    channel: PrefixSize<VarInt<i32>, Arc<[u8]>>, // TODO : Identifier
    data: Arc<[u8]>,
}
