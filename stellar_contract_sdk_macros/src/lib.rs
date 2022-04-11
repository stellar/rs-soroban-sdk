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

    let wrap_link_name = format!("${}", ident);
    let wrap_ident = format_ident!("contractfn_{}", ident);
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
    let wrap_call_inputs = inputs.iter().map(|f| {
        if let &FnArg::Typed(pat_type) = &f {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let i = &pat_ident.ident;
                let ts: TokenStream2 = quote! { #i.try_into().or_abort() }.into();
                return ts;
            }
        }
        panic!("This macro only accepts functions without a receiver.")
    });

    let ts: TokenStream = match output {
        ReturnType::Default => quote! {
            #func
            #[no_mangle]
            #[link_name = #wrap_link_name]
            fn #wrap_ident(#(#wrap_inputs),*) -> Val {
                #ident(#(#wrap_call_inputs),*);
                Val::from_void()
            }
        }
        .into(),
        ReturnType::Type(_, _) => quote! {
            #func
            #[no_mangle]
            #[link_name = #wrap_link_name]
            fn #wrap_ident(#(#wrap_inputs),*) -> Val {
                Val::from(#ident(#(#wrap_call_inputs),*))
            }
        }
        .into(),
    };
    ts
}
