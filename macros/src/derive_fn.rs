use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr_with_std::{
    SpecEntry, SpecEntryFunction, SpecEntryFunctionV0, SpecTypeDef, WriteXdr,
};
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Comma},
    Error, FnArg, Ident, Pat, PatIdent, PatType, ReturnType, Type, TypePath,
};

use crate::map_type::map_type;

#[allow(clippy::too_many_lines)]
pub fn derive_fn(
    call: &TokenStream2,
    ident: &Ident,
    inputs: &Punctuated<FnArg, Comma>,
    output: &ReturnType,
) -> TokenStream2 {
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
    let (spec_args, wrap_args, wrap_calls): (Vec<_>, Vec<_>, Vec<_>) = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .enumerate()
        .map(|(i, a)| {
            match a {
                FnArg::Typed(pat_type) => {
                    let spec = match map_type(&pat_type.ty) {
                        Ok(spec) => spec,
                        Err(e) => {
                            errors.push(e);
                            SpecTypeDef::I32
                        }
                    };
                    let ident = format_ident!("arg_{}", i);
                    let arg = FnArg::Typed(PatType {
                        attrs: vec![],
                        pat: Box::new(Pat::Ident(PatIdent{ ident: ident.clone(), attrs: vec![], by_ref: None, mutability: None, subpat: None })),
                        colon_token: Colon::default(),
                        ty: Box::new(Type::Verbatim(quote! { stellar_contract_sdk::RawVal })),
                    });
                    let call = quote! {
                        <_ as stellar_contract_sdk::TryFromVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::try_from_val(
                            &env,
                            #ident
                        ).unwrap()
                    };
                    (spec, arg, call)
                }
                FnArg::Receiver(_) => {
                    errors.push(Error::new(
                        a.span(),
                        "self argument not supported",
                    ));
                    (SpecTypeDef::I32, a.clone(), quote! { })
                }
            }
        }).multiunzip();

    // Prepare the output.
    let spec_result = match output {
        ReturnType::Type(_, ty) => vec![match map_type(ty) {
            Ok(spec) => spec,
            Err(e) => {
                errors.push(e);
                SpecTypeDef::I32
            }
        }],
        ReturnType::Default => vec![],
    };

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code parameters.
    let wrap_export_name = format!("{}", ident);
    let wrap_ident = format_ident!("__{}", ident);
    let env_call = if env_input.is_some() {
        quote! { env.clone(), }
    } else {
        quote! {}
    };

    // Generated code spec.
    let spec_entry_fn = SpecEntryFunctionV0 {
        name: wrap_export_name.clone().try_into().unwrap(),
        input_types: spec_args.try_into().unwrap(),
        output_types: spec_result.try_into().unwrap(),
    };
    let spec_entry = SpecEntry::Function(SpecEntryFunction::V0(spec_entry_fn));
    let spec_xdr = spec_entry.to_xdr().unwrap();
    let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
    let spec_xdr_len = spec_xdr.len();
    let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());

    // Generated code.
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
        pub static #spec_ident: [u8; #spec_xdr_len] = *#spec_xdr_lit;

        #[export_name = #wrap_export_name]
        fn #wrap_ident(env: stellar_contract_sdk::Env, #(#wrap_args),*) -> stellar_contract_sdk::RawVal {
            <_ as stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::into_val(
                #call(
                    #env_call
                    #(#wrap_calls),*
                ),
                &env
            )
        }
    }
}
