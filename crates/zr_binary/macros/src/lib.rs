mod provider;

extern crate proc_macro;
use crate::provider::field::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


/// derive macro implements Binary trait.
/// example :
/// ```
/// use zr_binary::{binary::Binary, error};
/// use zr_binary_macros::Binary;
/// 
/// #[derive(Binary)]
/// pub struct Test {
///     field_a: i32,
///     #[some_if = "self.field_a >= 10"]
///     field_b: Option<i32>
/// }
/// 
/// fn main() -> error::Result<()> {
///     let test = Test {
///         field_a: 8,
///         field_b: Some(99)
///     };
///     let binary = test.to_binary();
///     let test_bis = Test::from_binary(binary)?;
///     assert_eq!(test_bis.field_a, test.field_a);
///     assert_eq!(test_bis.field_b, None);
///     Ok(())
/// }
/// ```
#[proc_macro_derive(Binary, attributes(some_if, prefixed_length))]
pub fn binary_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs: _, vis: _, ident, generics, data } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();
    let gen = match data {
        syn::Data::Struct(data_struct) => {
            let fields = data_struct.fields;
            let binary_len_provider = field_binary_len_provider(&fields);
            let to_binary_provider = field_to_binary_provider(&fields);
            let from_binary_provider = field_from_binary_provider(&fields);
            
            let binary_len = provider::binary_len_provider(&binary_len_provider);
            let to_binary = provider::to_binary_provider(&to_binary_provider);
            let from_binary = provider::from_binary_provier(&from_binary_provider);
            quote! {
                impl #generic_impl #generic_ty zr_binary::binary::Binary for #ident #where_clause {
                    #binary_len
                    #to_binary
                    #from_binary
                }
            }
        },
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => todo!(),
    };
    // println!("{gen}");
    gen.into()
}
