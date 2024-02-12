pub mod err;
pub mod nbt;
pub mod text;
pub mod uuid;

use self::err::FormatError;
pub trait Format {
    fn format(&self) -> Vec<u8>;
    fn from_file(path: String) -> Result<Self, FormatError> where Self: Sized;
    fn from_bin(bin: Vec<u8>) -> Result<Self, FormatError> where Self: Sized;
}

#[macro_export]
macro_rules! bin_fmt {
    ($fmt:ident => [$($label:ident: $t:ident),+]) => {
        #[derive(Debug)]
        pub struct $fmt {
            $(
                $label: $t,
            )*
        }

        impl crate::tools::utils::bin::Binary for $fmt {
            fn to_bin(&self) -> Vec<u8> {
                let mut vec = Vec::new();
                $(vec.append(&mut self.$label.to_bin());)*
                vec
            }
        
            fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
                let mut _cursor = 0_usize;
                $(
                    if _cursor > bin.len() {
                        return Err(crate::tools::utils::bin::BinaryError::wrong_size(bin.len(), _cursor));
                    }
                    let $label: $t = $t::from_bin(bin[_cursor..bin.len()].to_vec())?;
                    _cursor += $label.byte_length();
                )*
                Ok(Self { $($label,)* })
            }
            
            fn byte_length(&self) -> usize {
                let mut size = 0;
                $(
                    size += self.$label.byte_length(); 
                )*
                size
            }
        }
    };
}