#[macro_export]
macro_rules! err {
    ($err_name: ident { $($att: ident: $t: ty),* }) => {
        #[derive(Debug)]
        pub struct $err_name {
            $($att: $t),*
        }

        impl $err_name {
            pub fn new($($att: $t),*) -> Self {
                Self { $($att),* }
            }

            $(
                pub fn $att(self) -> $t {
                    self.$att
                }
            )*
        }

        impl std::fmt::Display for $err_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut format = String::new();
                $(
                    format += format!("{}, ", self.$att).as_str();
                )*
                write!(f, "{}", format)
            }
        }
    };
}