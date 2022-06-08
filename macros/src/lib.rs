extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType, ReturnType, Type};

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let sig = &func.sig;
    let ident = &sig.ident;
    let inputs = &sig.inputs;
    let output = &sig.output;

    // Prepare the spec parameters.
    let spec_ident = format_ident!("_SPEC_{}", ident.to_string().to_uppercase());

    // Prepare the wrap parameters.
    let wrap_ident = format_ident!("_{}", ident);
    let wrap_inputs_env_ident = inputs
        .first()
        .and_then(|f| {
            if let &FnArg::Typed(pat_type) = &f {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .expect("only accepts functions with first parameter as the Env type");
    let wrap_inputs = inputs.iter().enumerate().map(|(i, f)| {
        if let &FnArg::Typed(pat_type) = &f {
            if i == 0 {
                return f.clone();
            }
            return FnArg::Typed(PatType {
                attrs: pat_type.attrs.clone(),
                pat: pat_type.pat.clone(),
                colon_token: pat_type.colon_token,
                ty: Box::new(Type::Verbatim(
                    TokenStream::from(quote! {
                        stellar_contract_sdk::RawVal
                    })
                    .into(),
                )),
            });
        }
        panic!("only accepts functions without a self argument")
    });
    let wrap_call_inputs = inputs.iter().skip(1).map(|f| {
        if let &FnArg::Typed(pat_type) = &f {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let ident = &pat_ident.ident;
                let ts: TokenStream2 = quote! {
                    <_ as stellar_contract_sdk::TryFromVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::try_from_val(&#wrap_inputs_env_ident, #ident).unwrap()
                };
                return ts;
            }
        }
        panic!("only accepts functions without a self argument")
    });

    // Output.
    let ts: TokenStream = match output {
        ReturnType::Default => quote! {
            #func
            #[no_mangle]
            fn #wrap_ident(#(#wrap_inputs),*) -> stellar_contract_sdk::RawVal {
                #ident(#(#wrap_call_inputs),*);
                stellar_contract_sdk::RawVal::from_void()
            }
            #[cfg(target_family = "wasm")]
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; 10] = *b"abcdefghij";
        }
        .into(),
        ReturnType::Type(_, _) => quote! {
            #func
            #[no_mangle]
            fn #wrap_ident(#(#wrap_inputs),*) -> stellar_contract_sdk::RawVal {
                <_ as stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::into_val(
                    #ident(
                        #wrap_inputs_env_ident.clone(),
                        #(#wrap_call_inputs),*
                    ),
                    &#wrap_inputs_env_ident
                )
            }
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; 10] = *b"abcdefghij";
        }
        .into(),
    };
    ts
}
