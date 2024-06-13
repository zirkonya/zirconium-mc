pub type Result<T> = std::result::Result<T, BinaryError>;

#[derive(Debug)]
pub enum BinaryError {
    LengthError,
    FormatError,
}