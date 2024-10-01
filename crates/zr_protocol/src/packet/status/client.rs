use zr_binary_macros::Binary;
use zr_network_macros::Packet;

#[derive(Binary, Debug, Packet)]
#[id = 0x00]
pub struct StatusRequest;

#[derive(Binary, Debug, Packet)]
#[id = 0x01]
pub struct Ping {
    pub(crate) payload: i64,
}