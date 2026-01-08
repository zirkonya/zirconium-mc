use zr_protocol::macros::Serialize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
pub struct ConfirmTeleportation {
    pub teleportation_id: VarInt<i32>,
}
