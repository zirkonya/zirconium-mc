use std::sync::Arc;

use zr_protocol::macros::Serialize;
use zr_protocol::message::size::PrefixSize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct CookieRequest {
    key: PrefixSize<VarInt<i32>, Arc<[u8]>>, // TODO : Identifier
}
