use zr_protocol::macros::Serialize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
pub struct UpdateEntityPosition {
    pub entity_id: VarInt<i32>,
    pub delta: (i16, i16, i16),
    pub on_ground: bool,
}
