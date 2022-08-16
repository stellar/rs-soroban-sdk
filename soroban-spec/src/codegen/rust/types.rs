use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use soroban_env_host::xdr::{ScSpecTypeDef, ScSpecUdtStructV0, ScSpecUdtUnionV0};

/// Constructs a token stream containing a single struct that mirrors the struct
/// spec.
pub fn generate_struct(spec: &ScSpecUdtStructV0) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_string().unwrap());
    let fields = spec.fields.iter().map(|f| {
        let f_ident = format_ident!("{}", f.name.to_string().unwrap());
        let f_type = generate_type_ident(&f.type_);
        quote! { pub #f_ident: #f_type }
    });
    quote! {
        #[::soroban_sdk::contracttype]
        pub struct #ident { #(#fields,)* }
    }
}

/// Constructs a token stream containing a single enum that mirrors the union
/// spec.
pub fn generate_union(spec: &ScSpecUdtUnionV0) -> TokenStream {
    let ident = format_ident!("{}", spec.name.to_string().unwrap());
    let variants = spec.cases.iter().map(|c| {
        let v_ident = format_ident!("{}", c.name.to_string().unwrap());
        let v_type = c
            .type_
            .as_ref()
            .map(generate_type_ident)
            .map_or_else(|| quote! {}, |t| quote! { (#t) });
        quote! { #v_ident #v_type }
    });
    quote! {
        #[::soroban_sdk::contracttype]
        pub enum #ident { #(#variants,)* }
    }
}

pub fn generate_type_ident(spec: &ScSpecTypeDef) -> TokenStream {
    match spec {
        ScSpecTypeDef::U64 => quote! { u64 },
        ScSpecTypeDef::I64 => quote! { i64 },
        ScSpecTypeDef::U32 => quote! { u32 },
        ScSpecTypeDef::I32 => quote! { i32 },
        ScSpecTypeDef::Bool => quote! { bool },
        ScSpecTypeDef::Symbol => quote! { ::soroban_sdk::Symbol },
        ScSpecTypeDef::Bitset => quote! { ::soroban_sdk::Bitset },
        ScSpecTypeDef::Status => quote! { ::soroban_sdk::Status },
        ScSpecTypeDef::Bytes => quote! { ::soroban_sdk::Bytes },
        ScSpecTypeDef::BigInt => quote! { ::soroban_sdk::BigInt },
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
            quote! { ::soroban_sdk::Vec<#element_ident> }
        }
        ScSpecTypeDef::Map(m) => {
            let key_ident = generate_type_ident(&m.key_type);
            let value_ident = generate_type_ident(&m.value_type);
            quote! { ::soroban_sdk::Map<#key_ident, #value_ident> }
        }
        ScSpecTypeDef::Set(s) => {
            let element_ident = generate_type_ident(&s.element_type);
            quote! { ::soroban_sdk::Set<#element_ident> }
        }
        ScSpecTypeDef::Tuple(t) => {
            let type_idents = t.value_types.iter().map(generate_type_ident);
            quote! { (#(#type_idents,)*) }
        }
        ScSpecTypeDef::Udt(u) => {
            let ident = format_ident!("{}", u.name.to_string().unwrap());
            quote! { #ident }
        }
    }
}
