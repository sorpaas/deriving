use syn::{Field, Data, Fields, Variant, Ident, Meta, NestedMeta, Attribute, Lit, LitStr};
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use quote::{quote, quote_spanned};
use proc_macro2::TokenStream;

pub fn normalized_fields(fields: &Fields) -> Vec<(TokenStream, Field)> {
    let mut ret = Vec::new();

    match fields {
        Fields::Named(ref fields) => {
            for f in fields.named.iter() {
                let name = &f.ident;
                ret.push((quote_spanned! { f.span() => #name },
                          f.clone()));
            }
        },
        Fields::Unnamed(ref fields) => {
            for (i, f) in fields.unnamed.iter().enumerate() {
                let i = syn::Index::from(i);
                ret.push((quote_spanned! { f.span() => #i },
                          f.clone()))
            }
        },
        Fields::Unit => (),
    }

    ret
}

pub fn is_fields_variant_unnamed(variant: &Variant) -> bool {
    match variant.fields {
        Fields::Named(_) => false,
        Fields::Unnamed(_) => true,
        Fields::Unit => false,
    }
}

pub fn normalized_variant_match_cause(enum_name: &Ident, variant: &Variant, inner: TokenStream) -> TokenStream {
    let ident = &variant.ident;

    match variant.fields {
        Fields::Named(ref fields) => {
            let fields = fields.named
                .iter()
                .map(|f| {
                    let name = &f.ident;

                    quote_spanned! { f.span() => #name }
                });

            quote! {
                #enum_name::#ident { #(#fields),* } => {
                    #inner
                },
            }
        },
        Fields::Unnamed(ref fields) => {
            let fields = fields.unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| {
                    let name = Ident::new(&format!("v{}", i), f.span());

                    quote_spanned! { f.span() => #name }
                })
                .collect::<Vec<_>>();

            let fields = quote! { #(#fields,)* };

            quote! {
                #enum_name::#ident(#fields) => {
                    let variant = (#fields);

                    #inner
                },
            }
        },
        Fields::Unit => {
            quote! {
                #enum_name::#ident => {
                    #inner
                },
            }
        },
    }
}

pub fn struct_fields(data: &Data) -> Option<&Punctuated<Field, Comma>> {
    match data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => Some(&fields.named),
                Fields::Unnamed(_) => None,
                Fields::Unit => None,
            }
        },
        Data::Enum(_) | Data::Union(_) => None,
    }
}

pub fn has_attribute(prefix: &str, attrs: &[Attribute], name: &str) -> bool {
    for attr in attrs {
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        match meta {
            Meta::List(list) => {
                if list.ident == prefix {
                    for nested in &list.nested {
                        match nested {
                            NestedMeta::Meta(Meta::Word(value)) => {
                                if value == name {
                                    return true
                                }
                            },
                            _ => (),
                        }
                    }
                }
            },
            _ => (),
        }
    }

    false
}

pub fn attribute_value(prefix: &str, attrs: &[Attribute], name: &str) -> Option<LitStr> {
    for attr in attrs {
        let meta = match attr.parse_meta() {
            Ok(meta) => meta,
            Err(_) => continue,
        };

        match meta {
            Meta::List(list) => {
                if list.ident == prefix {
                    for nested in &list.nested {
                        match nested {
                            NestedMeta::Meta(Meta::NameValue(value)) => {
                                if value.ident == name {
                                    match &value.lit {
                                        Lit::Str(s) => {
                                            return Some(s.clone())
                                        },
                                        _ => (),
                                    }
                                }
                            },
                            _ => (),
                        }
                    }
                }
            },
            _ => (),
        }
    }

    None
}
