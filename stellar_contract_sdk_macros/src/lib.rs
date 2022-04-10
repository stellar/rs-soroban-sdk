extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, ItemFn, Pat, PatType,
    ReturnType, Type,
};

#[proc_macro_attribute]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let sig = &func.sig;
    let ident = &sig.ident;
    let args = &sig.inputs;
    let ret = &sig.output;

    let wrap_ident = format_ident!("__cf_{}", ident);
    let wrap_args = args.iter().map(|f| {
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
    // TODO: Replace the manual punctuation with quote! #_,* syntax.
    let mut wrap_args_punctuated: Punctuated<FnArg, Comma> = Punctuated::new();
    wrap_args.for_each(|f| wrap_args_punctuated.push(f));

    let wrap_ret = match ret {
        ReturnType::Default => ret.clone(),
        ReturnType::Type(ra, _) => ReturnType::Type(
            *ra,
            Box::new(Type::Verbatim(TokenStream::from(quote! {Val}).into()).clone()),
        ),
    };

    let param_idents = args.iter().map(|f| {
        if let &FnArg::Typed(pat_type) = &f {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let i = &pat_ident.ident;
                // TODO: Add `.intro()` functions to types that or_abort().
                return quote! { #i.try_into().or_abort() };
            }
        }
        panic!("This macro only accepts functions without a receiver.")
    });
    // TODO: Replace the manual punctuation with quote! #_,* syntax.
    let mut params_punctuated: Punctuated<TokenStream2, Comma> = Punctuated::new();
    param_idents.for_each(|i| params_punctuated.push(i.into()));

    // TODO: Don't include the Val::from for () return types.
    let ts: TokenStream = quote! {
        #func
        #[no_mangle]
        fn #wrap_ident(#wrap_args_punctuated) #wrap_ret {
            return Val::from(#ident(#params_punctuated));
        }
    }
    .into();
    // TODO: Remove before merge.
    println!("{}", ts);
    ts
}
