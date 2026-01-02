use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::ScSpecFunctionV0;

use super::types::generate_type_ident;

// IMPORTANT: The "docs" fields of spec entries are not output in Rust token
// streams as rustdocs, because rustdocs can contain Rust code, and that code
// will be executed. Generated code may be generated from untrusted Wasm
// containing untrusted spec docs.

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
pub fn generate_trait(name: &str, specs: &[&ScSpecFunctionV0]) -> TokenStream {
    let trait_ident = format_ident!("{}", name);
    let fns: Vec<_> = specs.iter().map(|s| generate_function(*s)).collect();
    quote! {
        pub trait #trait_ident { #(#fns;)* }
    }
}

/// Constructs a token stream representing a single function definition based on the provided
/// function specification.
///
/// # Parameters
/// - `s`: A reference to a `ScSpecFunctionV0` containing the specification of the function to generate.
///
/// # Returns
/// A `TokenStream` containing the generated function definition.
pub fn generate_function(s: &ScSpecFunctionV0) -> TokenStream {
    let fn_ident = format_ident!("{}", s.name.to_utf8_string().unwrap());
    let fn_inputs = s.inputs.iter().map(|input| {
        let name = format_ident!("{}", input.name.to_utf8_string().unwrap());
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
}
