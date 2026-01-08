use std::sync::Arc;

use zr_protocol::macros::Serialize;
use zr_protocol::message::option::PrefixOption;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct LoginPluginResponse {
    message_id: VarInt<i32>,
    data: PrefixOption<Arc<[u8]>>,
}
