use zr_binary::{binary::Binary, error::BinaryError, varint::VarInt};

use crate::error::MalformedError;

#[derive(Debug, Clone)]
pub struct Identifier {
    namespace: String,
    value: String
}

impl Binary for Identifier {
    fn binary_len(&self) -> usize {
        let len = self.namespace.len() + self.value.len() + 1;
        let varint: VarInt<i32> = len.into();
        len + varint.binary_len()
    }

    fn to_binary(self) -> Vec<u8> {
        format!("{}:{}", self.namespace, self.value).to_binary()
    }

    fn from_binary(bin: Vec<u8>) -> zr_binary::error::Result<Self> where Self: Sized {
        let string = String::from_binary(bin)?;
        let parts: Vec<&str> = string.split(':').collect();
        if parts.len() != 2 {
            return Err(BinaryError::FormatError);
        }
        Identifier::new(parts[0], parts[1]).map_err(|_| BinaryError::FormatError)
    }
}

impl  Identifier {
    pub fn new(namespace: &str, value: &str) -> Result<Self, MalformedError> {
        if namespace.chars().all(|c| c.is_alphanumeric() || ".-_".contains(c)) && value.chars().all(|c| c.is_alphanumeric() || ".-_/".contains(c)) {
            Ok(Self { namespace: namespace.to_string(), value: value.to_string() })
        } else {
            Err(MalformedError)
        }
    }
}

impl  ToString for Identifier {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.value)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Identifier;

    #[rstest]
    #[case("minecraft", "stick", true)]
    #[case("minecraft", "diamond_block", true)]
    #[case("minecraft", "1.20.2", true)]
    #[case("minecraft", "-_-", true)]
    #[case("minecraft", "villager/plains", true)]
    #[case("zirconium", "block", true)]
    #[case("m!necraft", "stick", false)]
    #[case("minecraft", "st!ck", false)]
    #[case("minecr@ft", "st:ck", false)]
    #[case("zirconium/minecraft", "block", false)]
    pub fn test_identifier(#[case] namespace: &str, #[case] value: &str, #[case] expected: bool) {
        let identifier = Identifier::new(namespace, value);
        match identifier {
            Ok(identifier) => assert!(expected && identifier.to_string() == format!("{namespace}:{value}")),
            Err(_) => assert!(!expected)  
        }
    }
}