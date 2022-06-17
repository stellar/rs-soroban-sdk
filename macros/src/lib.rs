extern crate proc_macro;

use itertools::MultiUnzip;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use stellar_xdr::{SpecEntry, SpecEntryFunction, SpecEntryFunctionV0};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident,
    ImplItem, ItemFn, ItemImpl, PatType, ReturnType, Type, TypePath, Visibility,
};

// TODO: Investigate how to make the multiple spec statics be joined into a
// variable length XDR array instead of being a stream of XDR values.

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = &func.sig.ident;
    let call = quote! { #ident };
    let wrap_and_spec = wrap_and_spec(&call, ident, &func.sig.inputs, &func.sig.output);
    quote! {
        #func
        #wrap_and_spec
    }
    .into()
}

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn contractimpl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let imp = parse_macro_input!(input as ItemImpl);
    let is_trait = imp.trait_.is_some();
    let ty = &imp.self_ty;
    let wrap_and_specs = imp
        .items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Method(m) => Some(m),
            _ => None,
        })
        .filter(|m| is_trait || matches!(m.vis, Visibility::Public(_)))
        .map(|m| {
            let ident = &m.sig.ident;
            let call = quote! { <#ty>::#ident };
            wrap_and_spec(&call, ident, &m.sig.inputs, &m.sig.output)
        });
    quote! {
        #imp
        #(#wrap_and_specs)*
    }
    .into()
}

#[allow(clippy::too_many_lines)]
fn wrap_and_spec(
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
                    let spec = pat_type.ty.to_token_stream().to_string(); // TODO: Map types to SCType for spec.
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
                    ("".to_string(), a.clone(), quote! { })
                }
            }
        }).multiunzip();

    // Prepare the output.
    let spec_result = match output {
        // TODO: Map types to SCType.
        ReturnType::Default => "()".to_string(),
        ReturnType::Type(_, ty) => ty.to_token_stream().to_string(),
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
        input_types: [].try_into().unwrap(),
        output_types: [].try_into().unwrap(),
    };
    let spec_entry = SpecEntry::Function(SpecEntryFunction::V0(spec_entry_fn));
    let spec_entry_xdr = spec_entry.to_xdr();
    let spec_entry_xdr_lit = proc_macro2::Literal::byte_string(spec_args_xdr.as_bytes());
    let spec_entry_xdr_len = spec_args_xdr.len();
    let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());

    // Generated code.
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
        pub static #spec_ident: [u8; #spec_entry_xdr_len] = *#spec_args_xdr_lit;

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
