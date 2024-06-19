use serde::{Deserialize, Serialize};

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