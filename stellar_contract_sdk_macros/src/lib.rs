extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, ReturnType, Type};

#[proc_macro_attribute]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let sig = &func.sig;
    let ident = &sig.ident;
    let inputs = &sig.inputs;
    let output = &sig.output;

    // TODO: Figure out a shorter safe prefix. I tried a dollar-sign prefix, but
    // it didn't work on imports in tests. Ask @graydon.
    let wrap_ident = format_ident!("__cf_{}", ident);
    let wrap_inputs = inputs.iter().map(|f| {
        if let &FnArg::Typed(pat_type) = &f {
            return FnArg::Typed(PatType {
                attrs: pat_type.attrs.clone(),
                pat: pat_type.pat.clone(),
                colon_token: pat_type.colon_token,
                ty: Box::new(Type::Verbatim(TokenStream::from(quote! {Val}).into()).clone()),
            });
        }
        panic!("This macro only accepts functions without a receiver.")
    });

    let wrap_output = match output {
        ReturnType::Default => output.clone(),
        ReturnType::Type(ra, _) => ReturnType::Type(
            *ra,
            Box::new(Type::Verbatim(TokenStream::from(quote! {Val}).into()).clone()),
        ),
    };

    let param_idents = inputs.iter().map(|f| {
        if let &FnArg::Typed(pat_type) = &f {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let i = &pat_ident.ident;
                let ts: TokenStream2 = quote! { #i.try_into().or_abort() }.into();
                return ts;
            }
        }
        panic!("This macro only accepts functions without a receiver.")
    });

    // TODO: Don't include the Val::from for () return types.
    let ts: TokenStream = quote! {
        #func
        #[no_mangle]
        fn #wrap_ident(#(#wrap_inputs),*) #wrap_output {
            return Val::from(#ident(#(#param_idents),*));
        }
    }
    .into();
    // TODO: Remove before merge.
    // println!("{}", ts);
    ts
}
