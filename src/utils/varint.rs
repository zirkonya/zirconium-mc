use zr_protocol::serialization::Serialize;

#[derive(Debug, Clone, Copy)]
pub struct VarInt<I> {
    value: I,
}

impl<I> VarInt<I> {
    pub fn new(value: I) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &I {
        &self.value
    }
}

const CONTINUE_BIT: u8 = 0x80;
const SEGMENT_BITS: u8 = 0x7F;

impl Serialize for VarInt<i32> {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        let mut value = self.value as u32; // Convertir en u32 pour le décalage logique
        loop {
            if (value & !(SEGMENT_BITS as u32)) == 0 {
                buffer.write_all(&[value as u8])?;
                return Ok(());
            }
            buffer.write_all(&[((value & SEGMENT_BITS as u32) | CONTINUE_BIT as u32) as u8])?;
            value >>= 7;
        }
    }

    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let mut value: i32 = 0;
        let mut position = 0;
        loop {
            let mut byte = [0u8];
            reader.read_exact(&mut byte)?;
            value |= ((byte[0] & SEGMENT_BITS) as i32) << position;
            if (byte[0] & CONTINUE_BIT) == 0 {
                break;
            }
            position += 7;
            if position >= 32 {
                return Err(std::io::Error::other("VarInt is too big"));
            }
        }
        Ok(Self { value })
    }
}

impl Serialize for VarInt<i64> {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        let mut value = self.value as u64;
        loop {
            if (value & !(SEGMENT_BITS as u64)) == 0 {
                buffer.write_all(&[value as u8])?;
                return Ok(());
            }
            buffer.write_all(&[((value & SEGMENT_BITS as u64) | CONTINUE_BIT as u64) as u8])?;
            value >>= 7;
        }
    }

    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let mut value: i64 = 0;
        let mut position = 0;
        loop {
            let mut byte = [0u8];
            reader.read_exact(&mut byte)?;
            value |= ((byte[0] & SEGMENT_BITS) as i64) << position;
            if (byte[0] & CONTINUE_BIT) == 0 {
                break;
            }
            position += 7;
            if position >= 64 {
                return Err(std::io::Error::other("VarLong is too big"));
            }
        }
        Ok(Self { value })
    }
}

macro_rules! from_into {
    ($from: ty => $into: ty) => {
        impl Into<$into> for VarInt<$from> {
            fn into(self) -> $into {
                self.value as $into
            }
        }

        impl From<$into> for VarInt<$from> {
            fn from(value: $into) -> Self {
                Self {
                    value: value as $from,
                }
            }
        }
    };
}

from_into!(i32 => usize);
from_into!(i32 => i32);
from_into!(i64 => usize);
from_into!(i64 => i64);
