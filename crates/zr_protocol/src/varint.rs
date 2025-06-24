use std::{borrow::Cow, fmt::{self, Debug}, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt<I>(I);

impl<I: Default> Default for VarInt<I> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I> VarInt<I> {
    pub fn new(i: I) -> Self {
        Self(i)
    }

    pub fn inner(&self) -> &I {
        &self.0
    }

    pub fn into_inner(self) -> I {
        self.0
    }
}

// Implémentations Serialize pour VarInt<i32>
impl Serialize for VarInt<i32> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            // Pour JSON/formats lisibles : sérialise la valeur directement
            serializer.serialize_i32(self.0)
        } else {
            // Pour binaire : utilise l'encodage VarInt
            let bytes: Cow<[u8]> = (*self).try_into()
                .map_err(|_| serde::ser::Error::custom("Failed to encode VarInt"))?;
            serializer.serialize_bytes(&bytes)
        }
    }
}

// Implémentations Deserialize pour VarInt<i32>
struct VarIntI32Visitor;

impl<'de> Visitor<'de> for VarIntI32Visitor {
    type Value = VarInt<i32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a VarInt<i32>")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VarInt::new(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            Ok(VarInt::new(value as i32))
        } else {
            Err(E::custom(format!("i64 value {} out of range for i32", value)))
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value <= i32::MAX as u32 {
            Ok(VarInt::new(value as i32))
        } else {
            Err(E::custom(format!("u32 value {} out of range for i32", value)))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value <= i32::MAX as u64 {
            Ok(VarInt::new(value as i32))
        } else {
            Err(E::custom(format!("u64 value {} out of range for i32", value)))
        }
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        VarInt::try_from(Cow::Borrowed(value))
            .map_err(|_| E::custom("Failed to decode VarInt from bytes"))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        VarInt::try_from(Cow::Owned(value))
            .map_err(|_| E::custom("Failed to decode VarInt from byte buffer"))
    }
}

impl<'de> Deserialize<'de> for VarInt<i32> {
    fn deserialize<D>(deserializer: D) -> Result<VarInt<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            // Pour JSON/formats lisibles : désérialise comme i32
            deserializer.deserialize_i32(VarIntI32Visitor)
        } else {
            // Pour binaire : désérialise comme bytes
            deserializer.deserialize_bytes(VarIntI32Visitor)
        }
    }
}

// Implémentations Serialize pour VarInt<i64>
impl Serialize for VarInt<i64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            // Pour JSON/formats lisibles : sérialise la valeur directement
            serializer.serialize_i64(self.0)
        } else {
            // Pour binaire : utilise l'encodage VarInt
            let bytes: Cow<[u8]> = (*self).try_into()
                .map_err(|_| serde::ser::Error::custom("Failed to encode VarInt"))?;
            serializer.serialize_bytes(&bytes)
        }
    }
}

// Implémentations Deserialize pour VarInt<i64>
struct VarIntI64Visitor;

impl<'de> Visitor<'de> for VarIntI64Visitor {
    type Value = VarInt<i64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a VarInt<i64>")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VarInt::new(value as i64))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VarInt::new(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(VarInt::new(value as i64))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value <= i64::MAX as u64 {
            Ok(VarInt::new(value as i64))
        } else {
            Err(E::custom(format!("u64 value {} out of range for i64", value)))
        }
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        VarInt::try_from(Cow::Borrowed(value))
            .map_err(|_| E::custom("Failed to decode VarInt from bytes"))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        VarInt::try_from(Cow::Owned(value))
            .map_err(|_| E::custom("Failed to decode VarInt from byte buffer"))
    }
}

impl<'de> Deserialize<'de> for VarInt<i64> {
    fn deserialize<D>(deserializer: D) -> Result<VarInt<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            // Pour JSON/formats lisibles : désérialise comme i64
            deserializer.deserialize_i64(VarIntI64Visitor)
        } else {
            // Pour binaire : désérialise comme bytes
            deserializer.deserialize_bytes(VarIntI64Visitor)
        }
    }
}

impl<I> AddAssign for VarInt<I> where I: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<I> AddAssign<I> for VarInt<I> where I: AddAssign {
    fn add_assign(&mut self, rhs: I) {
        self.0 += rhs
    }
}

impl<I> SubAssign for VarInt<I> where I: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<I> SubAssign<I> for VarInt<I> where I: SubAssign {
    fn sub_assign(&mut self, rhs: I) {
        self.0 -= rhs
    }
}

