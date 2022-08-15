use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use soroban_env_host::xdr::ScSpecFunctionV0;

use crate::types::generate_type_ident;

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
pub fn generate_trait(name: &str, specs: &[&ScSpecFunctionV0]) -> TokenStream {
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
