use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{spanned::Spanned, Error, FnArg, Pat, TraitItemMethod, Type, TypePath};

pub fn derive_client(name: &str, methods: &[&TraitItemMethod]) -> TokenStream {
    // Map the traits methods to methods for the Client.
    let mut errors = Vec::<Error>::new();
    let methods: Vec<_> = methods.iter()
        .map(|m| {
            let fn_ident = &m.sig.ident;
            let fn_name = fn_ident.to_string();

            // Check for the Env argument.
            let env_input = m.sig.inputs.first().and_then(|a| match a {
                FnArg::Typed(pat_type) => {
                    let ty = &*pat_type.ty;
                    if let Type::Path(TypePath {
                        path: syn::Path { segments, .. },
                        ..
                    }) = ty
                    {
                        if segments.last().map_or(false, |s| s.ident == "Env") {
                            Some(a)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                FnArg::Receiver(_) => None,
            });

            // Map all remaining inputs.
            let (fn_input_names, fn_input_types): (Vec<_>, Vec<_>) = m.sig.inputs
                .iter()
                .skip(if env_input.is_some() { 1 } else { 0 })
                .enumerate().map(|(i, t)| {
                    match t {
                        FnArg::Typed(pat_type) => {
                            if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                                let ident = format_ident!("arg_{}", i);
                                let mut pat_type = pat_type.clone();
                                let mut pat_ident = pat_ident.clone();
                                pat_ident.ident = ident.clone();
                                pat_type.pat = Box::new(Pat::Ident(pat_ident));
                                (ident, FnArg::Typed(pat_type))
                            } else {
                                errors.push(Error::new(t.span(), "argument not supported"));
                                (format_ident!(""), t.clone())
                            }
                        }
                        _ => {
                            errors.push(Error::new(t.span(), "argument not supported"));
                            (format_ident!(""), t.clone())
                        }
                    }
                })
                .unzip();
            let fn_output = &m.sig.output;
            quote!{
                pub fn #fn_ident(env: &::soroban_sdk::Env, contract_id: &::soroban_sdk::BytesN<32>, #(#fn_input_types),*) #fn_output {
                    use ::soroban_sdk::IntoVal;
                    const FN_SYMBOL: ::soroban_sdk::Symbol = ::soroban_sdk::Symbol::from_str(#fn_name);
                    env.invoke_contract(contract_id, &FN_SYMBOL, ::soroban_sdk::vec![env, #(#fn_input_names.into_env_val(&env)),*])
                }
            }
        })
        .collect();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* }.into();
    }

    // Render the Client.
    let client_ident = format_ident!("{}", name);
    quote! {
        pub struct #client_ident;
        impl #client_ident { #(#methods)* }
    }
}
