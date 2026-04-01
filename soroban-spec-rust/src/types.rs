use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEventParamLocationV0, ScSpecEventV0, ScSpecTypeDef, ScSpecUdtEnumV0,
    ScSpecUdtErrorEnumV0, ScSpecUdtStructV0, ScSpecUdtUnionV0, StringM,
};

// IMPORTANT: The "docs" fields of spec entries are not output in Rust token
// streams as rustdocs, because rustdocs can contain Rust code, and that code
// will be executed. Generated code may be generated from untrusted Wasm
// containing untrusted spec docs.

#[derive(thiserror::Error, Debug)]
pub enum GenerateError {
    #[error("invalid UTF-8 in spec entry name")]
    InvalidUtf8,
    #[error("invalid Rust identifier: {0:?}")]
    InvalidIdent(String),
}

/// Converts a spec string to a valid UTF-8 string, returning an error if it
/// contains invalid UTF-8.
fn to_utf8<const N: u32>(s: &StringM<N>) -> Result<String, GenerateError> {
    s.to_utf8_string().map_err(|_| GenerateError::InvalidUtf8)
}

/// Creates a Rust identifier from a string, returning an error if it is not a
/// valid identifier.
pub(crate) fn to_ident(s: &str) -> Result<Ident, GenerateError> {
    syn::parse_str::<Ident>(s).map_err(|_| GenerateError::InvalidIdent(s.to_string()))
}

/// Converts a spec string to a Rust identifier, returning an error if it
/// contains invalid UTF-8 or is not a valid identifier.
pub(crate) fn to_ident_from_spec<const N: u32>(s: &StringM<N>) -> Result<Ident, GenerateError> {
    to_ident(&to_utf8(s)?)
}

/// Options for controlling code generation behavior.
#[derive(Default)]
pub struct GenerateOptions {
    /// When true, generated types are annotated so their specs are exported regardless of
    /// Rust visibility (equivalent to `export = true` in macros).
    /// When false (default), generated types behave as if `export = false` in macros.
    pub export: bool,
}

