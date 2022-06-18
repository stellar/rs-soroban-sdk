use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, Ident, PatType,
    ReturnType, Type, TypePath,
};

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

    // Output.
    let wrap_export_name = format!("{}", ident);
    let wrap_ident = format_ident!("__{}", ident);
    let env_call = if env_input.is_some() {
        quote! { __e.clone(), }
    } else {
        quote! {}
    };
    let spec_ident = format_ident!("__SPEC_{}", ident.to_string().to_uppercase());
    let spec_args_str = format!(
        // TODO: Produce XDR instead.
        "[{}({}):{}]",
        wrap_export_name,
        spec_args.join(","),
        spec_result,
    );
    let spec_args_bytes = spec_args_str.as_bytes();
    let spec_args_literal = proc_macro2::Literal::byte_string(spec_args_bytes);
    let spec_args_literal_size = spec_args_bytes.len();
    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
        pub static #spec_ident: [u8; #spec_args_literal_size] = *#spec_args_literal;

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
