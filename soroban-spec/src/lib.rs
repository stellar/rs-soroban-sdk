use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stellar_xdr::{ScSpecEntry, ScSpecFunctionV0, ScSpecUdtStructV0, ScSpecUdtUnionV0};

pub fn contract_for_specs(name: &str, specs: &[ScSpecEntry]) -> TokenStream {
    let mod_ident = format_ident!("{}", name);
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
    let trait_ = trait_for_function_specs(
        name,
        &spec_fns,
    );
    quote! {
        mod #mod_ident {
            #trait_
        }
    }
}

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
pub fn trait_for_function_specs(name: &str, specs: &[&ScSpecFunctionV0]) -> TokenStream {
    let trait_ident = format_ident!("{}", name);
    let fns: Vec<_> = specs
        .iter()
        .map(|s| {
            let fn_ident = format_ident!("{}", s.name.to_string().unwrap());
            quote! {
                fn #fn_ident() -> () {
                    todo!()
                }
            }
        })
        .collect();
    quote! {
        pub trait #trait_ident {
            #(
                #fns,
            )*
        }
    }
}

/// Constructs a token stream containing a single zero-sized struct, and a
/// corresponding implementation, that provides a client for cross-contract
/// calls to a contract that implements the function spec.
pub fn client_impl_for_function_specs(
    name: &str,
    impl_trait: Option<&str>,
    fn_specs: &[&ScSpecFunctionV0],
) -> TokenStream {
    quote! {}
}

/// Constructs a token stream containing a single struct that mirrors the type
/// spec.
pub fn struct_for_spec(spec: &ScSpecUdtStructV0) -> TokenStream {
    quote! {}
}

/// Constructs a token stream containing a single enum that mirrors the type
/// spec.
pub fn enum_for_spec(spec: ScSpecUdtUnionV0) -> TokenStream {
    quote! {}
}

// TODO: Create a macro in soroban-sdk-macros that uses the above functions to
// render a complete contract specification.
//
// Something in the form:
//
// ```
// contractuse!(mod = "increment", client = true, spec = "...");
// ```
//
// A future version will also support a `wasm` option for extracting the spec
// from a wasm file.
