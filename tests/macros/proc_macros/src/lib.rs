/// A collection of proc-macros used by the test_macros test vector to validate that the
/// soroban-sdk macros are composable and compatible with a variety of other macros.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// An attribute macro that expects to be used on a function.
#[proc_macro_attribute]
pub fn parse_item_fn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    quote! { #item }.into()
}
