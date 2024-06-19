use serde::{Deserialize, Serialize};
use zr_binary::{binary::Binary, error::BinaryError};

pub mod either;

#[derive(Debug, Clone)]
pub struct Nbt<T>(T) where T: Serialize + for<'a> Deserialize<'a> ;

impl <T> Nbt<T>
    where T: Serialize + for<'a> Deserialize<'a>
{
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl <T> Binary for Nbt<T>
    where T: Serialize + for<'a> Deserialize<'a> + Clone
{
    fn binary_len(&self) -> usize {
        // found better way
        self.clone().to_binary().len()
    }

    fn to_binary(self) -> Vec<u8> {
        // temporary, I'll do my own crate
        let mut v = fastnbt::to_bytes(&self.0).expect("flemme de gerer l'erreur en vrai");
        v.remove(2);
        v.remove(1);
        v.into()
    }

    fn from_binary(mut bin: Vec<u8>) -> zr_binary::error::Result<Self> where Self: Sized {
        // temporary, I'll do my own crate
        bin.insert(1, 0);
        bin.insert(1, 0);
        let v: T = fastnbt::from_bytes(&bin).map_err(|_| BinaryError::FormatError)?;
        Ok(Nbt(v))
    }
}