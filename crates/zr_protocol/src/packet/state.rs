
#[derive(Debug, PartialEq, Eq)]
pub enum PacketState {
    Handshake,
    Status,
    Login,
    Play
}