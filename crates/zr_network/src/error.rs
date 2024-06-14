pub type Result<T> = std::result::Result<T, PacketError>;

#[derive(Debug)]
pub enum PacketError {
    WrongId {
        expected: i32,
        found: i32
    },
    DataError(zr_binary::error::BinaryError),
    IOError(std::io::Error)
}