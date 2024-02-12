pub mod binarray;
pub mod bintuple;
pub mod binvector;
pub mod macros;

use std::{io, fmt::Display};

use crate::tools::maths::varint::{to_vari32, from_vari32, vari32_byte_length};

#[derive(Debug)]
pub struct BinaryError{
    message: String
}

impl Display for BinaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<BinaryError> for io::Error {
    fn from(err: BinaryError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

impl BinaryError {
    #[inline(always)]
    pub fn new(message: String) -> Self {
        BinaryError { message }
    }

    #[inline(always)]
    pub fn wrong_size(waited: usize, obtained: usize) -> Self {
        Self::new(format!("Wrong size waited: {}; obtained: {}", waited, obtained))
    }

    #[inline(always)]
    pub fn unvailable() -> Self {
        Self::new(format!("Unvailable for {:?}", stringify!(Self)))
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Self::new("Null packet.".to_string())
    }

    #[inline(always)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    #[inline(always)]
    pub fn weird(size: i32) -> Self {
        Self::new(format!("Weird length {}", size))
    }

    #[inline(always)]
    pub fn too_short() -> Self {
        Self::new("Packet too short".to_string())
    }

    #[inline(always)]
    pub fn wrong_format() -> Self {
        Self::new("Wrong format".to_string())
    }
}

pub trait Binary {
    fn to_bin(&self) -> Vec<u8>;
    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized;
    fn byte_length(&self) -> usize;
}

impl Binary for bool {
    fn to_bin(&self) -> Vec<u8> {
        if *self {
            vec![0x01]
        } else {
            vec![0x00]
        }
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        Ok(bin[0] != 0x00)
    }


    #[inline(always)]
    fn byte_length(&self) -> usize {
        1
    }
}

impl Binary for u8 {
    fn to_bin(&self) -> Vec<u8> {
        vec![*self]
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 1 {
            Err(BinaryError::wrong_size(2usize, bin.len()))
        } else {
            Ok(bin[0])
        }
    }


    #[inline(always)]
    fn byte_length(&self) -> usize {
        1
    }
}

impl Binary for i8 {
    fn to_bin(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 1 {
            Err(BinaryError::wrong_size(1, bin.len()))
        } else {
            Ok(bin[0] as i8)
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        1
    }
}

impl Binary for u16 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 2 {
            Err(BinaryError::wrong_size(2, bin.len()))
        } else {
            Ok(u16::from_le_bytes([bin[0], bin[1]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        2
    }
}

impl Binary for i16 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 2 {
            Err(BinaryError::wrong_size(2, bin.len()))
        } else {
            Ok(i16::from_le_bytes([bin[0], bin[1]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        2
    }
}

impl Binary for u32 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 4 {
            Err(BinaryError::wrong_size(4, bin.len()))
        } else {
            Ok(u32::from_le_bytes([bin[0], bin[1], bin[2], bin[3]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        4
    }
}

impl Binary for i32 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 4 {
            Err(BinaryError::wrong_size(4, bin.len()))
        } else {
            Ok(i32::from_le_bytes([bin[0], bin[1], bin[2], bin[3]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        4
    }
}

impl Binary for u64 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 8 {
            Err(BinaryError::wrong_size(8, bin.len()))
        } else {
            Ok(u64::from_le_bytes([bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        8
    }
}

impl Binary for i64 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 8 {
            Err(BinaryError::wrong_size(8, bin.len()))
        } else {
            Ok(i64::from_le_bytes([bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        8
    }
}

impl Binary for u128 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 16 {
            Err(BinaryError::wrong_size(16, bin.len()))
        } else {
            Ok(u128::from_le_bytes([bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7], bin[8], bin[9], bin[10], bin[11], bin[12], bin[13], bin[14], bin[15]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        16
    }
}

impl Binary for i128 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 16 {
            Err(BinaryError::wrong_size(16, bin.len()))
        } else {
            Ok(i128::from_le_bytes([bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7], bin[8], bin[9], bin[10], bin[11], bin[12], bin[13], bin[14], bin[15]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        16
    }
}

impl Binary for f32 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 4 {
            Err(BinaryError::wrong_size(4, bin.len()))
        } else {
            Ok(f32::from_le_bytes([bin[0], bin[1], bin[2], bin[3]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        4
    }
}

impl Binary for f64 {
    fn to_bin(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() < 8 {
            Err(BinaryError::wrong_size(8, bin.len()))
        } else {
            Ok(f64::from_le_bytes([bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7]]))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        8
    }
}

impl Binary for String {
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = to_vari32(self.len() as i32);
        bin.append(&mut self.as_bytes().to_vec());
        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        // Error if : bin len < 0; bin len - len size < len
        if bin.len() == 0 {
            return Err(BinaryError::empty())
        }
        let len = from_vari32(&bin[0..5]);
        let cursor = vari32_byte_length(len);
        if len < 0 || bin.len() - cursor < len as usize {
            return Err(BinaryError::wrong_size(len as usize + cursor, bin.len()));
        }
        Ok(String::from_utf8_lossy(&bin[cursor..(cursor+len as usize)]).to_string())
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        vari32_byte_length(self.len() as i32) + self.len()
    }
}

impl <T: Binary + Clone> Binary for Vec<T> {
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = (self.len() as i32).to_bin();
        for elem in self {
            bin.append(&mut elem.to_bin());
        }
        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let mut vec = Self::new();
        let len = i32::from_bin(bin[0..4].to_vec())?;
        let mut offset = 4;
        for _ in 0..len {
            let elem = T::from_bin(bin[(offset as usize)..bin.len()].to_vec())?;
            vec.push(elem.clone());
            offset += elem.byte_length();
        }
        Ok(vec)
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        let mut length = 4;
        for elem in self {
            length += elem.byte_length();
        }
        length
    }
}

// TODO : add Option<T> binary implementation + resolve conflict
impl <T: Binary + Clone> Binary for Option<T> {
    fn to_bin(&self) -> Vec<u8> {
        match self {
            None => vec![],
            Some(some) => some.to_bin(),
        }
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(T::from_bin(bin)?))
        }
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        match self {
            None => 0,
            Some(some) => some.byte_length(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Binary;


    #[test]
    fn test_binary_bool() {
        let true_var = true;
        let bin = true_var.to_bin();
        assert_eq!(true_var, bool::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_byte() {
        let var = 0x7F;
        let bin = var.to_bin();
        assert_eq!(var, u8::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_short() {
        let var = 0x7FFF;
        let bin = var.to_bin();
        assert_eq!(var, u16::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_int() {
        let var = 0x7FFFFFFF;
        let bin = var.to_bin();
        assert_eq!(var, u32::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_long() {
        let var = 0x7FFFFFFFFFFFFFFF;
        let bin = var.to_bin();
        assert_eq!(var, u64::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_float() {
        let var = 123456.123456789;
        let bin = var.to_bin();
        assert_eq!(var, f32::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_double() {
        let var = 123456.123456789;
        let bin = var.to_bin();
        assert_eq!(var, f64::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_string() {
        let var = "Hello, World!".to_string();
        let bin = var.to_bin();
        assert_eq!(var, String::from_bin(bin).expect("Error while parsing"));
    }

    #[test]
    fn test_binary_vector() {
        let var = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let bin = var.to_bin();
        assert_eq!(var, Vec::from_bin(bin).expect("Error while parsing"));
    }
}
