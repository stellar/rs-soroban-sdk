use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::{SpecEntry, SpecEntryFunction, SpecEntryFunctionV0, SpecTypeDef, WriteXdr};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident, PatType,
    ReturnType, Type, TypePath,
};

use crate::map_type::map_type;

#[allow(clippy::too_many_lines)]
pub fn wrap_and_spec_fn(
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
        .map(|a| {
            match a {
                FnArg::Typed(pat_type) => {
                    let pat = pat_type.pat.clone();
                    let spec = map_type(&pat_type.ty);
                    let arg = FnArg::Typed(PatType {
                        attrs: Vec::new(),
                        pat: pat_type.pat.clone(),
                        colon_token: pat_type.colon_token,
                        ty: Box::new(Type::Verbatim(quote! { stellar_contract_sdk::RawVal })),
                    });
                    let call = quote! {
                        <_ as stellar_contract_sdk::TryFromVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::try_from_val(
                            &__e,
                            #pat
                        ).unwrap()
                    };
                    (spec, arg, call)
                }
                FnArg::Receiver(_) => {
                    errors.push(syn::Error::new(
                        a.span(),
                        "self argument not supported",
                    ));
                    (SpecTypeDef::I32, a.clone(), quote! { })
                }
            }
        }).multiunzip();

    // Prepare the output.
    let spec_result = match output {
        ReturnType::Type(_, ty) => vec![map_type(ty)],
        ReturnType::Default => vec![],
    };

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(syn::Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code parameters.
    let wrap_export_name = format!("{}", ident);
    let wrap_ident = format_ident!("__{}", ident);
    let env_call = if env_input.is_some() {
        quote! { __e.clone(), }
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
        fn #wrap_ident(__e: stellar_contract_sdk::Env, #(#wrap_args),*) -> stellar_contract_sdk::RawVal {
            <_ as stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::into_val(
                #call(
                    #env_call
                    #(#wrap_calls),*
                ),
                &__e
            )
        }
    }
}
