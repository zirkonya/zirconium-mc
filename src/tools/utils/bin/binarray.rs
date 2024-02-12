use super::*;

#[derive(Clone)]
pub struct Array<T>(Vec<T>);

impl <T: Binary + Clone> Binary for Array<T> {
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = vec![];
        for elem in &self.0 {
            bin.append(&mut elem.to_bin());
        }
        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let mut vec = Vec::new();
        let mut cursor = 0_usize;
        loop {
            if cursor >= bin.len() {
                return Ok(Array(vec));
            }
            let tmp = T::from_bin(bin[cursor..].to_vec())?;
            vec.push(tmp.clone());
            cursor += tmp.byte_length();
        }
    }

    fn byte_length(&self) -> usize {
        let mut size = 0_usize;
        for elem in &self.0 {
            size += elem.byte_length();
        }
        size
    }
}