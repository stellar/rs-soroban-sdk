use proc_macro2::TokenStream;
use quote::quote;
use stellar_xdr::{ScSpecFunctionV0, ScSpecUdtStructV0, ScSpecUdtUnionV0};

/// Constructs a token stream containing a single trait that has a function for
/// every function spec.
pub fn trait_for_function_specs(name: String, fn_specs: &[ScSpecFunctionV0]) -> TokenStream {
    quote! {}
}

/// Constructs a token stream containing a single zero-sized struct, and a
/// corresponding implementation, that provides a client for cross-contract
/// calls to a contract that implements the function spec.
pub fn client_impl_for_function_specs(
    name: String,
    impl_trait: Option<String>,
    fn_specs: &[ScSpecFunctionV0],
) -> TokenStream {
    quote! {}
}

/// Constructs a token stream containing a single struct that mirrors the type
/// spec.
pub fn struct_for_spec(s: ScSpecUdtStructV0) -> TokenStream {
    quote! {}
}

/// Constructs a token stream containing a single enum that mirrors the type
/// spec.
pub fn enum_for_spec(s: ScSpecUdtUnionV0) -> TokenStream {
    quote! {}
}

// TODO: Create a macro in soroban-sdk-macros that uses the above functions to
// render a complete contract specification.
//
// Something in the form:
//
// ```
// mod increment {
//   contractuse!(client = true, spec = "...");
// }
// ```
//
// A future version will also support a `wasm` option for extracting the spec
// from a wasm file.