impl<I> MulAssign for VarInt<I> where I: MulAssign {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl<I> MulAssign<I> for VarInt<I> where I: MulAssign {
    fn mul_assign(&mut self, rhs: I) {
        self.0 *= rhs
    }
}

impl<I> DivAssign for VarInt<I> where I: DivAssign {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl<I> DivAssign<I> for VarInt<I> where I: DivAssign {
    fn div_assign(&mut self, rhs: I) {
        self.0 /= rhs
    }
}

impl<I> Add for VarInt<I> where I: Add<Output = I> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl<I> Add<I> for VarInt<I> where I: Add<Output = I> {
    type Output = Self;

    fn add(self, rhs: I) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl<I> Sub for VarInt<I> where I: Sub<Output = I> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl<I> Sub<I> for VarInt<I> where I: Sub<Output = I> {
    type Output = Self;
    fn sub(self, rhs: I) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl<I> Mul for VarInt<I> where I: Mul<Output = I> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl<I> Mul<I> for VarInt<I> where I: Mul<Output = I> {
    type Output = Self;
    fn mul(self, rhs: I) -> Self::Output {
        Self::new(self.0 * rhs)
    }
}

impl<I> Div for VarInt<I> where I: Div<Output = I> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 / rhs.0)
    }
}

impl<I> Div<I> for VarInt<I> where I: Div<Output = I> {
    type Output = Self;
    fn div(self, rhs: I) -> Self::Output {
        Self::new(self.0 / rhs)
    }
}

impl<I> From<I> for VarInt<I> where I: Copy {
    fn from(value: I) -> Self {
        Self(value)
    }
}

impl VarInt<i32> {
    pub const fn size(&self) -> usize {
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
}

impl VarInt<i64> {
    pub const fn size(&self) -> usize {
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
}

impl<'a> TryInto<Cow<'a, [u8]>> for VarInt<i32> {
    type Error = ();
    fn try_into(self) -> Result<Cow<'a, [u8]>, Self::Error> {
        let mut slice = Vec::with_capacity(self.size());
        const SEGMENT_BITS: u32 = 0x7F;
        const CONTINUE_BIT: u32 = 0x80;
        let mut value = self.0 as u32;
        loop {
            if value & !SEGMENT_BITS == 0 {
                slice.push(value as u8);
                break;
            }
            slice.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
            value >>= 7;
        }
        slice.shrink_to_fit();
        Ok(Cow::Owned(slice.to_owned()))
    }
}

impl<'a> TryFrom<Cow<'a, [u8]>> for VarInt<i32> {
    type Error = ();

    fn try_from(slice: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;
        let mut value: u32  = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
        loop {
            if position >= u32::BITS as usize || cursor >= slice.len() {
                break Err(());
            }
            let current_byte: u8 = slice[cursor];
            value |= ((current_byte & SEGMENT_BITS) as u32) << position;
            if current_byte & CONTINUE_BIT == 0 {
                break Ok(Self::new(value as i32));
            }
            cursor += 1;
            position += 7;
        }
    }
}

impl<'a> TryInto<Cow<'a, [u8]>> for VarInt<i64> {
    type Error = ();
    fn try_into(self) -> Result<Cow<'a, [u8]>, Self::Error> {
        let mut slice = Vec::with_capacity(self.size());
        const SEGMENT_BITS: u64 = 0x7F;
        const CONTINUE_BIT: u64 = 0x80;
        let mut value = self.0 as u64;
        loop {
            if value & !SEGMENT_BITS == 0 {
                slice.push(value as u8);
                break;
            }
            slice.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);
            value >>= 7;
        }
        slice.shrink_to_fit();
        Ok(Cow::Owned(slice.to_owned()))
    }
}

impl<'a> TryFrom<Cow<'a, [u8]>> for VarInt<i64> {
    type Error = ();

    fn try_from(slice: Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;
        let mut value: u32  = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
        loop {
            if position >= u32::BITS as usize || cursor >= slice.len() {
                break Err(());
            }
            let current_byte: u8 = slice[cursor];
            value |= ((current_byte & SEGMENT_BITS) as u32) << position;
            if current_byte & CONTINUE_BIT == 0 {
                break Ok(Self::new(value as i64));
            }
            cursor += 1;
            position += 7;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use crate::varint::VarInt;

    #[test]
    pub fn from() {
        let slice: &[u8] = &[255_u8, 255, 255, 255, 15];
        let varint: VarInt<i32> = VarInt::try_from(Cow::Borrowed(slice)).unwrap();
        assert_eq!(varint.0, -1);
    }

    #[test]
    pub fn into() {
        let i = VarInt::new(-1);
        let into: Cow<[u8]> = i.try_into().unwrap();
        
        assert_eq!(into.as_ref(), &[255, 255, 255, 255, 15]);
    }
}