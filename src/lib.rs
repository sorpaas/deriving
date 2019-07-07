use syn::{Field, Data, Fields, Meta, NestedMeta, Attribute, Lit};
use syn::punctuated::Punctuated;
use syn::token::Comma;

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

pub fn attribute_value(prefix: &str, attrs: &[Attribute], name: &str) -> Option<String> {
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
                                            return Some(s.value())
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