/// Constructs a token stream containing a single struct that mirrors the struct
/// spec.
pub fn generate_struct(spec: &ScSpecUdtStructV0) -> Result<TokenStream, GenerateError> {
    generate_struct_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single struct that mirrors the struct
/// spec, with configurable options.
pub fn generate_struct_with_options(
    spec: &ScSpecUdtStructV0,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let ident = to_ident_from_spec(&spec.name)?;

    if spec.lib.len() > 0 {
        let lib_ident = to_ident_from_spec(&spec.lib)?;
        Ok(quote! {
            type #ident = ::#lib_ident::#ident;
        })
    } else if spec
        .fields
        .iter()
        .map(|f| to_utf8(&f.name).map(|n| n.parse::<usize>().is_ok()))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .all(|is_num| *is_num)
    {
        let fields = spec.fields.iter().map(|f| {
            let f_type = generate_type_ident(&f.type_)?;
            Ok(quote! { pub #f_type })
        }).collect::<Result<Vec<_>, GenerateError>>()?;
        let contracttype_attr = contracttype_attr(opts.export);
        Ok(quote! {
            #contracttype_attr
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #ident ( #(#fields),* );
        })
    } else {
        let fields = spec.fields.iter().map(|f| {
            let f_ident = to_ident_from_spec(&f.name)?;
            let f_type = generate_type_ident(&f.type_)?;
            Ok(quote! { pub #f_ident: #f_type })
        }).collect::<Result<Vec<_>, GenerateError>>()?;
        let contracttype_attr = contracttype_attr(opts.export);
        Ok(quote! {
            #contracttype_attr
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #ident { #(#fields,)* }
        })
    }
}

/// Constructs a token stream containing a single enum that mirrors the union
/// spec.
pub fn generate_union(spec: &ScSpecUdtUnionV0) -> Result<TokenStream, GenerateError> {
    generate_union_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single enum that mirrors the union
/// spec, with configurable options.
pub fn generate_union_with_options(
    spec: &ScSpecUdtUnionV0,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let ident = to_ident_from_spec(&spec.name)?;
    if spec.lib.len() > 0 {
        let lib_ident = to_ident_from_spec(&spec.lib)?;
        Ok(quote! {
            pub type #ident = ::#lib_ident::#ident;
        })
    } else {
        let variants = spec
            .cases
            .iter()
            .map(|c| {
                let name = match c {
                    stellar_xdr::ScSpecUdtUnionCaseV0::VoidV0(v) => &v.name,
                    stellar_xdr::ScSpecUdtUnionCaseV0::TupleV0(t) => &t.name,
                };
                let v_ident = to_ident_from_spec(name)?;
                match c {
                    stellar_xdr::ScSpecUdtUnionCaseV0::VoidV0(_) => Ok(quote! { #v_ident }),
                    stellar_xdr::ScSpecUdtUnionCaseV0::TupleV0(t) => {
                        let v_type = t
                            .type_
                            .iter()
                            .map(generate_type_ident)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(quote! { #v_ident ( #(#v_type),* ) })
                    }
                }
            })
            .collect::<Result<Vec<_>, GenerateError>>()?;
        let contracttype_attr = contracttype_attr(opts.export);
        Ok(quote! {
            #contracttype_attr
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #ident { #(#variants,)* }
        })
    }
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec.
pub fn generate_enum(spec: &ScSpecUdtEnumV0) -> Result<TokenStream, GenerateError> {
    generate_enum_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec, with configurable options.
pub fn generate_enum_with_options(
    spec: &ScSpecUdtEnumV0,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let ident = to_ident_from_spec(&spec.name)?;
    if spec.lib.len() > 0 {
        let lib_ident = to_ident_from_spec(&spec.lib)?;
        Ok(quote! {
            pub type #ident = ::#lib_ident::#ident;
        })
    } else {
        let variants = spec
            .cases
            .iter()
            .map(|c| {
                let v_ident = to_ident_from_spec(&c.name)?;
                let v_value = Literal::u32_unsuffixed(c.value);
                Ok(quote! { #v_ident = #v_value })
            })
            .collect::<Result<Vec<_>, GenerateError>>()?;
        let contracttype_attr = contracttype_attr(opts.export);
        Ok(quote! {
            #contracttype_attr
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #ident { #(#variants,)* }
        })
    }
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec, that is intended for use with errors.
pub fn generate_error_enum(spec: &ScSpecUdtErrorEnumV0) -> Result<TokenStream, GenerateError> {
    generate_error_enum_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec, that is intended for use with errors, with configurable options.
pub fn generate_error_enum_with_options(
    spec: &ScSpecUdtErrorEnumV0,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let ident = to_ident_from_spec(&spec.name)?;
    if spec.lib.len() > 0 {
        let lib_ident = to_ident_from_spec(&spec.lib)?;
        Ok(quote! {
            pub type #ident = ::#lib_ident::#ident;
        })
    } else {
        let variants = spec
            .cases
            .iter()
            .map(|c| {
                let v_ident = to_ident_from_spec(&c.name)?;
                let v_value = Literal::u32_unsuffixed(c.value);
                Ok(quote! { #v_ident = #v_value })
            })
            .collect::<Result<Vec<_>, GenerateError>>()?;
        let contracterror_attr = if opts.export {
            quote! { #[soroban_sdk::contracterror] }
        } else {
            quote! { #[soroban_sdk::contracterror(export = false)] }
        };
        Ok(quote! {
            #contracterror_attr
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #ident { #(#variants,)* }
        })
    }
}

/// Constructs a token stream containing a single struct that mirrors the event
/// spec.
pub fn generate_event(spec: &ScSpecEventV0) -> Result<TokenStream, GenerateError> {
    generate_event_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single struct that mirrors the event
/// spec, with configurable options.
pub fn generate_event_with_options(
    spec: &ScSpecEventV0,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let ident = to_ident_from_spec(&spec.name)?;

    if spec.lib.len() > 0 {
        let lib_ident = to_ident_from_spec(&spec.lib)?;
        Ok(quote! {
            type #ident = ::#lib_ident::#ident;
        })
    } else {
        let topics = spec.prefix_topics.iter().map(|t| t.to_string());
        let fields = spec
            .params
            .iter()
            .map(|p| {
                let p_ident = to_ident_from_spec(&p.name)?;
                let p_type = generate_type_ident(&p.type_)?;
                Ok(match p.location {
                    ScSpecEventParamLocationV0::TopicList => quote! {
                        #[topic]
                        pub #p_ident: #p_type
                    },
                    ScSpecEventParamLocationV0::Data => quote! {
                        pub #p_ident: #p_type
                    },
                })
            })
            .collect::<Result<Vec<_>, GenerateError>>()?;
        let export_attr = if opts.export {
            quote! {}
        } else {
            quote! { export = false, }
        };
        Ok(quote! {
            #[soroban_sdk::contractevent(#export_attr topics = [#(#topics,)*])]
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #ident { #(#fields,)* }
        })
    }
}

/// Returns the `#[soroban_sdk::contracttype]` attribute, optionally with
/// `export = false` when the generated type should not export its spec.
fn contracttype_attr(export: bool) -> TokenStream {
    if export {
        quote! { #[soroban_sdk::contracttype] }
    } else {
        quote! { #[soroban_sdk::contracttype(export = false)] }
    }
}

pub fn generate_type_ident(spec: &ScSpecTypeDef) -> Result<TokenStream, GenerateError> {
    match spec {
        ScSpecTypeDef::Val => Ok(quote! { soroban_sdk::Val }),
        ScSpecTypeDef::U64 => Ok(quote! { u64 }),
        ScSpecTypeDef::I64 => Ok(quote! { i64 }),
        ScSpecTypeDef::U32 => Ok(quote! { u32 }),
        ScSpecTypeDef::I32 => Ok(quote! { i32 }),
        ScSpecTypeDef::U128 => Ok(quote! { u128 }),
        ScSpecTypeDef::I128 => Ok(quote! { i128 }),
        ScSpecTypeDef::Bool => Ok(quote! { bool }),
        ScSpecTypeDef::Symbol => Ok(quote! { soroban_sdk::Symbol }),
        ScSpecTypeDef::Error => Ok(quote! { soroban_sdk::Error }),
        ScSpecTypeDef::Bytes => Ok(quote! { soroban_sdk::Bytes }),
        ScSpecTypeDef::Address => Ok(quote! { soroban_sdk::Address }),
        ScSpecTypeDef::MuxedAddress => Ok(quote! { soroban_sdk::MuxedAddress }),
        ScSpecTypeDef::String => Ok(quote! { soroban_sdk::String }),
        ScSpecTypeDef::Option(o) => {
            let value_ident = generate_type_ident(&o.value_type)?;
            Ok(quote! { Option<#value_ident> })
        }
        ScSpecTypeDef::Result(r) => {
            let ok_ident = generate_type_ident(&r.ok_type)?;
            let error_ident = generate_type_ident(&r.error_type)?;
            Ok(quote! { Result<#ok_ident, #error_ident> })
        }
        ScSpecTypeDef::Vec(v) => {
            let element_ident = generate_type_ident(&v.element_type)?;
            Ok(quote! { soroban_sdk::Vec<#element_ident> })
        }
        ScSpecTypeDef::Map(m) => {
            let key_ident = generate_type_ident(&m.key_type)?;
            let value_ident = generate_type_ident(&m.value_type)?;
            Ok(quote! { soroban_sdk::Map<#key_ident, #value_ident> })
        }
        ScSpecTypeDef::Tuple(t) => {
            let type_idents = t
                .value_types
                .iter()
                .map(generate_type_ident)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(quote! { (#(#type_idents,)*) })
        }
        ScSpecTypeDef::BytesN(b) => {
            let n = Literal::u32_unsuffixed(b.n);
            Ok(quote! { soroban_sdk::BytesN<#n> })
        }
        ScSpecTypeDef::Udt(u) => {
            let ident = to_ident_from_spec(&u.name)?;
            Ok(quote! { #ident })
        }
        ScSpecTypeDef::Void => Ok(quote! { () }),
        ScSpecTypeDef::Timepoint => Ok(quote! { soroban_sdk::Timepoint }),
        ScSpecTypeDef::Duration => Ok(quote! { soroban_sdk::Duration }),
        ScSpecTypeDef::U256 => Ok(quote! { soroban_sdk::U256 }),
        ScSpecTypeDef::I256 => Ok(quote! { soroban_sdk::I256 }),
    }
}

#[cfg(test)]
mod test {
    use crate::ToFormattedString;

    use super::{generate_event, generate_struct, GenerateError};
    use quote::quote;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{
        ScSpecEventDataFormat, ScSpecEventParamLocationV0, ScSpecEventParamV0, ScSpecEventV0,
        ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, ScSymbol, VecM,
    };

    #[test]
    fn test_generate_event_no_topics_no_fields() {
        let tokens = generate_event(&ScSpecEventV0 {
            lib: "".try_into().unwrap(),
            doc: "".try_into().unwrap(),
            name: "MyEvent".try_into().unwrap(),
            prefix_topics: [].try_into().unwrap(),
            params: [].try_into().unwrap(),
            data_format: ScSpecEventDataFormat::Map,
        })
        .unwrap();
        let expect = quote! {
            #[soroban_sdk::contractevent(export = false, topics = [])]
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct MyEvent {}
        };
        assert_eq!(
            tokens.to_formatted_string().unwrap(),
            expect.to_formatted_string().unwrap()
        );
    }

    #[test]
    fn test_generate_event_topics_fields() {
        let tokens = generate_event(&ScSpecEventV0 {
            lib: "".try_into().unwrap(),
            doc: "".try_into().unwrap(),
            name: "MyEvent".try_into().unwrap(),
            prefix_topics: [ScSymbol("my_event".try_into().unwrap())]
                .try_into()
                .unwrap(),
            params: [
                ScSpecEventParamV0 {
                    doc: "".try_into().unwrap(),
                    name: "from".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                    location: ScSpecEventParamLocationV0::Data,
                },
                ScSpecEventParamV0 {
                    doc: "".try_into().unwrap(),
                    name: "to".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                    location: ScSpecEventParamLocationV0::TopicList,
                },
            ]
            .try_into()
            .unwrap(),
            data_format: ScSpecEventDataFormat::Map,
        })
        .unwrap();
        let expect = quote! {
            #[soroban_sdk::contractevent(export = false, topics = ["my_event"])]
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct MyEvent {
                pub from: u32,
                #[topic]
                pub to: u32,
            }
        };
        assert_eq!(
            tokens.to_formatted_string().unwrap(),
            expect.to_formatted_string().unwrap()
        );
    }

    #[test]
    fn test_invalid_utf8_returns_error() {
        let spec = ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: vec![0xff, 0xfe].try_into().unwrap(),
            fields: VecM::default(),
        };
        let result = generate_struct(&spec);
        assert!(matches!(result, Err(GenerateError::InvalidUtf8)));
    }

    #[test]
    fn test_invalid_ident_returns_error() {
        let spec = ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "not a valid ident".try_into().unwrap(),
            fields: VecM::default(),
        };
        let result = generate_struct(&spec);
        assert!(matches!(result, Err(GenerateError::InvalidIdent(_))));
    }

    #[test]
    fn test_invalid_utf8_in_field_name_returns_error() {
        let spec = ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "ValidName".try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: vec![0xff].try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            }]
            .try_into()
            .unwrap(),
        };
        let result = generate_struct(&spec);
        assert!(result.is_err());
    }
}
