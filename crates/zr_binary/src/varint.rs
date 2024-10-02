use std::{
    fmt::{Debug, Write},
    i32,
    io::Read,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use serde::{de::Visitor, Deserialize, Serialize};

use crate::{binary::Binary, error::BinaryError};

pub const SEGMENT_BITS: u8 = 0x7F;
pub const CONTINUE_BIT: u8 = 0x80;

#[macro_export]
macro_rules! read_byte {
    ($reader: expr) => {{
        let mut byte: u8 = 0;
        match $reader.read_exact(std::slice::from_mut(&mut byte)) {
            Ok(_) => Ok(byte),
            Err(e) => Err(e),
        }
    }};
}

pub fn from_reader<R: Read>(reader: &mut R) -> std::io::Result<Result<VarInt<i32>, BinaryError>> {
    let segment_bits = SEGMENT_BITS;
    let continue_bit = CONTINUE_BIT;
    let mut value: u32 = 0;
    let mut position: usize = 0;
    loop {
        if position >= u32::BITS as usize {
            break Ok(Err(BinaryError::FormatError));
        }
        let current_byte: u8 = read_byte!(reader)?;
        value |= ((current_byte & segment_bits) as u32) << position;
        if current_byte & continue_bit == 0 {
            break Ok(Ok(VarInt::<i32>::new(value as i32)));
        }
        position += 7;
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt<T: Default>(pub T);

impl<T: Default> VarInt<T> {
    pub fn new(i: T) -> Self {
        Self(i)
    }
}
impl<T: Default + Add<Output = T>> Add for VarInt<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Default + AddAssign> AddAssign for VarInt<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<T: Default + Sub<Output = T>> Sub for VarInt<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Default + SubAssign> SubAssign for VarInt<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<T: Default + Div<Output = T>> Div for VarInt<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl<T: Default + DivAssign> DivAssign for VarInt<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl<T: Default + Mul<Output = T>> Mul for VarInt<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T: Default + MulAssign> MulAssign for VarInt<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Binary for VarInt<i32> {
    fn binary_len(&self) -> usize {
        let value = self.0;
        if value < 0x80 {
            1
        } else if value < 0x4000 {
            2
        } else if value < 0x200000 {
            3
        } else if value < 0x10000000 {
            4
        } else {
            5
        }
    }

    fn to_binary(self) -> Vec<u8> {
        let segment_bits = SEGMENT_BITS as u32;
        let continue_bit = CONTINUE_BIT as u32;
        let mut value = self.0 as u32;
        let mut vec = vec![];
        loop {
            if (value & !segment_bits) == 0 {
                vec.push(value as u8);
                break;
            }
            vec.push(((value & segment_bits) | continue_bit) as u8);
            value >>= 7;
        }
        vec
    }

    fn from_binary(bin: Vec<u8>) -> crate::error::Result<Self>
    where
        Self: Sized,
    {
        let segment_bits = SEGMENT_BITS;
        let continue_bit = CONTINUE_BIT;
        let mut value: u32 = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
        loop {
            if position >= u32::BITS as usize || cursor >= bin.len() {
                break Err(BinaryError::FormatError);
            }
            let current_byte: u8 = bin[cursor];
            value |= ((current_byte & segment_bits) as u32) << position;
            if current_byte & continue_bit == 0 {
                break Ok(Self::new(value as i32));
            }

            cursor += 1;
            position += 7;
        }
    }
}

impl Binary for VarInt<i64> {
    fn binary_len(&self) -> usize {
        let value = self.0 as u64;
        if value < 0x80 {
            1
        } else if value < 0x4000 {
            2
        } else if value < 0x200000 {
            3
        } else if value < 0x10000000 {
            4
        } else if value < 0x800000000 {
            5
        } else if value < 0x40000000000 {
            6
        } else if value < 0x2000000000000 {
            7
        } else if value < 0x100000000000000 {
            8
        } else {
            9
        }
    }

    fn to_binary(self) -> Vec<u8> {
        let segment_bits = SEGMENT_BITS as u64;
        let continue_bit = CONTINUE_BIT as u64;
        let mut value = self.0 as u64;
        let mut vec = vec![];
        loop {
            if (value & !segment_bits) == 0 {
                vec.push(value as u8);
                break;
            }
            vec.push(((value & segment_bits) | continue_bit) as u8);
            value >>= 7;
        }
        vec
    }

    fn from_binary(bin: Vec<u8>) -> crate::error::Result<Self>
    where
        Self: Sized,
    {
        let segment_bits = SEGMENT_BITS;
        let continue_bit = CONTINUE_BIT;
        let mut value: u64 = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
        loop {
            let current_byte: u8 = bin[cursor];
            value |= ((current_byte & segment_bits) as u64) << position;
            if current_byte & continue_bit == 0 {
                break Ok(Self::new(value as i64));
            }
            cursor += 1;
            position += 7;
            if position >= u32::BITS as usize || cursor >= bin.len() {
                break Err(BinaryError::FormatError);
            }
        }
    }
}

pub struct VarIntVisitor;
pub struct VarLongVisitor;

impl<'de> Visitor<'de> for VarIntVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(format!("an integer between {} and {}", i32::MIN, i32::MAX).as_str())
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl<'de> Visitor<'de> for VarLongVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(format!("an integer between {} and {}", i64::MIN, i64::MAX).as_str())
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

impl Serialize for VarInt<i32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl Serialize for VarInt<i64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for VarInt<i32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_i32(VarIntVisitor)
            .map(|i| VarInt(i))
    }
}

impl<'de> Deserialize<'de> for VarInt<i64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_i64(VarLongVisitor)
            .map(|i| VarInt(i))
    }
}

macro_rules! convert {
    ($from:ty => $to:ty) => {
        impl From<$from> for VarInt<$to> {
            fn from(value: $from) -> Self {
                Self(value as $to)
            }
        }

        impl Into<$from> for VarInt<$to> {
            fn into(self) -> $from {
                self.0 as $from
            }
        }
    };
}

convert!(u8    => i32);
convert!(u16   => i32);
convert!(u32   => i32);
convert!(u64   => i32);
convert!(u128  => i32);
convert!(usize => i32);

convert!(i8    => i32);
convert!(i16   => i32);
convert!(i32   => i32);
convert!(i64   => i32);
convert!(i128  => i32);
convert!(isize => i32);

convert!(u8    => i64);
convert!(u16   => i64);
convert!(u32   => i64);
convert!(u64   => i64);
convert!(u128  => i64);
convert!(usize => i64);

convert!(i8    => i64);
convert!(i16   => i64);
convert!(i32   => i64);
convert!(i64   => i64);
convert!(i128  => i64);
convert!(isize => i64);
