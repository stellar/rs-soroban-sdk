use itertools::MultiUnzip;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, FnArg, Lifetime, Type, TypePath, TypeReference};

use crate::syn_ext;

pub fn derive_args_type(ty: &str, name: &str) -> TokenStream {
    let ty_str = quote!(#ty).to_string();
    let args_doc =
        format!("{name} is a type for building arg lists for functions defined in {ty_str}.");
    let args_ident = format_ident!("{name}");
    quote! {
        #[doc = #args_doc]
        pub struct #args_ident;
    }
}

pub fn derive_args_impl(name: &str, fns: &[syn_ext::Fn]) -> TokenStream {
    // Map the traits methods to methods for the Args.
    let mut errors = Vec::<Error>::new();
    let fns: Vec<_> = fns
        .iter()
        .map(|f| {
            let fn_ident = &f.ident;

            // Check for the Env argument.
            let env_input = f.inputs.first().and_then(|a| match a {
                FnArg::Typed(pat_type) => {
                    let mut ty = &*pat_type.ty;
                    if let Type::Reference(TypeReference { elem, .. }) = ty {
                        ty = elem;
                    }
                    if let Type::Path(TypePath {
                        path: syn::Path { segments, .. },
                        ..
                    }) = ty
                    {
                        if segments.last().map_or(false, |s| s.ident == "Env") {
                            Some(())
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
            let fn_input_lifetime = Lifetime::new("'i", Span::call_site());
            let (fn_input_names, fn_input_types, fn_input_fn_args): (Vec<_>, Vec<_>, Vec<_>) = f
                .inputs
                .iter()
                .skip(if env_input.is_some() { 1 } else { 0 })
                .map(|t| {
                    let ident = match syn_ext::fn_arg_ident(t) {
                        Ok(ident) => ident,
                        Err(e) => {
                            errors.push(e);
                            format_ident!("_")
                        }
                    };
                    let ty = match syn_ext::fn_arg_ref_type(t, Some(&fn_input_lifetime)) {
                        Ok(ty) => Some(ty),
                        Err(e) => {
                            errors.push(e);
                            None
                        }
                    };
                    (
                        ident,
                        ty,
                        syn_ext::fn_arg_make_ref(t, Some(&fn_input_lifetime)),
                    )
                })
                .multiunzip();

            quote! {
                #[inline(always)]
                #[allow(clippy::unused_unit)]
                pub fn #fn_ident<#fn_input_lifetime>(#(#fn_input_fn_args),*)
                    -> (#(#fn_input_types,)*)
                {
                    (#(#fn_input_names,)*)
                }
            }
        })
        .collect();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Render the Client.
    let args_ident = format_ident!("{}", name);
    quote! {
        impl #args_ident {
            #(#fns)*
        }
    }
}
