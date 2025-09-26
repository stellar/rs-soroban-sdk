#![feature(prelude_import)]
#[macro_use]
extern crate std;
/// A collection of proc-macros used by the test_macros test vector to validate that the
/// soroban-sdk macros are composable and compatible with a variety of other macros.
use proc_macro::TokenStream;
use quote::quote;
#[prelude_import]
use std::prelude::rust_2021::*;
use syn::{parse_macro_input, ItemFn};
/// An attribute macro that expects to be used on a function.
#[proc_macro_attribute]
pub fn parse_item_fn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item = match ::syn::parse::<ItemFn>(input) {
        ::syn::__private::Ok(data) => data,
        ::syn::__private::Err(err) => {
            return ::syn::__private::TokenStream::from(err.to_compile_error());
        }
    };
    {
        let mut _s = ::quote::__private::TokenStream::new();
        ::quote::ToTokens::to_tokens(&item, &mut _s);
        _s
    }
    .into()
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
