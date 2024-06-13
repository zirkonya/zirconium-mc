use field::*;
use quote::quote;

#[macro_use]
pub mod field;

pub(crate) fn binary_len_provider(fields: &Vec<FieldProviderBinaryLen>) -> proc_macro2::TokenStream {
    quote! {
        fn binary_len(&self) -> usize {
            #(#fields +)* 0
        }
    }
}

pub(crate) fn to_binary_provider(fields: &Vec<FieldProviderToBinary>) -> proc_macro2::TokenStream {
    quote! {
        fn to_binary(self) -> Vec<u8> {
            let mut __binary = Vec::new();
            #(#fields)*
            __binary
        }
    }
}

pub(crate) fn from_binary_provier(fields: &Vec<FieldProviderFromBinary>) -> proc_macro2::TokenStream {
    let field_names: Vec<_> = fields.iter().map(|field| match field {
        FieldProviderFromBinary::Classic(ClassicInner {  ident, ty: _  }) => ident,
        FieldProviderFromBinary::Option(OptionInner {  condition: _, ident, ty: _  }) => ident,
        FieldProviderFromBinary::PrefixedLength(PrefixedLength {  length_ty: _, ident, ty: _ , generic: _ }) => ident,
    }).collect();
    quote! {
        fn from_binary(__binary: Vec<u8>) -> zr_binary::error::Result<Self> where Self: Sized {
            let mut __cursor = 0;
            #(#fields)*
            Ok(Self {
                #(#field_names),*
            })
        }
    }
}