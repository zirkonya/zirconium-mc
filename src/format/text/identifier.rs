use crate::{err, tools::utils::bin::{Binary, BinaryError}};
use regex::Regex;
#[derive(Clone)]
pub struct Identifier(String);

err!(IdentifierError { message: String });

impl Identifier {
    pub fn new(string: String) -> Result<Self, IdentifierError> {
        if Identifier::is_identifier(string.clone()) {
            Ok(Self(string))
        } else {
            Err(IdentifierError { message: format!("{} is not identifier", string) })
        }
    }

    pub fn is_identifier(string: String) -> bool {
        let reg_identifier = Regex::new(r"^[a-z0-9./]:[a-z0-9./_]$").unwrap();
        string.len() <= 32_767_usize && reg_identifier.is_match(&string)
    }
}

impl Binary for Identifier {
    fn to_bin(&self) -> Vec<u8> {
        self.0.to_bin()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        if bin.len() == 0 {
            return Err(BinaryError::empty());
        }
        let string = String::from_bin(bin)?;
        if string.len() > 32_727_usize {
            return Err(BinaryError::new(format!("Identifier too long : {}", string.len())));
        }
        Ok(Identifier(string))
    }

    fn byte_length(&self) -> usize {
        self.0.byte_length()
    }
}