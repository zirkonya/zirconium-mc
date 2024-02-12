use crate::tools::{maths::varint::varint::VarInt, utils::bin::{Binary, BinaryError}};

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Color {
    Pink = 0,
    Blue = 1,
    Red = 2,
    Green = 3,
    Yellow = 4,
    Purple = 5, 
    White = 6
}

impl Binary for Color {
    fn to_bin(&self) -> Vec<u8> {
        VarInt::new(*self as i32).to_bin()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
    let value = VarInt::from_bin(bin)?;
        match value.value() {
            0 => Ok(Color::Pink),
            1 => Ok(Color::Blue),
            2 => Ok(Color::Red),
            3 => Ok(Color::Green),
            4 => Ok(Color::Yellow),
            5 => Ok(Color::Purple),
            6 => Ok(Color::White),
            _ => Err(BinaryError::wrong_format())
        }
    }

    fn byte_length(&self) -> usize {
        VarInt::new(*self as i32).byte_length()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Pink
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Division {
    No = 0,
    Six = 1,
    Ten = 2,
    Twelves = 3,
    Twenty = 4
}

impl Binary for Division {
    fn to_bin(&self) -> Vec<u8> {
        VarInt::new(*self as i32).to_bin()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let value = VarInt::from_bin(bin)?;
        match value.value() {
            0 => Ok(Division::No),
            1 => Ok(Division::Six),
            2 => Ok(Division::Ten),
            3 => Ok(Division::Twelves),
            4 => Ok(Division::Twenty),
            _ => Err(BinaryError::wrong_format())
        }
    }

    fn byte_length(&self) -> usize {
        VarInt::new(*self as i32).byte_length()
    }
}

impl Default for Division {
    fn default() -> Self {
        Division::Six
    }
}