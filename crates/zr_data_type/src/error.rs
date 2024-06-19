use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct MalformedError;

impl Display for MalformedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for MalformedError {}