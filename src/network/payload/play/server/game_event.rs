use zr_protocol::macros::Serialize;

#[derive(Serialize, Debug)]
pub struct GameEvent {
    pub event: u8,
    pub value: f32,
}
