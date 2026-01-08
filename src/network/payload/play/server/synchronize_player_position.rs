use zr_protocol::macros::Serialize;

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug)]
pub struct SynchronizePlayerPosition {
    pub teleport_id: VarInt<i32>,
    pub position: (f64, f64, f64),
    pub velocity: (f64, f64, f64),
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u32,
}
