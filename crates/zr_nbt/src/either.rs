use serde::{Deserialize, Serialize};
use zr_binary::binary::Binary;

/// The `Either` enum is a data structure for handling multiple cases.
/// It allows to represent two possible cases.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Either<One, Other> {
    One(One),
    Other(Other),
}

/// Implement the Default trait for Either,
/// When no value is specified, the default value for `Either` will be the `One` variant with the default value of `One`.
impl<One: Default, Other> Default for Either<One, Other> {
    fn default() -> Self {
        Self::One(One::default())
    }
}

impl<One, Other> Binary for Either<One, Other>
where
    One: Binary,
    Other: Binary,
{
    fn binary_len(&self) -> usize {
        match self {
            Self::One(one) => one.binary_len(),
            Self::Other(other) => other.binary_len(),
        }
    }

    fn to_binary(self) -> Vec<u8> {
        match self {
            Self::One(one) => one.to_binary(),
            Self::Other(other) => other.to_binary(),
        }
    }

    fn from_binary(_: Vec<u8>) -> zr_binary::error::Result<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
