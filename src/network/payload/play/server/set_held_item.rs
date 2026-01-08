use zr_protocol::macros::Serialize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
pub struct SetHeldItem {
    slot: VarInt<i32>,
}
