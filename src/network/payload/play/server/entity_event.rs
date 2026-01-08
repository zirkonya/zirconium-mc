use zr_protocol::macros::Serialize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
pub struct EntityEvent {
    pub entity_id: VarInt<i32>,
    pub entity_status: u8, // TODO : Byte Enum (Entity Statuses)
}
