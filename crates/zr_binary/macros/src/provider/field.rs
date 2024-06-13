use quote::{quote, ToTokens, TokenStreamExt};

const SOME_IF_ATTRIBUTE: &str = "some_if";
const PREFIXED_LENGTH_ATTRIBUTE: &str = "prefixed_length";

pub(crate) struct ClassicInner {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

pub(crate) struct OptionInner {
    pub condition: proc_macro2::TokenStream,
    pub ident: syn::Ident,
    pub ty: syn::Type
}

pub(crate) struct PrefixedLength {
    pub length_ty: proc_macro2::TokenStream,
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub generic: syn::Type,
}

pub(crate) enum FieldProviderBinaryLen {
    Classic(ClassicInner),
    Option(OptionInner),
    PrefixedLength(PrefixedLength)
}

pub(crate) enum FieldProviderFromBinary {
    Classic(ClassicInner),
    Option(OptionInner),
    PrefixedLength(PrefixedLength)
}

pub(crate) enum FieldProviderToBinary {
    Classic(ClassicInner),
    Option(OptionInner),
    PrefixedLength(PrefixedLength)
}

macro_rules! remove_quote {
    ($value: ident <$ty:ty>) => {
        syn::parse_str::<$ty>($value.to_string().replace("\"", "").as_str()).expect("invalid condition").to_token_stream()
    };
}

fn get_vector_element_type(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Path(type_path) => {
            if let Some(segement) = type_path.path.segments.iter().find(|&seg| seg.ident == "Vec") {
                if let syn::PathArguments::AngleBracketed(args) = &segement.arguments {
                    if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                        ty.clone()
                    } else {
                        panic!("Work only with Vec")
                    }
                } else {
                    panic!("Work only with Vec")
                }
            } else {
                panic!("Work only with Vec")
            }
        },
        _ => panic!("Work only with Vec"),
    }
}

pub fn field_binary_len_provider(fields: &syn::Fields) -> Vec<FieldProviderBinaryLen> {
    let mut vec = Vec::new();
    for field in fields {
        let ident = field.ident.clone().expect("Not yet implemented for tuple struct");
        let ty = field.ty.clone();
        vec.push(if let Some(attr) = field.attrs.iter().find(|&attr| attr.path().is_ident(SOME_IF_ATTRIBUTE)) {
            let condition = attr.meta.require_name_value().expect("#[some_if = \"<condition>\"]").value.to_token_stream();
            FieldProviderBinaryLen::Option(OptionInner { condition, ident, ty })
        } else if let Some(attr) = field.attrs.iter().find(|&attr| attr.path().is_ident(PREFIXED_LENGTH_ATTRIBUTE)) {
            let length_ty = &attr.meta.require_name_value().expect("#[prefixed_length = \"<type>\"").value.to_token_stream();
            let length_ty = remove_quote!(length_ty <syn::Type>);
            let generic = get_vector_element_type(&ty);
            FieldProviderBinaryLen::PrefixedLength(PrefixedLength { length_ty, ty, ident, generic })
        } else {
            FieldProviderBinaryLen::Classic(ClassicInner { ident, ty })
        });
    }
    vec
}

pub fn field_to_binary_provider(fields: &syn::Fields) -> Vec<FieldProviderToBinary> {
    let mut vec = Vec::new();
    for field in fields {
        let ident = field.ident.clone().expect("Not yet implemented for tuple struct");
        let ty = field.ty.clone();
        vec.push(if let Some(attr) = field.attrs.iter().find(|&attr| attr.path().is_ident(SOME_IF_ATTRIBUTE)) {
            let condition = attr.meta.require_name_value().expect("#[some_if = \"<condition>\"]").value.to_token_stream();
            FieldProviderToBinary::Option(OptionInner { condition, ident, ty })
        } else if let Some(attr) = field.attrs.iter().find(|&attr| attr.path().is_ident(PREFIXED_LENGTH_ATTRIBUTE)) {
            let length_ty = &attr.meta.require_name_value().expect("#[prefixed_length = \"<type>\"").value.to_token_stream();
            let length_ty = remove_quote!(length_ty <syn::Type>);
            let generic = get_vector_element_type(&ty);
            FieldProviderToBinary::PrefixedLength(PrefixedLength { length_ty, ty, ident, generic })
        } else {
            FieldProviderToBinary::Classic(ClassicInner { ident, ty })
        });
    }
    vec
}

