use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum WorkerError {
    Stoped,
}

unsafe impl Sync for WorkerError {}

impl Error for WorkerError {}

impl Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
