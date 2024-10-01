use std::{collections::HashMap, hash::Hash, u128};

use uuid::Uuid;

use crate::{
    error::{BinaryError, Result},
    varint::VarInt,
};

pub trait Binary {
    fn binary_len(&self) -> usize;
    fn to_binary(self) -> Vec<u8>;
    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
}

impl Binary for bool {
    fn binary_len(&self) -> usize {
        1
    }

    fn to_binary(self) -> Vec<u8> {
        if self {
            vec![0x01]
        } else {
            vec![0x00]
        }
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.is_empty() {
            Err(BinaryError::LengthError)
        } else {
            Ok(bin[0] == 1)
        }
    }
}

impl Binary for i8 {
    fn binary_len(&self) -> usize {
        1
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.is_empty() {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0]]))
        }
    }
}

impl Binary for u8 {
    fn binary_len(&self) -> usize {
        1
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.is_empty() {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0]]))
        }
    }
}

impl Binary for i16 {
    fn binary_len(&self) -> usize {
        2
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 2 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0], bin[1]]))
        }
    }
}

impl Binary for u16 {
    fn binary_len(&self) -> usize {
        2
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 2 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0], bin[1]]))
        }
    }
}

impl Binary for i32 {
    fn binary_len(&self) -> usize {
        4
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 4 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0], bin[1], bin[2], bin[3]]))
        }
    }
}

impl Binary for u32 {
    fn binary_len(&self) -> usize {
        4
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 4 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0], bin[1], bin[2], bin[3]]))
        }
    }
}

impl Binary for i64 {
    fn binary_len(&self) -> usize {
        8
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 8 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([
                bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7],
            ]))
        }
    }
}

impl Binary for u64 {
    fn binary_len(&self) -> usize {
        8
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 8 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([
                bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7],
            ]))
        }
    }
}

impl Binary for i128 {
    fn binary_len(&self) -> usize {
        16
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 16 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([
                bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7], bin[8], bin[9],
                bin[10], bin[11], bin[12], bin[13], bin[14], bin[15],
            ]))
        }
    }
}

impl Binary for u128 {
    fn binary_len(&self) -> usize {
        16
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 16 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([
                bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7], bin[8], bin[9],
                bin[10], bin[11], bin[12], bin[13], bin[14], bin[15],
            ]))
        }
    }
}

impl Binary for f32 {
    fn binary_len(&self) -> usize {
        4
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 4 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([bin[0], bin[1], bin[2], bin[3]]))
        }
    }
}

impl Binary for f64 {
    fn binary_len(&self) -> usize {
        8
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() < 8 {
            Err(BinaryError::LengthError)
        } else {
            Ok(Self::from_be_bytes([
                bin[0], bin[1], bin[2], bin[3], bin[4], bin[5], bin[6], bin[7],
            ]))
        }
    }
}

impl<T: Binary> Binary for Option<T> {
    fn binary_len(&self) -> usize {
        match self {
            Some(t) => t.binary_len(),
            None => 0,
        }
    }

    fn to_binary(self) -> Vec<u8> {
        match self {
            Some(t) => t.to_binary(),
            None => Vec::new(),
        }
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        if bin.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(T::from_binary(bin)?))
        }
    }
}

impl Binary for String {
    fn binary_len(&self) -> usize {
        let length = VarInt::new(self.len() as i32);
        length.binary_len() + self.len()
    }

    fn to_binary(self) -> Vec<u8> {
        let len = VarInt::new(self.len() as i32);
        let mut binary = len.to_binary();
        binary.append(&mut self.as_bytes().to_vec());
        binary
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        let len = VarInt::<i32>::from_binary(bin[0..5].to_vec())?;
        let cursor = len.binary_len();
        let utf8 = String::from_utf8_lossy(&bin[cursor..(cursor + len.0 as usize)]).to_string();
        Ok(utf8)
    }
}

impl<T: Binary + Clone> Binary for Vec<T> {
    fn binary_len(&self) -> usize {
        self.iter().map(|elem| elem.binary_len()).sum()
    }

    fn to_binary(self) -> Vec<u8> {
        self.into_iter().flat_map(|elem| elem.to_binary()).collect()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut cursor = 0;
        let mut vec = Vec::new();
        while !bin.is_empty() {
            let current = T::from_binary(bin[cursor..].to_vec())?;
            cursor += current.binary_len();
            vec.push(current);
        }
        Ok(vec)
    }
}

impl Binary for Uuid {
    fn binary_len(&self) -> usize {
        16
    }

    fn to_binary(self) -> Vec<u8> {
        self.to_u128_le().to_binary()
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self::from_u128_le(u128::from_binary(bin)?))
    }
}

impl<K, V> Binary for HashMap<K, V>
where
    K: Binary + Eq + Hash,
    V: Binary,
{
    fn binary_len(&self) -> usize {
        let mut len = 0;
        for (k, v) in self {
            len += k.binary_len() + v.binary_len()
        }
        len
    }

    fn to_binary(self) -> Vec<u8> {
        let mut bin = Vec::with_capacity(self.binary_len());
        for (k, v) in self {
            bin.extend(k.to_binary());
            bin.extend(v.to_binary());
        }
        bin
    }

    fn from_binary(bin: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut cursor = 0;
        let mut map = HashMap::new();
        while cursor < bin.len() {
            let key = K::from_binary(bin[cursor..].to_vec())?;
            cursor += key.binary_len();
            let value = V::from_binary(bin[cursor..].to_vec())?;
            map.insert(key, value);
        }
        Ok(map)
    }
}
