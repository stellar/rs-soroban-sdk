use crate::{attribute::pass_through_attr_to_gen_code, map_type::map_type};
use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use sha2::{Digest, Sha256};
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Comma},
    Attribute, Error, FnArg, Ident, Pat, PatIdent, PatType, Path, Type, TypePath, TypeReference,
};

#[allow(clippy::too_many_arguments)]
pub fn derive_pub_fn(
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
            let mut is_ref = false;
            let mut ty = &*pat_type.ty;
            if let Type::Reference(TypeReference { elem, .. }) = ty {
                is_ref = true;
                ty = elem;
            }
            if let Type::Path(TypePath {
                path: syn::Path { segments, .. },
                ..
            }) = ty
            {
                if segments.last().map_or(false, |s| s.ident == "Env") {
                    Some(is_ref)
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
    let (wrap_args, passthrough_calls, wrap_calls): (Vec<_>, Vec<_>, Vec<_>) = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .enumerate()
        .map(|(i, a)| match a {
            FnArg::Typed(pat_ty) => {
                // If fn is a __check_auth implementation, allow the first argument,
                // signature_payload of type Bytes (32 size), to be a Hash.
                let allow_hash = ident == "__check_auth" && i == 0;

                // Error if the type of the fn is not mappable.
                if let Err(e) = map_type(&pat_ty.ty, allow_hash) {
                    errors.push(e);
                }

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
                    ty: Box::new(Type::Verbatim(quote! { #crate_path::Val })),
                });
                let passthrough_call = quote! { #ident };
                let call = quote! {
                    <_ as #crate_path::unwrap::UnwrapOptimized>::unwrap_optimized(
                        <_ as #crate_path::TryFromValForContractFn<#crate_path::Env, #crate_path::Val>>::try_from_val_for_contract_fn(
                            &env,
                            &#ident
                        )
                    )
                };
                (arg, passthrough_call, call)
            }
            FnArg::Receiver(_) => {
                errors.push(Error::new(a.span(), "self argument not supported"));
                (a.clone(), quote! {}, quote! {})
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
    let env_call = if let Some(is_ref) = env_input {
        if is_ref {
            quote! { &env, }
        } else {
            quote! { env.clone(), }
        }
    } else {
        quote! {}
    };
    let slice_args: Vec<TokenStream2> = (0..wrap_args.len()).map(|n| quote! { args[#n] }).collect();
    let arg_count = slice_args.len();
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

    let testutils_only_code = if cfg!(feature = "testutils") {
        Some(quote! {
            #[deprecated(note = #deprecated_note)]
            pub fn invoke_raw_slice(
                env: #crate_path::Env,
                args: &[#crate_path::Val],
            ) -> #crate_path::Val {
                if args.len() != #arg_count {
                    panic!("invalid number of input arguments: {} expected, got {}", #arg_count, args.len());
                }
                #[allow(deprecated)]
                invoke_raw(env, #(#slice_args),*)
            }
        })
    } else {
        None
    };

    // Filter attributes to those that should be passed through to the generated code.
    let attrs = attrs
        .iter()
        .filter(|attr| pass_through_attr_to_gen_code(attr))
        .collect::<Vec<_>>();

    // Generated code.
    Ok(quote! {
        #[doc(hidden)]
        #(#attrs)*
        pub mod #hidden_mod_ident {
            use super::*;

            #[deprecated(note = #deprecated_note)]
            pub fn invoke_raw(env: #crate_path::Env, #(#wrap_args),*) -> #crate_path::Val {
                #use_trait;
                <_ as #crate_path::IntoVal<#crate_path::Env, #crate_path::Val>>::into_val(
                    #[allow(deprecated)]
                    &#call(
                        #env_call
                        #(#wrap_calls),*
                    ),
                    &env
                )
            }

            #testutils_only_code

            #[deprecated(note = #deprecated_note)]
            #[cfg_attr(target_family = "wasm", export_name = #wrap_export_name)]
            pub extern "C" fn invoke_raw_extern(#(#wrap_args),*) -> #crate_path::Val {
                #[allow(deprecated)]
                invoke_raw(
                    #crate_path::Env::default(),
                    #(#passthrough_calls),*
                )
            }

            use super::*;
        }
    })
}

#[allow(clippy::too_many_lines)]
pub fn derive_contract_function_registration_ctor<'a>(
    crate_path: &Path,
    ty: &Type,
    trait_ident: Option<&Ident>,
    methods: impl Iterator<Item = &'a syn::ImplItemFn>,
) -> TokenStream2 {
    if cfg!(not(feature = "testutils")) {
        return quote!();
    }

    let (idents, wrap_idents): (Vec<_>, Vec<_>) = methods
        .map(|m| {
            let ident = format!("{}", m.sig.ident);
            let wrap_ident = format_ident!("__{}", m.sig.ident);
            (ident, wrap_ident)
        })
        .multiunzip();

    let ty_str = quote!(#ty).to_string().replace(' ', "").replace(':', "_");
    let trait_str = quote!(#trait_ident).to_string();
    let methods_hash = format!("{:x}", Sha256::digest(idents.join(",").as_bytes()));
    let ctor_ident = format_ident!("__{ty_str}_{trait_str}_{methods_hash}_ctor");

    quote! {
        #[doc(hidden)]
        #[#crate_path::reexports_for_macros::ctor::ctor]
        #[allow(non_snake_case)]
        fn #ctor_ident() {
            #(
                <#ty as #crate_path::testutils::ContractFunctionRegister>::register(
                    #idents,
                    #[allow(deprecated)]
                    &#wrap_idents::invoke_raw_slice,
                );
            )*
        }
    }
}
