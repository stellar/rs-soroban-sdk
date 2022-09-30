use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stellar_xdr::ScSpecFunctionV0;

use super::types::generate_type_ident;

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
pub fn generate_trait(name: &str, specs: &[&ScSpecFunctionV0]) -> TokenStream {
    let trait_ident = format_ident!("{}", name);
    let fns: Vec<_> = specs
        .iter()
        .map(|s| {
            let fn_ident = format_ident!("{}", s.name.to_string().unwrap());
            let fn_inputs = s.inputs.iter().map(|input| {
                let name = format_ident!("{}", input.name.to_string().unwrap());
                let type_ident = generate_type_ident(&input.type_);
                quote! { #name: #type_ident }
            });
            let fn_output = s
                .outputs
                .to_option()
                .map(|t| generate_type_ident(&t))
                .map(|t| quote! { -> #t });
            quote! {
                fn #fn_ident(env: soroban_sdk::Env, #(#fn_inputs),*) #fn_output
            }
        })
        .collect();
    quote! {
        pub trait #trait_ident { #(#fns;)* }
    }
}
