use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Meta, MetaList, parse::Parser as _, parse_macro_input};

#[proc_macro_derive(Packets, attributes(packet, default))]
pub fn derive_packets(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let Data::Enum(data_enum) = &input.data else {
        panic!("Packets can only be derived for enums");
    };

    let mut from_match_arms = Vec::new();
    let mut to_match_arms = Vec::new();
    let mut default_variant = None;

    for variant in &data_enum.variants {
        // Vérifier si c'est la variante par défaut
        let is_default = variant
            .attrs
            .iter()
            .any(|attr| attr.path().is_ident("default"));

        if is_default {
            if default_variant.is_some() {
                panic!("Only one variant can be marked with #[default]");
            }
            default_variant = Some(&variant.ident);
            continue; // Skip normal processing for default variant
        }

        // Récupérer les attributs #[packet(...)]
        let packet_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("packet"))
            .expect("Each variant must have a #[packet(...)] attribute or #[default]");

        let (id, bound, state) = parse_packet_attr(packet_attr);

        // Nom de la variante
        let variant_name = &variant.ident;

        // Récupérer le type du champ (suppose un seul champ non nommé)
        let inner_type = match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => &fields.unnamed[0].ty,
            _ => panic!("Each variant must have exactly one unnamed field"),
        };

        // Générer le match arm pour from_packet
        from_match_arms.push(quote! {
            (Some(#bound), Some(#state), #id) => {
                let value = #inner_type::from_bytes(payload)?;
                return Ok(Self::#variant_name(value));
            }
        });

        // Générer le match arm pour to_packet
        to_match_arms.push(quote! {
            Self::#variant_name(inner) => {
                let mut buffer = Vec::new();
                inner.write_to(&mut buffer)?;
                Packet {
                    id: crate::utils::varint::VarInt::new(#id),
                    bound: Some(#bound),
                    state: Some(#state),
                    payload: buffer.into(),
                }
            }
        });
    }

    // Générer le cas par défaut pour from_packet
    let default_case = if let Some(default_var) = default_variant {
        let default_type = data_enum
            .variants
            .iter()
            .find(|v| &v.ident == default_var)
            .and_then(|v| match &v.fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Some(&fields.unnamed[0].ty),
                _ => None,
            })
            .expect("Default variant must have exactly one unnamed field");

        quote! {
            (bound, state, id) => {
                let value = #default_type::from_bytes(payload)?;
                return Ok(Self::#default_var(value));
            }
        }
    } else {
        quote! {
            (bound, state, id) => {
                return Err(std::io::Error::other(format!(
                    "not yet implemented, ({:?}, {:?}, {})",
                    bound, state, id
                )));
            }
        }
    };

    // Générer le cas par défaut pour to_packet (si nécessaire)
    let to_default_case = if let Some(default_var) = default_variant {
        quote! {
            Self::#default_var(inner) => {
                return Err(std::io::Error::other(
                    "Cannot convert default variant to packet without explicit id/bound/state"
                ));
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl #enum_name {
            pub fn from_packet(
                Packet {
                    id,
                    bound,
                    state,
                    payload,
                }: &Packet,
            ) -> std::io::Result<Self> {
                match (bound, state, id.value()) {
                    #(#from_match_arms)*
                    #default_case
                }
            }

            pub fn to_packet(&self) -> std::io::Result<Packet> {
                Ok(match self {
                    #(#to_match_arms)*
                    #to_default_case
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_packet_attr(
    attr: &syn::Attribute,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let meta = &attr.meta;
    let Meta::List(MetaList { tokens, .. }) = meta else {
        panic!("Expected #[packet(...)]");
    };

    let parser = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated;
    let nested = parser
        .parse2(tokens.clone())
        .expect("Failed to parse packet attributes");

    let mut id = None;
    let mut bound = None;
    let mut state = None;

    for meta in nested {
        match meta {
            Meta::NameValue(nv) if nv.path.is_ident("id") => {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    if let Lit::Int(lit_int) = &expr_lit.lit {
                        id = Some(lit_int.clone());
                    }
                }
            }
            Meta::NameValue(nv) if nv.path.is_ident("bound") => {
                if let syn::Expr::Path(expr_path) = &nv.value {
                    bound = Some(expr_path.path.clone());
                }
            }
            Meta::NameValue(nv) if nv.path.is_ident("state") => {
                if let syn::Expr::Path(expr_path) = &nv.value {
                    state = Some(expr_path.path.clone());
                }
            }
            _ => {}
        }
    }

    let id = id.expect("Missing 'id' in #[packet(...)]");
    let bound = bound.expect("Missing 'bound' in #[packet(...)]");
    let state = state.expect("Missing 'state' in #[packet(...)]");

    (quote! { #id }, quote! { #bound }, quote! { #state })
}
