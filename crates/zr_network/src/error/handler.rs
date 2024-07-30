use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HandlerError {
    // TODO : Handler Error
}

unsafe impl Sync for HandlerError {}

impl Error for HandlerError {}

impl Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
