use std::{borrow::Cow, fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VarInt<I>(I);

impl<I: Default> Default for VarInt<I> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<I> VarInt<I> {
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