use crate::tools::utils::bin::{Binary,BinaryError};


pub mod protocol;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

impl Binary for State {
    fn to_bin(&self) -> Vec<u8> {
        match self {
            State::Handshaking => vec![0x00],
            State::Status => vec![0x01],
            State::Login => vec![0x02],
            State::Play => vec![0x03],
        }
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let value = u8::from_bin(bin)?;
        match value {
            0 => Ok(State::Handshaking),
            1 => Ok(State::Status),
            2 => Ok(State::Login),
            3 => Ok(State::Play),
            _ => Err(BinaryError::new(String::from("Wrong value for State")))
        }
    }

    fn byte_length(&self) -> usize {
        1
    }
}