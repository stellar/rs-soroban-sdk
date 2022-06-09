extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident,
    ImplItem, ItemFn, ItemImpl, PatType, Type, TypePath, Visibility,
};

// TODO: Investigate how to make the multiple spec statics be joined into a
// variable length XDR array instead of being a stream of XDR values.

#[proc_macro_attribute]
#[allow(clippy::missing_panics_doc)]
pub fn contractfn(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let ident = &func.sig.ident;
    let call = quote! { #ident };
    let wrap_and_spec = wrap_and_spec(&call, ident, &func.sig.inputs);
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
            wrap_and_spec(&call, ident, &m.sig.inputs)
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
) -> TokenStream2 {
    // Collect errors as we go and emit them at the end.
    let mut errors = Vec::<Error>::new();

    // Prepare the spec parameters.
    let spec_ident = format_ident!("__SPEC_{}", ident.to_string().to_uppercase());
    let spec_inputs = format!(
        "{:?}",
        inputs
            .iter()
            .skip(1)
            .map(|f| {
                if let &FnArg::Typed(pat_type) = &f {
                    return match &*pat_type.ty {
                        Type::Path(p) => p.into_token_stream(),
                        _ => todo!(),
                    };
                }
                panic!("only accepts functions without a self argument")
            })
            .reduce(|a, b| quote! { #a #b })
    );
    let spec_inputs_literal = proc_macro2::Literal::byte_string(spec_inputs.as_bytes());
    let spec_inputs_literal_size = spec_inputs.len();

    // Prepare the env input.
    let env_input = inputs.first();
    if let Some(env_input) = env_input {
        match &env_input {
            FnArg::Typed(pat_type) => {
                let ty = &*pat_type.ty;
                if let Type::Path(TypePath {
                    path: syn::Path { segments, .. },
                    ..
                }) = ty
                {
                    if !segments.last().map_or(false, |s| s.ident == "Env") {
                        errors.push(syn::Error::new(
                            segments.span(),
                            "first argument must be of type Env",
                        ));
                    }
                } else {
                    errors.push(syn::Error::new(
                        ty.span(),
                        "first argument must be of type Env",
                    ));
                }
            }
            FnArg::Receiver(r) => {
                errors.push(syn::Error::new(
                    r.span(),
                    "self argument not supported first argument must be of type Env",
                ));
            }
        };
    }

    // Prepare the argument inputs.
    let (wrap_args, wrap_calls): (Vec<_>, Vec<_>) = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .map(|a| {
            if let &FnArg::Typed(pat_type) = &a {
                let pat = pat_type.pat.clone();
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
                (arg, call)
            } else {
                errors.push(syn::Error::new(
                    a.span(),
                    "argument not supported",
                ));
                (a.clone(), quote! { })
            }
        })
        .unzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(syn::Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Output.
    let wrap_export_name = format!("{}", ident);
    let wrap_ident = format_ident!("__{}", ident);
    quote! {
        #[export_name = #wrap_export_name]
        fn #wrap_ident(__e: stellar_contract_sdk::Env, #(#wrap_args),*) -> stellar_contract_sdk::RawVal {
            <_ as stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal>>::into_val(
                #call(
                    __e.clone(),
                    #(#wrap_calls),*
                ),
                &__e
            )
        }
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
        pub static #spec_ident: [u8; #spec_inputs_literal_size] = *#spec_inputs_literal;
    }
}
