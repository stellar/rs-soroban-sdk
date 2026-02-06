use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEventParamLocationV0, ScSpecEventV0, ScSpecTypeDef, ScSpecUdtEnumV0,
    ScSpecUdtErrorEnumV0, ScSpecUdtStructV0, ScSpecUdtUnionV0,
};

// IMPORTANT: The "docs" fields of spec entries are not output in Rust token
// streams as rustdocs, because rustdocs can contain Rust code, and that code
// will be executed. Generated code may be generated from untrusted Wasm
// containing untrusted spec docs.

// TODO: Replace the unwrap()s in this code with returning Result.
// TODO: Create Idents in a way that we can get a Result back and return it too
// because at the moment the format_ident! calls can panic if the inputs do not
// result in a valid ident.

/// Options for controlling code generation behavior.
#[derive(Default)]
pub struct GenerateOptions {
    /// When true, generated types use default export behavior (pub types export their spec).
    /// When false (default), generated types use `export = false`.
    pub export: bool,
}

/// Constructs a token stream containing a single struct that mirrors the struct
/// spec.
pub fn generate_struct(spec: &ScSpecUdtStructV0) -> TokenStream {
    generate_struct_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single struct that mirrors the struct
/// spec, with configurable options.
pub fn generate_struct_with_options(
    spec: &ScSpecUdtStructV0,
    opts: &GenerateOptions,
) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_utf8_string().unwrap());

    if spec.lib.len() > 0 {
        let lib_ident = format_ident!("{}", spec.lib.to_utf8_string().unwrap());
        quote! {
            type #ident = ::#lib_ident::#ident;
        }
    } else if spec
        .fields
        .iter()
        .all(|f| f.name.to_utf8_string().unwrap().parse::<usize>().is_ok())
    {
        let fields = spec.fields.iter().map(|f| {
            let f_type = generate_type_ident(&f.type_);
            quote! { pub #f_type }
        });
        let contracttype_attr = contracttype_attr(opts.export);
        quote! {
            #contracttype_attr
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #ident ( #(#fields),* );
        }
    } else {
        let fields = spec.fields.iter().map(|f| {
            let f_ident = format_ident!("{}", f.name.to_utf8_string().unwrap());
            let f_type = generate_type_ident(&f.type_);
            quote! { pub #f_ident: #f_type }
        });
        let contracttype_attr = contracttype_attr(opts.export);
        quote! {
            #contracttype_attr
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #ident { #(#fields,)* }
        }
    }
}

/// Constructs a token stream containing a single enum that mirrors the union
/// spec.
pub fn generate_union(spec: &ScSpecUdtUnionV0) -> TokenStream {
    generate_union_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single enum that mirrors the union
/// spec, with configurable options.
pub fn generate_union_with_options(spec: &ScSpecUdtUnionV0, opts: &GenerateOptions) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_utf8_string().unwrap());
    if spec.lib.len() > 0 {
        let lib_ident = format_ident!("{}", spec.lib.to_utf8_string_lossy());
        quote! {
            pub type #ident = ::#lib_ident::#ident;
        }
    } else {
        let variants = spec.cases.iter().map(|c| {
            let name = match c {
                stellar_xdr::ScSpecUdtUnionCaseV0::VoidV0(v) => v.name.clone(),
                stellar_xdr::ScSpecUdtUnionCaseV0::TupleV0(t) => t.name.clone(),
            };
            let v_ident = format_ident!("{}", name.to_utf8_string_lossy());
            match c {
                stellar_xdr::ScSpecUdtUnionCaseV0::VoidV0(_) => {
                    quote! { #v_ident }
                }
                stellar_xdr::ScSpecUdtUnionCaseV0::TupleV0(t) => {
                    let v_type = t.type_.iter().map(generate_type_ident);
                    quote! { #v_ident ( #(#v_type),* ) }
                }
            }
        });
        let contracttype_attr = contracttype_attr(opts.export);
        quote! {
            #contracttype_attr
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #ident { #(#variants,)* }
        }
    }
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec.
pub fn generate_enum(spec: &ScSpecUdtEnumV0) -> TokenStream {
    generate_enum_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec, with configurable options.
pub fn generate_enum_with_options(spec: &ScSpecUdtEnumV0, opts: &GenerateOptions) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_utf8_string().unwrap());
    if spec.lib.len() > 0 {
        let lib_ident = format_ident!("{}", spec.lib.to_utf8_string_lossy());
        quote! {
            pub type #ident = ::#lib_ident::#ident;
        }
    } else {
        let variants = spec.cases.iter().map(|c| {
            let v_ident = format_ident!("{}", c.name.to_utf8_string().unwrap());
            let v_value = Literal::u32_unsuffixed(c.value);
            quote! { #v_ident = #v_value }
        });
        let contracttype_attr = contracttype_attr(opts.export);
        quote! {
            #contracttype_attr
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #ident { #(#variants,)* }
        }
    }
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec, that is intended for use with errors.
pub fn generate_error_enum(spec: &ScSpecUdtErrorEnumV0) -> TokenStream {
    generate_error_enum_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single enum that mirrors the enum
/// spec, that is intended for use with errors, with configurable options.
pub fn generate_error_enum_with_options(
    spec: &ScSpecUdtErrorEnumV0,
    opts: &GenerateOptions,
) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_utf8_string().unwrap());
    if spec.lib.len() > 0 {
        let lib_ident = format_ident!("{}", spec.lib.to_utf8_string_lossy());
        quote! {
            pub type #ident = ::#lib_ident::#ident;
        }
    } else {
        let variants = spec.cases.iter().map(|c| {
            let v_ident = format_ident!("{}", c.name.to_utf8_string().unwrap());
            let v_value = Literal::u32_unsuffixed(c.value);
            quote! { #v_ident = #v_value }
        });
        let contracterror_attr = if opts.export {
            quote! { #[soroban_sdk::contracterror] }
        } else {
            quote! { #[soroban_sdk::contracterror(export = false)] }
        };
        quote! {
            #contracterror_attr
            #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub enum #ident { #(#variants,)* }
        }
    }
}

/// Constructs a token stream containing a single struct that mirrors the event
/// spec.
pub fn generate_event(spec: &ScSpecEventV0) -> TokenStream {
    generate_event_with_options(spec, &GenerateOptions::default())
}

/// Constructs a token stream containing a single struct that mirrors the event
/// spec, with configurable options.
pub fn generate_event_with_options(spec: &ScSpecEventV0, opts: &GenerateOptions) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_utf8_string().unwrap());

    if spec.lib.len() > 0 {
        let lib_ident = format_ident!("{}", spec.lib.to_utf8_string().unwrap());
        quote! {
            type #ident = ::#lib_ident::#ident;
        }
    } else {
        let topics = spec.prefix_topics.iter().map(|t| t.to_string());
        let fields = spec.params.iter().map(|p| {
            let p_ident = format_ident!("{}", p.name.to_utf8_string().unwrap());
            let p_type = generate_type_ident(&p.type_);
            match p.location {
                ScSpecEventParamLocationV0::TopicList => quote! {
                    #[topic]
                    pub #p_ident: #p_type
                },
                ScSpecEventParamLocationV0::Data => quote! {
                    pub #p_ident: #p_type
                },
            }
        });
        let export_attr = if opts.export {
            quote! {}
        } else {
            quote! { export = false, }
        };
        quote! {
            #[soroban_sdk::contractevent(#export_attr topics = [#(#topics,)*])]
            #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
            pub struct #ident { #(#fields,)* }
        }
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

pub fn generate_type_ident(spec: &ScSpecTypeDef) -> TokenStream {
    match spec {
        ScSpecTypeDef::Val => quote! { soroban_sdk::Val },
        ScSpecTypeDef::U64 => quote! { u64 },
        ScSpecTypeDef::I64 => quote! { i64 },
        ScSpecTypeDef::U32 => quote! { u32 },
        ScSpecTypeDef::I32 => quote! { i32 },
        ScSpecTypeDef::U128 => quote! { u128 },
        ScSpecTypeDef::I128 => quote! { i128 },
        ScSpecTypeDef::Bool => quote! { bool },
        ScSpecTypeDef::Symbol => quote! { soroban_sdk::Symbol },
        ScSpecTypeDef::Error => quote! { soroban_sdk::Error },
        ScSpecTypeDef::Bytes => quote! { soroban_sdk::Bytes },
        ScSpecTypeDef::Address => quote! { soroban_sdk::Address },
        ScSpecTypeDef::MuxedAddress => quote! { soroban_sdk::MuxedAddress },
        ScSpecTypeDef::String => quote! { soroban_sdk::String },
        ScSpecTypeDef::Option(o) => {
            let value_ident = generate_type_ident(&o.value_type);
            quote! { Option<#value_ident> }
        }
        ScSpecTypeDef::Result(r) => {
            let ok_ident = generate_type_ident(&r.ok_type);
            let error_ident = generate_type_ident(&r.error_type);
            quote! { Result<#ok_ident, #error_ident> }
        }
        ScSpecTypeDef::Vec(v) => {
            let element_ident = generate_type_ident(&v.element_type);
            quote! { soroban_sdk::Vec<#element_ident> }
        }
        ScSpecTypeDef::Map(m) => {
            let key_ident = generate_type_ident(&m.key_type);
            let value_ident = generate_type_ident(&m.value_type);
            quote! { soroban_sdk::Map<#key_ident, #value_ident> }
        }
        ScSpecTypeDef::Tuple(t) => {
            let type_idents = t.value_types.iter().map(generate_type_ident);
            quote! { (#(#type_idents,)*) }
        }
        ScSpecTypeDef::BytesN(b) => {
            let n = Literal::u32_unsuffixed(b.n);
            quote! { soroban_sdk::BytesN<#n> }
        }
        ScSpecTypeDef::Udt(u) => {
            let ident = format_ident!("{}", u.name.to_utf8_string().unwrap());
            quote! { #ident }
        }
        ScSpecTypeDef::Void => quote! { () },
        ScSpecTypeDef::Timepoint => quote! { soroban_sdk::Timepoint },
        ScSpecTypeDef::Duration => quote! { soroban_sdk::Duration },
        ScSpecTypeDef::U256 => quote! { soroban_sdk::U256 },
        ScSpecTypeDef::I256 => quote! { soroban_sdk::I256 },
    }
}

#[cfg(test)]
mod test {
    use crate::ToFormattedString;

    use super::generate_event;
    use quote::quote;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{
        ScSpecEventDataFormat, ScSpecEventParamLocationV0, ScSpecEventParamV0, ScSpecEventV0,
        ScSpecTypeDef, ScSymbol,
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
        });
        let expect = quote! {
            #[soroban_sdk::contractevent(topics = [])]
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
        });
        let expect = quote! {
            #[soroban_sdk::contractevent(topics = ["my_event"])]
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
}
