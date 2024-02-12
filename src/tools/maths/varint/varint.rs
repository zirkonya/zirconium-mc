use crate::tools::utils::bin::{Binary, BinaryError};

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct VarInt<I> {
    value: I
}

impl <I: Copy> VarInt<I> {
    
    pub fn new(value: I) -> Self {
        Self { value }
    }

    pub fn value(&self) -> I {
        self.value
    }
}

impl Binary for VarInt<i32> {
    
    fn to_bin(&self) -> Vec<u8> {
        let mut value = self.value;
        let mut vec = Vec::new();
        let len = self.byte_length();
        for _ in 1..len {
            vec.push((value & SEGMENT_BITS as i32) as u8 | CONTINUE_BIT);
            value >>= 7;
        }
    
        vec.push(value as u8 & SEGMENT_BITS);
        vec
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        let mut value: i32 = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
    
        loop {
            let current_byte: u8 = bin[cursor];
            value |= ((current_byte & SEGMENT_BITS) as i32) << position;
    
            if current_byte & CONTINUE_BIT == 0 {
                break Ok(Self::new(value));
            }
    
            cursor += 1;
            position += 7;
    
            if position >= i32::BITS as usize {
                break Err(BinaryError::new("VarInt too big".to_string()));
            }
            if cursor >= bin.len() {
                break Err(BinaryError::new("Incomplete VarInt encoding".to_string()));
            }    
        }
    }
    
    fn byte_length(&self) -> usize {
        let value = self.value as u32;
        let mut exp = 7_u32;
        let mut size = 1;
        while exp <= 32 && value >= 2_u32.pow(exp) {
            size += 1;
            exp += 7;
        }
        size as usize
    }
}

impl Binary for VarInt<i64> {

    fn to_bin(&self) -> Vec<u8> {
        let mut value = self.value;
        let mut vec = Vec::new();
        let len = self.byte_length();
        for _ in 1..len {
            vec.push((value & SEGMENT_BITS as i64) as u8 | CONTINUE_BIT);
            value >>= 7;
        }
    
        vec.push(value as u8 & SEGMENT_BITS);
        vec
    }
    

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        let mut value: i64 = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
    
        loop {
            let current_byte: u8 = bin[cursor];
            value |= ((current_byte & SEGMENT_BITS) as i64) << position;
    
            if current_byte & CONTINUE_BIT == 0 {
                break Ok(Self::new(value));
            }
    
            cursor += 1;
            position += 7;
    
            if position >= i32::BITS as usize {
                break Err(BinaryError::new("VarInt too big".to_string()));
            }
            if cursor >= bin.len() {
                break Err(BinaryError::new("Incomplete VarInt encoding".to_string()));
            }    
        }
    }
    
    fn byte_length(&self) -> usize {
        let value = self.value as u32;
        let mut exp = 7_u32;
        let mut size = 1;
        while exp <= 32 && value >= 2_u32.pow(exp) {
            size += 1;
            exp += 7;
        }
        size as usize
    }
}

#[cfg(test)]
mod test {
    use crate::tools::utils::bin::Binary;
    use super::VarInt;

    #[test]
    fn test_varint() {
        let varint = VarInt::new(i32::MIN);
        let bin = varint.to_bin();
        assert_eq!(VarInt::from_bin(bin).expect("Error"), varint)
    }
}