pub fn field_from_binary_provider(fields: &syn::Fields) -> Vec<FieldProviderFromBinary> {
    let mut vec = Vec::new();
    for field in fields {
        let ident = field.ident.clone().expect("Not yet implemented for tuple struct");
        let ty = field.ty.clone();
        vec.push(if let Some(attr) = field.attrs.iter().find(|&attr| attr.path().is_ident(SOME_IF_ATTRIBUTE)) {
            let condition = attr.meta.require_name_value().expect("#[some_if = \"<condition>\"]").value.to_token_stream();
            FieldProviderFromBinary::Option(OptionInner { condition, ident, ty })
        } else if let Some(attr) = field.attrs.iter().find(|&attr| attr.path().is_ident(PREFIXED_LENGTH_ATTRIBUTE)) {
            let length_ty = &attr.meta.require_name_value().expect("#[prefixed_length = \"<type>\"").value.to_token_stream();
            let length_ty = remove_quote!(length_ty <syn::Type>);
            let generic = get_vector_element_type(&ty);
            FieldProviderFromBinary::PrefixedLength(PrefixedLength { length_ty, ty, ident, generic })
        } else {
            FieldProviderFromBinary::Classic(ClassicInner { ident, ty })
        });
    }
    vec
}

fn remove_self(condition: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    syn::parse_str::<syn::Expr>(condition.to_string().replace("self.", "").replace("\"", "").as_str()).expect("invalid condition").to_token_stream()
}

impl ToTokens for FieldProviderBinaryLen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let to_append = match self {
            FieldProviderBinaryLen::Classic(ClassicInner {  ident, ty  }) => {
                quote! { <#ty as zr_binary::binary::Binary>::binary_len(&self.#ident) }
            },
            FieldProviderBinaryLen::Option(OptionInner {  condition, ident, ty  }) => {
                let condition = remove_quote!(condition <syn::Expr>);
                quote! {
                    if #condition {
                        <#ty as zr_binary::binary::Binary>::binary_len(&self.#ident)
                    } else {
                        0
                    }
                }
            },
            FieldProviderBinaryLen::PrefixedLength(PrefixedLength { length_ty, ident, ty , generic: _}) => {
                quote! {
                    ({
                        let __len = <#ty as zr_binary::binary::Binary>::binary_len(&self.#ident);
                        let __len_as_ty: #length_ty = __len.into();
                        <#length_ty as zr_binary::binary::Binary>::binary_len(&__len_as_ty) + __len
                    })
                }
            }
        };
        tokens.append_all(to_append);
    }
}

impl ToTokens for FieldProviderToBinary {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let to_append = match self {
            FieldProviderToBinary::Classic(ClassicInner { ident, ty }) => {
                quote! { __binary.append(&mut <#ty as zr_binary::binary::Binary>::to_binary(self.#ident)); }
            },
            FieldProviderToBinary::Option(OptionInner { condition, ident, ty }) => {
                let condition = remove_quote!(condition <syn::Expr>);
                quote! {
                    if #condition {
                        __binary.append(&mut <#ty as zr_binary::binary::Binary>::to_binary(self.#ident));
                    }
                }
            }
            FieldProviderToBinary::PrefixedLength(PrefixedLength { length_ty, ident, ty , generic: _}) => {
                quote! {
                    {
                        let __len: #length_ty = <#ty as zr_binary::binary::Binary>::binary_len(&self.#ident).into();
                        __binary.append(&mut <#length_ty as zr_binary::binary::Binary>::to_binary(__len));
                        __binary.append(&mut <#ty as zr_binary::binary::Binary>::to_binary(self.#ident));
                    }
                }
            }
        };
        tokens.append_all(to_append);
    }
}

impl ToTokens for FieldProviderFromBinary {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let to_append = match self {
            FieldProviderFromBinary::Classic(ClassicInner {  ident, ty  }) => {
                quote! { 
                    let #ident = <#ty as zr_binary::binary::Binary>::from_binary(__binary[__cursor..].to_vec())?;
                    __cursor += <#ty as zr_binary::binary::Binary>::binary_len(&#ident);
                }
            },
            FieldProviderFromBinary::Option(OptionInner {  condition, ident, ty  }) => {
                let condition = remove_self(condition);
                quote! {
                    let #ident = if #condition {
                        let #ident = <#ty as zr_binary::binary::Binary>::from_binary(__binary[__cursor..].to_vec())?;
                        __cursor += <#ty as zr_binary::binary::Binary>::binary_len(&#ident);
                        #ident
                    } else {
                        None
                    };
                }
            },
            FieldProviderFromBinary::PrefixedLength(PrefixedLength { length_ty, ident, ty, generic }) => {
                quote! {
                    let #ident: #ty = {
                        let __length = <#length_ty as zr_binary::binary::Binary>::from_binary(__binary[__cursor..].to_vec())?;
                        __cursor += <#length_ty as zr_binary::binary::Binary>::binary_len(&__length);
                        let __end: usize = __length.into();
                        let mut __vec: #ty = Default::default();
                        for _ in 0..__end {
                            let __current = <#generic as zr_binary::binary::Binary>::from_binary(__binary[__cursor..].to_vec())?;
                            __cursor += <#generic as zr_binary::binary::Binary>::binary_len(&__current);
                            __vec.push(__current);
                        }
                        __vec
                    };
                }
            }
        };
        tokens.append_all(to_append);
    }
}