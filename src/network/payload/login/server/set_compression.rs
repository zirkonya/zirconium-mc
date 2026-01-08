use zr_protocol::macros::Serialize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct SetCompression {
    pub threshold: VarInt<i32>,
}
