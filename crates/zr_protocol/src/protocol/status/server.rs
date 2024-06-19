use zr_binary_macros::Binary;
use zr_network_macros::Packet;

#[derive(Binary, Debug, Packet)]
#[id = 0x00]
pub struct StatusResponse {
    pub(crate) json_response: String,
}

#[derive(Binary, Debug, Packet)]
#[id = 0x01]
pub struct Pong {
    pub(crate) payload: i64,
}