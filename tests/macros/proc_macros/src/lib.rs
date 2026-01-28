/// A collection of proc-macros used by the test_macros test vector to validate that the
/// soroban-sdk macros are composable and compatible with a variety of other macros.
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemImpl};

/// An attribute macro that expects to be used on an impl.
#[proc_macro_attribute]
pub fn parse_item_impl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemImpl);
    quote! { #item }.into()
}

/// An attribute macro that expects to be used on a function.
#[proc_macro_attribute]
pub fn parse_item_fn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemFn);
    quote! { #item }.into()
}

/// An attribute macro that checks it is being used on a method of a type, not a free function.
/// It does this by injecting code that references `Self`, which is only valid inside an impl block.
#[proc_macro_attribute]
pub fn check_fn_is_item_fn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemFn);
    // Insert a statement that uses `Self` at the beginning of the function
    let check_stmt: syn::Stmt = syn::parse_quote! {
        let _ = core::any::type_name::<Self>();
    };
    item.block.stmts.insert(0, check_stmt);
    quote! { #item }.into()
}
