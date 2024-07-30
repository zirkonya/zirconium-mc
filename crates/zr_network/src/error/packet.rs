use std::fmt::Display;

#[derive(Debug)]
pub enum PacketError {
    WrongId { expected: i32, found: i32 },
    DataError(zr_binary::error::BinaryError),
    IoError(std::io::Error),
}

unsafe impl Send for PacketError {}

impl std::error::Error for PacketError {}

impl Display for PacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
