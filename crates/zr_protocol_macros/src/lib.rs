use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Meta};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();
    let PacketAttributes { id, state, direction } = parse_packet_attributes(&attrs);
    quote! {
        impl #generic_impl #generic_ty crate::packet::PacketData for #ident #where_clause {
            const ID: i32 = #id;
            const STATE: crate::packet::state::PacketState = crate::packet::state::PacketState::#state;
            const DIRECTION: crate::packet::direction::PacketDirection = crate::packet::direction::PacketDirection::#direction;

            fn packet_name() -> &'static str {
                stringify!(#ident)
            }
        }
    }.into()
}

struct PacketAttributes {
    id: i32,
    state: syn::Ident,
    direction: syn::Ident,
}

fn parse_packet_attributes(attrs: &[Attribute]) -> PacketAttributes {
    let mut id = None;
    let mut state = None;
    let mut direction = None;
    
    for attr in attrs {
        if attr.path().is_ident("packet") {
            if let Ok(meta_list) = attr.meta.require_list() {
                for nested in meta_list.parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
                ).unwrap() {
                    match nested {
                        Meta::NameValue(nv) if nv.path.is_ident("id") => {
                            if let syn::Expr::Lit(lit) = &nv.value {
                                if let syn::Lit::Int(int_lit) = &lit.lit {
                                    id = Some(int_lit.base10_parse::<i32>().unwrap());
                                }
                            }
                        }
                        Meta::NameValue(nv) if nv.path.is_ident("state") => {
                            if let syn::Expr::Path(path) = &nv.value {
                                state = Some(path.path.get_ident().unwrap().clone());
                            }
                        }
                        Meta::NameValue(nv) if nv.path.is_ident("direction") => {
                            if let syn::Expr::Path(path) = &nv.value {
                                direction = Some(path.path.get_ident().unwrap().clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    
    PacketAttributes {
        id: id.expect("packet attribute must specify id"),
        state: state.expect("packet attribute must specify state"),
        direction: direction.expect("packet attribute must specify direction"),
    }
}