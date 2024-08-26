use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Block, DeriveInput, ImplItem, ImplItemFn, ItemImpl, Meta, MetaList};

extern crate proc_macro;

#[proc_macro_derive(Packet, attributes(id))]
pub fn packet_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        attrs,
        vis: _,
        ident,
        generics,
        data: _,
    } = parse_macro_input!(input as DeriveInput);
    let (generic_impl, generic_ty, where_clause) = generics.split_for_impl();

    let id = if let Some(attr) = attrs.iter().find(|&attr| attr.path().is_ident("id")) {
        attr.meta
            .require_name_value()
            .expect("#[id = <id>]")
            .value
            .to_token_stream()
    } else {
        panic!("The `id` attribute is required");
    };

    quote! {
        impl #generic_impl #generic_ty zr_network::packet::PacketData for #ident #where_clause {
            const ID: i32 = #id;
        }
    }
    .into()
}

// return (Event is call, the block to call when event is reach)
fn parse_on(input: ImplItemFn) -> (TokenStream, Block) {
    let ImplItemFn { attrs, block, .. } = input;
    let mut event: Option<TokenStream> = None;
    // found on attributes else panic!
    for attr in attrs {
        if let Meta::List(MetaList { path, tokens, .. }) = attr.meta {
            if path.is_ident("on") {
                event = Some(tokens.into());
                break;
            }
        }
    }
    if event.is_none() {
        panic!("#[on(<event>)] is required")
    }
    (
        event.expect("attribute on is requiered : #[on(Event..)]"),
        block,
    )
}

fn parse_match(mut blocks: Vec<(proc_macro2::TokenStream, Block)>) -> proc_macro2::TokenStream {
    blocks.sort_by_key(|(k, _)| k.to_string());
    let mut last = &blocks[0].0;
    let mut event_blocks: Vec<proc_macro2::TokenStream> = vec![];
    let mut current: proc_macro2::TokenStream = quote! {};
    for (event, block) in &blocks {
        if last.to_string() != event.to_string() {
            event_blocks.push(quote! {
                #last => {
                    #current
                }
            });
        }
        current = quote! {#current #block};
        last = event;
    }
    event_blocks.push(quote! {
        #last => {
            #current
        }
    });
    quote! {
        match event {
            #(#event_blocks)*
            _ => {}
        }
    }
}

// TODO : Add priority tag
#[proc_macro_attribute]
pub fn listener(_: TokenStream, input: TokenStream) -> TokenStream {
    let ItemImpl {
        generics,
        self_ty,
        items,
        ..
    } = parse_macro_input!(input as ItemImpl);
    let mut blocks: Vec<(proc_macro2::TokenStream, Block)> = Vec::new();
    for item in items {
        if let ImplItem::Fn(function) = item {
            let (event, block) = parse_on(function);
            blocks.push((event.into(), block));
        }
    }
    let match_pattern = parse_match(blocks);
    let gen = quote! {
        impl #generics Listener for #self_ty {
            fn handle_event(&mut self, event: &Event) -> Result<(), Box<dyn Error>> {
                #match_pattern
                Ok(())
            }
        }
    }
    .into();
    println!("{gen}");
    gen
}
