use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use soroban_env_host::xdr::{
    ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecUdtStructV0, ScSpecUdtUnionV0,
};

pub fn generate(specs: &[ScSpecEntry], wasm: Option<&str>) -> TokenStream {
    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    for s in specs {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
        }
    }
    let client_attr = quote! { #[::soroban_sdk::contractclient] };
    let wasm_attr = wasm.map(|wasm| quote! { #[::soroban_sdk::contractwasm(wasm = #wasm)] });
    let trait_ = generate_trait("Contract", &spec_fns);
    let structs = spec_structs.iter().map(|s| generate_struct(s));
    let unions = spec_unions.iter().map(|s| generate_union(s));
    quote! {
        #client_attr
        #wasm_attr
        #trait_

        #(#structs)*

        #(#unions)*
    }
}

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
fn generate_trait(name: &str, specs: &[&ScSpecFunctionV0]) -> TokenStream {
    let trait_ident = format_ident!("{}", name);
    let fns: Vec<_> = specs
        .iter()
        .map(|s| {
            let fn_ident = format_ident!("{}", s.name.to_string().unwrap());
            let fn_inputs = s.input_types.iter().enumerate().map(|(i, t)| {
                let name = format_ident!("a{}", i);
                let type_ident = generate_type_ident(t);
                quote! { #name: #type_ident }
            });
            let fn_outputs = s.output_types.iter().map(generate_type_ident);
            quote! {
                fn #fn_ident(env: ::soroban_sdk::Env, #(#fn_inputs),*) -> (#(#fn_outputs),*)
            }
        })
        .collect();
    quote! {
        pub trait #trait_ident { #(#fns;)* }
    }
}

/// Constructs a token stream containing a single struct that mirrors the struct
/// spec.
fn generate_struct(spec: &ScSpecUdtStructV0) -> TokenStream {
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
fn generate_union(spec: &ScSpecUdtUnionV0) -> TokenStream {
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

fn generate_type_ident(spec: &ScSpecTypeDef) -> TokenStream {
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
            let type_idents = t.value_types.iter().map(|vt| generate_type_ident(&vt));
            quote! { (#(#type_idents,)*) }
        }
        ScSpecTypeDef::Udt(u) => {
            let ident = format_ident!("{}", u.name.to_string().unwrap());
            quote! { #ident }
        }
    }
}
