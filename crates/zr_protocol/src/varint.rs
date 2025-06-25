use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::parser::binary::ToBytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt<I>(I);

impl<I: Default> Default for VarInt<I> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I> VarInt<I> {
    pub const fn max_size() -> usize {
        std::mem::size_of::<I>() + 1
    }

    pub const fn new(i: I) -> Self {
        Self(i)
    }

    pub fn inner(&self) -> &I {
        &self.0
    }

    pub fn into_inner(self) -> I {
        self.0
    }
}

impl<I> AddAssign for VarInt<I>
where
    I: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<I> AddAssign<I> for VarInt<I>
where
    I: AddAssign,
{
    fn add_assign(&mut self, rhs: I) {
        self.0 += rhs
    }
}

impl<I> SubAssign for VarInt<I>
where
    I: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<I> SubAssign<I> for VarInt<I>
where
    I: SubAssign,
{
    fn sub_assign(&mut self, rhs: I) {
        self.0 -= rhs
    }
}

impl<I> MulAssign for VarInt<I>
where
    I: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl<I> MulAssign<I> for VarInt<I>
where
    I: MulAssign,
{
    fn mul_assign(&mut self, rhs: I) {
        self.0 *= rhs
    }
}

impl<I> DivAssign for VarInt<I>
where
    I: DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl<I> DivAssign<I> for VarInt<I>
where
    I: DivAssign,
{
    fn div_assign(&mut self, rhs: I) {
        self.0 /= rhs
    }
}

impl<I> Add for VarInt<I>
where
    I: Add<Output = I>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl<I> Add<I> for VarInt<I>
where
    I: Add<Output = I>,
{
    type Output = Self;

    fn add(self, rhs: I) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl<I> Sub for VarInt<I>
where
    I: Sub<Output = I>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl<I> Sub<I> for VarInt<I>
where
    I: Sub<Output = I>,
{
    type Output = Self;
    fn sub(self, rhs: I) -> Self::Output {
        Self::new(self.0 - rhs)
    }
}

impl<I> Mul for VarInt<I>
where
    I: Mul<Output = I>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.0 * rhs.0)
    }
}

impl<I> Mul<I> for VarInt<I>
where
    I: Mul<Output = I>,
{
    type Output = Self;
    fn mul(self, rhs: I) -> Self::Output {
        Self::new(self.0 * rhs)
    }
}

impl<I> Div for VarInt<I>
where
    I: Div<Output = I>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.0 / rhs.0)
    }
}

impl<I> Div<I> for VarInt<I>
where
    I: Div<Output = I>,
{
    type Output = Self;
    fn div(self, rhs: I) -> Self::Output {
        Self::new(self.0 / rhs)
    }
}

impl VarInt<i32> {
    pub const fn size(&self) -> usize {
        Self::size_of(self.0)
    }

    pub const fn size_of(value: i32) -> usize {
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
        Self::size_of(self.0)
    }

    pub const fn size_of(value: i64) -> usize {
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

impl ToBytes for VarInt<i32> {
    fn bytes_len(&self) -> usize {
        self.size()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
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
        Ok((self.size(), B::from(slice)))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let slice: Vec<u8> = bytes.into();
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;
        let mut value: u32 = 0;
        let mut cursor: usize = 0;
        let mut position: usize = 0;
        loop {
            if position >= u32::BITS as usize || cursor >= slice.len() {
                break Err(());
            }
            let current_byte: u8 = slice[cursor];
            value |= ((current_byte & SEGMENT_BITS) as u32) << position;
            if current_byte & CONTINUE_BIT == 0 {
                let value = value as i32;
                break Ok((Self::size_of(value), Self::new(value)));
            }
            cursor += 1;
            position += 7;
        }
    }
}

macro_rules! primitive_conversion {
    ($primitive: ty => $varint: ty) => {
        impl From<$primitive> for VarInt<$varint> {
            fn from(value: $primitive) -> Self {
                Self(value as $varint)
            }
        }

        impl Into<$primitive> for VarInt<$varint> {
            fn into(self) -> $primitive {
                self.0 as $primitive
            }
        }
    };
}

primitive_conversion!(i8 => i32);
primitive_conversion!(u8 => i32);
primitive_conversion!(i16 => i32);
primitive_conversion!(u16 => i32);
primitive_conversion!(i32 => i32);
primitive_conversion!(u32 => i32);
primitive_conversion!(i64 => i32);
primitive_conversion!(u64 => i32);
primitive_conversion!(i128 => i32);
primitive_conversion!(u128 => i32);
primitive_conversion!(isize => i32);
primitive_conversion!(usize => i32);

primitive_conversion!(i8 => i64);
primitive_conversion!(u8 => i64);
primitive_conversion!(i16 => i64);
primitive_conversion!(u16 => i64);
primitive_conversion!(i32 => i64);
primitive_conversion!(u32 => i64);
primitive_conversion!(i64 => i64);
primitive_conversion!(u64 => i64);
primitive_conversion!(i128 => i64);
primitive_conversion!(u128 => i64);
primitive_conversion!(isize => i64);
primitive_conversion!(usize => i64);
