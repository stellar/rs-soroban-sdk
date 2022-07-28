use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::{ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, WriteXdr};
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{And, Colon, Comma},
    Error, FnArg, Ident, Pat, PatIdent, PatType, ReturnType, Type, TypePath, TypeReference,
};

use crate::map_type::map_type;

#[allow(clippy::too_many_lines)]
pub fn derive_fn(
    call: &TokenStream2,
    ident: &Ident,
    inputs: &Punctuated<FnArg, Comma>,
    output: &ReturnType,
    feature: &Option<String>,
    trait_ident: &Option<&Ident>,
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
    let (spec_args, wrap_args, wrap_calls, invoke_args, invoke_idents): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .enumerate()
        .map(|(i, a)| match a {
            FnArg::Typed(pat_type) => {
                let spec = match map_type(&pat_type.ty) {
                    Ok(spec) => spec,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                };
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
                    ty: Box::new(Type::Verbatim(quote! { soroban_sdk::RawVal })),
                });
                let call = quote! {
                    <_ as soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::RawVal>>::try_from_val(
                        &env,
                        #ident
                    ).unwrap()
                };
                let invoke_arg = FnArg::Typed(PatType {
                    attrs: vec![],
                    pat: Box::new(Pat::Ident(PatIdent {
                        ident: ident.clone(),
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        subpat: None,
                    })),
                    colon_token: Colon::default(),
                    ty: Box::new(Type::Reference(TypeReference {
                        and_token: And::default(),
                        lifetime: None,
                        mutability: None,
                        elem: pat_type.ty.clone(),
                    })),
                });
                let invoke_call = quote! { #ident };
                (spec, arg, call, invoke_arg, invoke_call)
            }
            FnArg::Receiver(_) => {
                errors.push(Error::new(a.span(), "self argument not supported"));
                (ScSpecTypeDef::I32, a.clone(), quote! {}, a.clone(), quote! {})
            }
        })
        .multiunzip();

    // Prepare the output.
    let spec_result = match output {
        ReturnType::Type(_, ty) => vec![match map_type(ty) {
            Ok(spec) => spec,
            Err(e) => {
                errors.push(e);
                ScSpecTypeDef::I32
            }
        }],
        ReturnType::Default => vec![],
    };

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return Err(quote! { #(#compile_errors)* });
    }

    // Generated code parameters.
    let wrap_export_name = format!("{}", ident);
    let pub_mod_ident = format_ident!("{}", ident);
    let hidden_mod_ident = format_ident!("__{}", ident);
    let deprecated_note = format!(
        "not intended for use, use {}::invoke instead",
        &pub_mod_ident
    );
    let env_call = if env_input.is_some() {
        quote! { env.clone(), }
    } else {
        quote! {}
    };
    let export_name = if let Some(cfg_feature) = feature {
        quote! { #[cfg_attr(feature = #cfg_feature, export_name = #wrap_export_name)] }
    } else {
        quote! { #[export_name = #wrap_export_name] }
    };
    let slice_args: Vec<TokenStream2> = (0..wrap_args.len()).map(|n| quote! { args[#n] }).collect();
    let use_trait = if let Some(t) = trait_ident {
        quote! { use super::#t }
    } else {
        quote! {}
    };

    // Generated code spec.
    let spec_entry = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        name: wrap_export_name.clone().try_into().unwrap(),
        input_types: spec_args.try_into().unwrap(),
        output_types: spec_result.try_into().unwrap(),
    });
    let spec_xdr = spec_entry.to_xdr().unwrap();
    let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
    let spec_xdr_len = spec_xdr.len();
    let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());
    let link_section = if let Some(cfg_feature) = feature {
        quote! { #[cfg_attr(all(target_family = "wasm", feature = #cfg_feature), link_section = "contractspecv0")] }
    } else {
        quote! { #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")] }
    };

    // Generated code.
    Ok(quote! {
        #[doc(hidden)]
        #link_section
        pub static #spec_ident: [u8; #spec_xdr_len] = *#spec_xdr_lit;

        #[doc(hidden)]
        #[deprecated(note = #deprecated_note)]
        pub mod #hidden_mod_ident {
            use super::*;

            #[deprecated(note = #deprecated_note)]
            #export_name
            pub fn invoke_raw(env: soroban_sdk::Env, #(#wrap_args),*) -> soroban_sdk::RawVal {
                #use_trait;
                <_ as soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal>>::into_val(
                    #[allow(deprecated)]
                    #call(
                        #env_call
                        #(#wrap_calls),*
                    ),
                    &env
                )
            }

            #[deprecated(note = #deprecated_note)]
            pub fn invoke_raw_slice(
                env: soroban_sdk::Env,
                args: &[soroban_sdk::RawVal],
            ) -> soroban_sdk::RawVal {
                #[allow(deprecated)]
                invoke_raw(env, #(#slice_args),*)
            }

            use super::*;
        }

        pub mod #pub_mod_ident {
            use super::*;

            pub fn invoke(
                e: &soroban_sdk::Env,
                contract_id: &soroban_sdk::Binary,
                #(#invoke_args),*
            ) #output {
                use soroban_sdk::{EnvVal, IntoVal, Symbol, Vec};
                let mut args: Vec<EnvVal> = Vec::new(e);
                #(args.push(#invoke_idents.clone().into_env_val(e));)*
                e.invoke_contract(contract_id.clone(), Symbol::from_str(#wrap_export_name), args)
            }

            #[cfg(feature = "testutils")]
            #[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
            pub fn invoke_xdr(
                e: &soroban_sdk::Env,
                contract_id: &soroban_sdk::Binary,
                #(#invoke_args),*
            ) #output {
                use soroban_sdk::TryIntoVal;
                e.invoke_contract_external_raw(
                    soroban_sdk::xdr::HostFunction::Call,
                    (contract_id, #wrap_export_name, #(#invoke_idents),*).try_into().unwrap()
                )
                .try_into_val(e)
                .unwrap()
            }
        }
    })
}

#[allow(clippy::too_many_lines)]
pub fn derive_contract_function_set<'a>(
    ty: &Box<Type>,
    methods: impl Iterator<Item = &'a syn::ImplItemMethod>,
) -> TokenStream2 {
    let (idents, wrap_idents): (Vec<_>, Vec<_>) = methods
        .map(|m| {
            let ident = format!("{}", m.sig.ident);
            let wrap_ident = format_ident!("__{}", m.sig.ident);
            (ident, wrap_ident)
        })
        .multiunzip();
    quote! {
        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::testutils::ContractFunctionSet for #ty {
            fn call(
                &self,
                func: &soroban_sdk::Symbol,
                env: soroban_sdk::Env,
                args: &[soroban_sdk::RawVal],
            ) -> Option<soroban_sdk::RawVal> {
                match func.to_str().as_ref() {
                    #(#idents => {
                        #[allow(deprecated)]
                        Some(#wrap_idents::invoke_raw_slice(env, args))
                    })*
                    _ => {
                        None
                    }
                }
            }
        }
    }
}
