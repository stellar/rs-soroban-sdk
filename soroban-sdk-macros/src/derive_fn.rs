use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Comma},
    Attribute, Error, FnArg, Ident, Pat, PatIdent, PatType, Path, Type, TypePath,
};

#[allow(clippy::too_many_arguments)]
pub fn derive_fn(
    crate_path: &Path,
    call: &TokenStream2,
    ident: &Ident,
    attrs: &[Attribute],
    inputs: &Punctuated<FnArg, Comma>,
    trait_ident: Option<&Ident>,
    client_ident: &str,
) -> Result<TokenStream2, TokenStream2> {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    // Prepare the env input.
    let env_input = inputs.first().and_then(|a| match a {
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

    // Prepare the argument inputs.
    let (wrap_args, wrap_calls): (Vec<_>, Vec<_>) = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .enumerate()
        .map(|(i, a)| match a {
            FnArg::Typed(_) => {
                let ident = format_ident!("arg_{}", i);
                let arg = FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        ident: ident.clone(),
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        subpat: None,
                    })),
                    colon_token: Colon::default(),
                    ty: Box::new(Type::Verbatim(quote! { #crate_path::RawVal })),
                });
                let call = quote! {
                    <_ as #crate_path::unwrap::UnwrapOptimized>::unwrap_optimized(
                        <_ as #crate_path::TryFromVal<#crate_path::Env, #crate_path::RawVal>>::try_from_val(
                            &env,
                            &#ident
                        )
                    )
                };
                (arg, call)
            }
            FnArg::Receiver(_) => {
                errors.push(Error::new(a.span(), "self argument not supported"));
                (a.clone(), quote! {})
            }
        })
        .multiunzip();

    // Generated code parameters.
    let wrap_export_name = &format!("{}", ident);
    let hidden_mod_ident = format_ident!("__{}", ident);
    let deprecated_note = format!(
        "use `{}::new(&env, &contract_id).{}` instead",
        client_ident, &ident
    );
    let env_call = if env_input.is_some() {
        quote! { env.clone(), }
    } else {
        quote! {}
    };
    let slice_args: Vec<TokenStream2> = (0..wrap_args.len()).map(|n| quote! { args[#n] }).collect();
    let use_trait = if let Some(t) = trait_ident {
        quote! { use super::#t }
    } else {
        quote! {}
    };

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return Err(quote! { #(#compile_errors)* });
    }

    // Generated code.
    Ok(quote! {
        #[doc(hidden)]
        #(#attrs)*
        pub mod #hidden_mod_ident {
            use super::*;

            #[deprecated(note = #deprecated_note)]
            #[cfg_attr(target_family = "wasm", export_name = #wrap_export_name)]
            pub fn invoke_raw(env: #crate_path::Env, #(#wrap_args),*) -> #crate_path::RawVal {
                #use_trait;
                <_ as #crate_path::IntoVal<#crate_path::Env, #crate_path::RawVal>>::into_val(
                    #[allow(deprecated)]
                    &#call(
                        #env_call
                        #(#wrap_calls),*
                    ),
                    &env
                )
            }

            #[deprecated(note = #deprecated_note)]
            pub fn invoke_raw_slice(
                env: #crate_path::Env,
                args: &[#crate_path::RawVal],
            ) -> #crate_path::RawVal {
                #[allow(deprecated)]
                invoke_raw(env, #(#slice_args),*)
            }

            use super::*;
        }
    })
}

#[allow(clippy::too_many_lines)]
pub fn derive_contract_function_set<'a>(
    crate_path: &Path,
    ty: &Type,
    methods: impl Iterator<Item = &'a syn::ImplItemFn>,
) -> TokenStream2 {
    let (idents, wrap_idents, attrs): (Vec<_>, Vec<_>, Vec<_>) = methods
        .map(|m| {
            let ident = format!("{}", m.sig.ident);
            let wrap_ident = format_ident!("__{}", m.sig.ident);
            let attrs = m
                .attrs
                .iter()
                // Don't propogate doc comments into the match statement below.
                .filter(|a| !a.path().is_ident("doc"))
                .collect::<Vec<_>>();
            (ident, wrap_ident, attrs)
        })
        .multiunzip();
    quote! {
        #[cfg(any(test, feature = "testutils"))]
        impl #crate_path::testutils::ContractFunctionSet for #ty {
            fn call(
                &self,
                func: &str,
                env: #crate_path::Env,
                args: &[#crate_path::RawVal],
            ) -> Option<#crate_path::RawVal> {
                match func {
                    #(
                        #(#attrs)*
                        #idents => {
                            #[allow(deprecated)]
                            Some(#wrap_idents::invoke_raw_slice(env, args))
                        }
                    )*
                    _ => {
                        None
                    }
                }
            }
        }
    }
}
