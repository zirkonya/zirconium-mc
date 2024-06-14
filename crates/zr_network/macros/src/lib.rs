use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(Packet, attributes(id))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, vis: _, ident, generics, data: _ } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();

    let id = if let Some(attr) = attrs.iter().find(|&attr| attr.path().is_ident("id")) {
        attr.meta.require_name_value().expect("#[id = <id>]").value.to_token_stream()
    } else {
        panic!("The `id` attribute is required");
    };

    let gen = quote! {
        impl #generic_impl #generic_ty zr_network::packet::PacketData for #ident #where_clause {
            const ID: i32 = #id;
        }
    };
    gen.into()
}