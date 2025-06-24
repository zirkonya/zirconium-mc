use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Packet, attributes(id, state, direction))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics,
        data,
    } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();

    quote! {
        impl #generic_impl #generic_ty ::parser::binary::ToBytes for #ident #where_clause {
            
        }
    }.into()
}