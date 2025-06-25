// Warning: static context

use std::ops::Deref;

#[derive(Debug)]
pub struct PrefixedLen<L, T>
where
    L: Into<usize> + From<usize> + ToBytes,
    T: ToBytes,
{
    size: L,
    data: T,
}

impl<L, T> PrefixedLen<L, T>
where
    L: Into<usize> + From<usize> + ToBytes,
    T: ToBytes,
{
    pub fn new<P>(data: P) -> Self
    where
        P: Into<T>,
    {
        let data: T = data.into();
        Self {
            size: data.bytes_len().into(),
            data,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<L, T> Deref for PrefixedLen<L, T>
where
    L: Into<usize> + From<usize> + ToBytes,
    T: ToBytes,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub trait ToBytes {
    fn bytes_len(&self) -> usize;
    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>;
    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized;
}

// ***

// ***

macro_rules! number_to_bytes {
    ($t: ty) => {
        impl ToBytes for $t {
            fn bytes_len(&self) -> usize {
                (<$t>::BITS / 8) as usize
            }

            fn to_bytes<B>(&self) -> Result<(usize, B), ()>
            where
                B: From<Vec<u8>>,
            {
                Ok((self.bytes_len(), B::from(self.to_be_bytes().to_vec())))
            }

            fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
            where
                B: Into<Vec<u8>>,
                Self: Sized,
            {
                const LEN: usize = (<$t>::BITS / 8) as usize;
                let bytes: Vec<u8> = bytes.into();
                if bytes.len() < LEN {
                    Err(())
                } else {
                    // ?
                    let mut array = [0_u8; LEN];
                    array.copy_from_slice(&bytes[0..LEN]);
                    Ok((LEN, Self::from_be_bytes(array)))
                }
            }
        }
    };
    ($t: ty; size: $s: literal) => {
        impl ToBytes for $t {
            fn bytes_len(&self) -> usize {
                $s
            }

            fn to_bytes<B>(&self) -> Result<(usize, B), ()>
            where
                B: From<Vec<u8>>,
            {
                Ok(($s, B::from(self.to_be_bytes().to_vec())))
            }

            fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
            where
                B: Into<Vec<u8>>,
                Self: Sized,
            {
                const LEN: usize = $s;
                let bytes: Vec<u8> = bytes.into();
                if bytes.len() < LEN {
                    Err(())
                } else {
                    // ?
                    let mut array = [0_u8; LEN];
                    array.copy_from_slice(&bytes[0..LEN]);
                    Ok((LEN, Self::from_be_bytes(array)))
                }
            }
        }
    };
}

impl ToBytes for bool {
    fn bytes_len(&self) -> usize {
        1
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        Ok((1, B::from(self.then_some(vec![0x01]).unwrap_or(vec![0x00]))))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let bytes: Vec<u8> = bytes.into();
        if bytes.len() < 1 {
            Err(())
        } else {
            Ok((1, bytes[0] != 0))
        }
    }
}

number_to_bytes!(i8);
number_to_bytes!(u8);

number_to_bytes!(i16);
number_to_bytes!(u16);

number_to_bytes!(i32);
number_to_bytes!(u32);

number_to_bytes!(i64);
number_to_bytes!(u64);

number_to_bytes!(i128);
number_to_bytes!(u128);

number_to_bytes!(isize);
number_to_bytes!(usize);

number_to_bytes!(f32; size: 4);
number_to_bytes!(f64; size: 8);

#[allow(dead_code)]
/// Warning : no prefixed size
impl<T> ToBytes for [T]
where
    T: ToBytes,
    Self: Sized,
{
    fn bytes_len(&self) -> usize {
        self.iter().map(|elem| elem.bytes_len()).sum()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        let mut size = 0;
        let mut result = Vec::with_capacity(self.bytes_len());
        for elem in self {
            let (current_size, mut bytes) = elem.to_bytes()?;
            size += current_size;
            result.append(&mut bytes);
        }
        Ok((size, B::from(result)))
    }

    fn from_bytes<B>(_: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        unreachable!()
    }
}

impl<T, const SIZE: usize> ToBytes for [T; SIZE]
where
    T: ToBytes + Default + Copy,
    Self: Sized,
{
    fn bytes_len(&self) -> usize {
        self.iter().map(|elem| elem.bytes_len()).sum()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        let mut size = 0;
        let mut result = Vec::with_capacity(self.bytes_len());
        for elem in self {
            let (current_size, mut bytes) = elem.to_bytes()?;
            size += current_size;
            result.append(&mut bytes);
        }
        Ok((size, B::from(result)))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let mut buffer = Vec::new();
        let mut cursor = 0;
        let bytes: Vec<u8> = bytes.into();
        for _ in 0..SIZE {
            let (current_size, value) = T::from_bytes(&bytes[cursor..])?;
            cursor += current_size;
            if cursor >= bytes.len() {
                return Err(());
            }
            buffer.push(value);
        }
        let mut slice: Self = [T::default(); SIZE];
        slice.copy_from_slice(&buffer);
        Ok((cursor, slice))
    }
}

impl<T> ToBytes for Vec<T>
where
    T: ToBytes,
{
    fn bytes_len(&self) -> usize {
        self.iter().map(|elem| elem.bytes_len()).sum()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        let mut size = 0;
        let mut result = Vec::with_capacity(self.bytes_len());
        for elem in self {
            let (current_size, mut bytes) = elem.to_bytes()?;
            size += current_size;
            result.append(&mut bytes);
        }
        Ok((size, B::from(result)))
    }

    fn from_bytes<B>(_: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        panic!("from bytes not implemented for Vec<T>")
    }
}

impl ToBytes for String {
    fn bytes_len(&self) -> usize {
        self.len()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        Ok((self.bytes_len(), B::from(self.bytes().collect::<Vec<u8>>())))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let bytes: Vec<u8> = bytes.into();
        let string = String::from_utf8(bytes).unwrap();
        Ok((string.len(), string))
    }
}

impl<L, T> ToBytes for PrefixedLen<L, T>
where
    L: Into<usize> + From<usize> + ToBytes,
    T: ToBytes,
{
    fn bytes_len(&self) -> usize {
        self.data.bytes_len() + self.size.bytes_len()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        let (prefix_size, mut prefix_bytes): (usize, Vec<u8>) = self.size.to_bytes()?;
        let (data_size, mut data_bytes): (usize, Vec<u8>) = self.data.to_bytes()?;
        prefix_bytes.append(&mut data_bytes);
        Ok((prefix_size + data_size, B::from(prefix_bytes)))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let bytes: Vec<u8> = bytes.into();
        let (cursor, prefix_length) = L::from_bytes(&bytes[..])?;
        let len: usize = prefix_length.into();
        let end = cursor + len;
        let (size, data) = T::from_bytes(&bytes[cursor..end])?;
        Ok((cursor + size, Self::new(data)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::binary::{PrefixedLen, ToBytes},
        varint::VarInt,
    };

    #[test]
    fn test_slice() {
        let slice = [1u8, 2, 3, 4];
        let (count, bytes): (_, Vec<u8>) = slice.to_bytes().unwrap();
        assert_eq!(count, 4);
        assert_eq!(bytes, vec![1u8, 2, 3, 4]);
    }

    #[test]
    fn test_prefixed_len() {
        let slice = PrefixedLen::<VarInt<i32>, Vec<u8>>::new(vec![1u8, 2, 3, 4]);
        let (count, bytes): (_, Vec<u8>) = slice.to_bytes().unwrap();
        assert_eq!(count, 5);
        assert_eq!(bytes, vec![4u8, 1, 2, 3, 4]);
    }
}
