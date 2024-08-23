use std::{error::Error, fmt::Display};

use super::{packet::PacketError, worker::WorkerError};

#[derive(Debug)]
pub enum NetworkError {
    ProtocolError,
    TimedOut,
    ThreadError(WorkerError),
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
