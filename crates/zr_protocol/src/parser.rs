use std::{io::Write, io::Read};

pub enum BinaryError {
    Error
}

pub trait BinaryWriter {
    fn write<B: Write>(&self, buffer: &mut B) -> Result<usize, BinaryError>;
}

pub trait BinaryReader {
    fn write<B: Read>(buffer: &B) -> Result<usize, BinaryError>;
}

impl BinaryWriter for bool {
    fn write<B: Write>(&self, buffer: &mut B) -> Result<usize, BinaryError> {
        buffer.write(&[*self as u8]).map_err(|_| BinaryError::Error)
    }
}

macro_rules! write_big_endian {
    ($t: ty) => {
        impl BinaryWriter for $t {
            fn write<B: Write>(&self, buffer: &mut B) -> Result<usize, BinaryError> {
                buffer.write(&self.to_be_bytes()).map_err(|_| BinaryError::Error)
            }
        }
    };
}

write_big_endian!(i8);
write_big_endian!(u8);

write_big_endian!(i16);
write_big_endian!(u16);

write_big_endian!(i32);
write_big_endian!(u32);

write_big_endian!(i64);
write_big_endian!(u64);

write_big_endian!(i128);
write_big_endian!(u128);

write_big_endian!(f32);
write_big_endian!(f64);
