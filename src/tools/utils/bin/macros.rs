#[macro_export]
macro_rules! gen_struct {
    ($data: ident { $($label: ident: $t: ty),* }) => {
        #[derive(Clone)]
        pub struct $data {
            $($label: $t),*
        }

        impl $data {
            pub fn new($($label: $t),*) -> Self {
                Self { $($label),* }
            }

            $(
                pub fn $label(&self) -> $t {
                    self.$label.clone()
                }
            )*
        }
    };
    ($data: ident) => {
        #[derive(Clone)]
        pub struct $data;
        impl $data {
            #[inline(always)]
            pub fn new() -> Self {
                Self
            }
        }
    };
}

#[macro_export]
macro_rules! gen_bin {
    ($data: ident { $($label: ident: $t: ty),* }) => {
        gen_struct!($data { $($label: $t),* });
        impl crate::tools::utils::bin::Binary for $data {
            fn to_bin(&self) -> Vec<u8> {
                let mut v = Vec::new();
                $(
                    v.append(&mut self.$label.to_bin());
                )*
                v
            }

            fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
                let _cursor = 0;
                $(
                    let $label: $t = <$t>::from_bin(bin[_cursor..].to_vec())?;
                    let _cursor = _cursor + $label.byte_length();
                )*
                Ok(Self::new($($label),*))
            }
            
            #[inline(always)]
            fn byte_length(&self) -> usize {
                $(&self.$label.byte_length() +)* 0 
            }
        }
    };
    ($data: ident) => {
        gen_struct!($data);
        impl crate::tools::utils::bin::Binary for $data {
            fn to_bin(&self) -> Vec<u8> {
                vec![]
            }

            fn from_bin(_: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
                Ok(Self)
            }
            
            #[inline(always)]
            fn byte_length(&self) -> usize {
                0
            }
        }
    };
}