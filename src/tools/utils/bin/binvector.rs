use crate::tools::maths::vector::{vector2::Vector2, vector3::Vector3};

use super::Binary;

impl <T: Binary> Binary for Vector2<T> {
    fn to_bin(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.append(&mut self.x().to_bin());
        vec.append(&mut self.y().to_bin());
        vec
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, super::BinaryError> where Self: Sized {
        let x = T::from_bin(bin.clone())?;
        let y = T::from_bin(bin[x.byte_length()..].to_vec())?;
        Ok(Self::new(x, y))
    }

    fn byte_length(&self) -> usize {
        self.x().byte_length() +
        self.y().byte_length()
    }
}

impl <T: Binary> Binary for Vector3<T> {
    fn to_bin(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.append(&mut self.x().to_bin());
        vec.append(&mut self.y().to_bin());
        vec.append(&mut self.z().to_bin());
        vec
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, super::BinaryError> where Self: Sized {
        let x = T::from_bin(bin.clone())?;
        let c = x.byte_length();
        let y = T::from_bin(bin[c..].to_vec())?;
        let z = T::from_bin(bin[c+y.byte_length()..].to_vec())?;
        Ok(Self::new(x, y, z))
    }

    fn byte_length(&self) -> usize {
        self.x().byte_length() +
        self.y().byte_length() +
        self.z().byte_length()
    }
}