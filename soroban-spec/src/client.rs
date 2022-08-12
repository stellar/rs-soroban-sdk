use proc_macro2::TokenStream;
use quote::quote;
use stellar_xdr::ScSpecFunctionV0;

/// Constructs a token stream containing a single zero-sized struct, and a
/// corresponding implementation, that provides a client for cross-contract
/// calls to a contract that implements the function spec.
pub fn gen_client(
    name: &str,
    impl_trait: Option<&str>,
    fn_specs: &[&ScSpecFunctionV0],
) -> TokenStream {
    quote! {}
}
