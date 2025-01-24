use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSymbol, StringM, VecM,
    WriteXdr, SCSYMBOL_LIMIT,
};
use syn::TypeReference;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Error, FnArg, Ident, Pat,
    ReturnType, Type, TypePath,
};

use crate::attribute::pass_through_attr_to_gen_code;
use crate::{doc::docs_from_attrs, map_type::map_type, DEFAULT_XDR_RW_LIMITS};

#[allow(clippy::too_many_arguments)]
pub fn derive_fn_spec(
    ty: &Type,
    ident: &Ident,
    attrs: &[Attribute],
    inputs: &Punctuated<FnArg, Comma>,
    output: &ReturnType,
    export: bool,
) -> Result<TokenStream2, TokenStream2> {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    // Prepare the env input.
    let env_input = inputs.first().and_then(|a| match a {
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

    // Prepare the argument inputs.
    let spec_args: Vec<_> = inputs
        .iter()
        .skip(if env_input.is_some() { 1 } else { 0 })
        .enumerate()
        .map(|(i, a)| match a {
            FnArg::Typed(pat_type) => {
                let name = if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                    pat_ident.ident.to_string()
                } else {
                    errors.push(Error::new(a.span(), "argument not supported"));
                    "".to_string()
                };

                // If fn is a __check_auth implementation, allow the first argument,
                // signature_payload of type Bytes (32 size), to be a Hash.
                let allow_hash = ident == "__check_auth" && i == 0;

                match map_type(&pat_type.ty, allow_hash) {
                    Ok(type_) => {
                        let name = name.try_into().unwrap_or_else(|_| {
                            const MAX: u32 = 30;
                            errors.push(Error::new(
                                a.span(),
                                format!("argument name too long, max length {} characters", MAX),
                            ));
                            StringM::<MAX>::default()
                        });
                        ScSpecFunctionInputV0 {
                            doc: "".try_into().unwrap(),
                            name,
                            type_,
                        }
                    }
                    Err(e) => {
                        errors.push(e);
                        ScSpecFunctionInputV0 {
                            doc: "".try_into().unwrap(),
                            name: "arg".try_into().unwrap(),
                            type_: ScSpecTypeDef::I32,
                        }
                    }
                }
            }
            FnArg::Receiver(_) => {
                errors.push(Error::new(a.span(), "self argument not supported"));
                ScSpecFunctionInputV0 {
                    doc: "".try_into().unwrap(),
                    name: "".try_into().unwrap(),
                    type_: ScSpecTypeDef::I32,
                }
            }
        })
        .collect();

    // Prepare the output.
    let spec_result = match output {
        ReturnType::Type(_, ty) => vec![match map_type(ty, true) {
            Ok(spec) => spec,
            Err(e) => {
                errors.push(e);
                ScSpecTypeDef::I32
            }
        }],
        ReturnType::Default => vec![],
    };

    // Generated code spec.
    let name = &format!("{}", ident);
    let spec_entry = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: docs_from_attrs(attrs),
        name: name.try_into().unwrap_or_else(|_| {
            errors.push(Error::new(
                ident.span(),
                format!(
                    "contract function name is too long: {}, max is {}",
                    name.len(),
                    SCSYMBOL_LIMIT,
                ),
            ));
            ScSymbol::default()
        }),
        inputs: spec_args.try_into().unwrap_or_else(|_| {
            const MAX: u32 = 10;
            errors.push(Error::new(
                inputs.iter().nth(MAX as usize).span(),
                format!(
                    "contract function has too many parameters, max count {} parameters",
                    MAX,
                ),
            ));
            VecM::<_, MAX>::default()
        }),
        outputs: spec_result.try_into().unwrap(),
    });
    let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
    let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
    let spec_xdr_len = spec_xdr.len();
    let spec_ident = format_ident!("__SPEC_XDR_FN_{}", ident.to_string().to_uppercase());
    let spec_fn_ident = format_ident!("spec_xdr_{}", ident.to_string());

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return Err(quote! { #(#compile_errors)* });
    }

    let export_attr = if export {
        Some(quote! { #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")] })
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
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        #(#attrs)*
        #export_attr
        pub static #spec_ident: [u8; #spec_xdr_len] = #ty::#spec_fn_ident();

        impl #ty {
            #[allow(non_snake_case)]
            #(#attrs)*
            pub const fn #spec_fn_ident() -> [u8; #spec_xdr_len] {
                *#spec_xdr_lit
            }
        }
    })
}
