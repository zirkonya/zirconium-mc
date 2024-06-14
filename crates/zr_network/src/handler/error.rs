use std::error::Error;

pub type Result<T> = std::result::Result<T, NetworkError>;

#[derive(Debug)]
pub enum NetworkError {
    ProtocolError,
    TimedOut,
    ConnectionClose,
    PacketError(crate::error::PacketError),
    ParseError(zr_binary::error::BinaryError),
    IOError(std::io::Error),
    Other(Box<dyn Error>),
    Custom(String)
}

unsafe impl Send for NetworkError {}