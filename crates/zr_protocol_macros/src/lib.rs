use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

mod packet;

#[proc_macro_derive(Packet, attributes(packet))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        ident,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();
    let packet::PacketAttributes {
        id,
        state,
        direction,
    } = packet::parse_packet_attributes(&attrs);
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

#[proc_macro_derive(ToBytes)]
pub fn to_bytes_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();
    let (idents, types) = match data {
        Data::Struct(DataStruct { fields, .. }) => {
            let mut idents = Vec::new();
            let mut types = Vec::new();
            for field in fields {
                idents.push(field.ident);
                types.push(field.ty);
            }
            (idents, types)
        },
        _ => panic!()
    };
    quote! {
        impl #generic_impl #generic_ty crate::parser::binary::ToBytes for #ident #where_clause {
            fn bytes_len(&self) -> usize {
                #(self.#idents.bytes_len())+*
            }

            fn to_bytes<B>(&self) -> Result<(usize, B), ()>
            where
                B: From<Vec<u8>> {
                let len = self.bytes_len();
                let mut buffer = Vec::with_capacity(len);
                #(
                    let (_, mut bytes): (_, Vec<u8>) = self.#idents.to_bytes()?;
                    buffer.append(&mut bytes);
                )*
                Ok((len, B::from(buffer)))
            }

            fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
            where
                B: Into<Vec<u8>>,
                Self: Sized {
                    let bytes = bytes.into();
                    let mut cursor = 0;
                    #(
                        let (len, #idents) = <#types as crate::parser::binary::ToBytes>::from_bytes(&bytes[cursor..])?;
                        cursor += len;
                    )*
                    Ok((len, Self {
                        #(#idents),*
                    }))
            }
        }
    }.into()
}