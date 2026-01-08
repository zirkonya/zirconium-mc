use std::{
    fmt::{Debug, Display},
    sync::Arc,
};
use crate::utils::varint::VarInt;
use zr_protocol::{message::size::PrefixSize, serialization::Serialize};

#[derive(Clone)]
pub struct Identifier(Arc<[u8]>);

impl Identifier {
    pub fn string(&self) -> Arc<str> {
        String::from_utf8_lossy(&self.0).into()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Identifier").field(&self.string()).finish()
    }
}

impl From<&[u8]> for Identifier {
    fn from(value: &[u8]) -> Self {
        Identifier(value.into())
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().into())
    }
}

impl Serialize for Identifier {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        PrefixSize::<VarInt<i32>, _>::new(self.0.clone()).write_to(buffer)
    }

    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let raw: Arc<[u8]> = PrefixSize::<VarInt<i32>, _>::read_from(reader)?.into_data();
        Ok(Identifier(raw))
    }
}
