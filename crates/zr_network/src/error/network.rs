use std::{error::Error, fmt::Display};

use super::{handler::HandlerError, packet::PacketError};

#[derive(Debug)]
pub enum NetworkError {
    ProtocolError,
    TimedOut,
    ThreadError(HandlerError),
    PacketError(PacketError),
    IOError(std::io::Error),
    Other(Box<dyn Error>),
}

unsafe impl Sync for NetworkError {}

impl Error for NetworkError {}

impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
