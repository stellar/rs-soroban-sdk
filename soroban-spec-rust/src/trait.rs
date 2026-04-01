use proc_macro2::TokenStream;
use quote::quote;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::ScSpecFunctionV0;

use super::syn_ext::str_to_ident;
use super::types::{generate_type_ident, GenerateError};

// IMPORTANT: The "docs" fields of spec entries are not output in Rust token
// streams as rustdocs, because rustdocs can contain Rust code, and that code
// will be executed. Generated code may be generated from untrusted Wasm
// containing untrusted spec docs.

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
pub fn generate_trait(
    name: &str,
    specs: &[&ScSpecFunctionV0],
) -> Result<TokenStream, GenerateError> {
    let trait_ident = str_to_ident(name)?;
    let fns = specs
        .iter()
        .map(|s| generate_function(s))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(quote! {
        pub trait #trait_ident { #(#fns;)* }
    })
}

/// Constructs a token stream representing a single function definition based on the provided
/// function specification.
///
/// # Parameters
/// - `s`: A reference to a `ScSpecFunctionV0` containing the specification of the function to generate.
///
/// # Returns
/// A `TokenStream` containing the generated function definition.
pub fn generate_function(s: &ScSpecFunctionV0) -> Result<TokenStream, GenerateError> {
    let fn_ident = str_to_ident(&s.name)?;
    let fn_inputs = s
        .inputs
        .iter()
        .map(|input| {
            let name = str_to_ident(&input.name)?;
            let type_ident = generate_type_ident(&input.type_)?;
            Ok(quote! { #name: #type_ident })
        })
        .collect::<Result<Vec<_>, GenerateError>>()?;
    let fn_output = s
        .outputs
        .to_option()
        .map(|t| generate_type_ident(&t))
        .transpose()?
        .map(|t| quote! { -> #t });
    Ok(quote! {
        fn #fn_ident(env: soroban_sdk::Env, #(#fn_inputs),*) #fn_output
    })
}
