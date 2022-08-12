use proc_macro2::TokenStream;
use stellar_xdr::ScSpecFunctionV0;

/// Constructs a token stream containing a single zero-sized struct, and a
/// corresponding implementation, that provides a client for cross-contract
/// calls to a contract that implements the function spec.
#[allow(dead_code)]
pub fn gen_client(
    _name: &str,
    _impl_trait: Option<&str>,
    _fn_specs: &[&ScSpecFunctionV0],
) -> TokenStream {
    todo!()
}
