use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{ScSpecEntry, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, StringM, WriteXdr};
use syn::{spanned::Spanned, Attribute, DataEnum, Error, ExprLit, Ident, Lit, Path};

use crate::{
    doc::docs_from_attrs,
    spec_marker::{encode_spec_marker, SPEC_MARKER_ERROR},
    DEFAULT_XDR_RW_LIMITS,
};

pub fn derive_type_error_enum_int(
    path: &Path,
    enum_ident: &Ident,
    attrs: &[Attribute],
    data: &DataEnum,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let variants = &data.variants;
    let (spec_cases, try_froms, into_errors, into_invoke_errors): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .map(|v| {
            let ident = &v.ident;
            let name = &ident.to_string();
            let discriminant: u32 = if let syn::Expr::Lit(ExprLit {
                lit: Lit::Int(ref lit_int),
                ..
            }) = v.discriminant.as_ref().unwrap().1
            {
                lit_int.base10_parse().unwrap_or_else(|_| {
                    errors.push(Error::new(
                        lit_int.span(),
                        "unsupported discriminant value on enum variant, must be parseable as u32",
                    ));
                    0
                })
            } else {
                errors.push(Error::new(
                    v.discriminant.as_ref().unwrap().1.span(),
                    "unsupported discriminant value on enum variant",
                ));
                0
            };
            let spec_case = ScSpecUdtErrorEnumCaseV0 {
                doc: docs_from_attrs(&v.attrs),
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into_error =
                quote! { #enum_ident::#ident => #path::Error::from_contract_error(#discriminant) };
            let into_invoke_error =
                quote! { #enum_ident::#ident => #path::InvokeError::Contract(#discriminant) };
            (spec_case, try_from, into_error, into_invoke_error)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
            doc: docs_from_attrs(attrs),
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: enum_ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_TYPE_{}", enum_ident.to_string().to_uppercase());
        // Create a marker that identifies this spec entry. The marker is in the regular data
        // section (subject to DCE) while the spec is in contractspecv0. A post-build tool will
        // find markers in the data section and keep only the corresponding specs in contractspecv0.
        // The marker is XDR-encoded: discriminant (4 bytes) + lib string + name string.
        let marker_ident = format_ident!(
            "__SPEC_XDR_MARKER_{}",
            enum_ident.to_string().to_uppercase()
        );
        let lib_str = lib.as_deref().unwrap_or_default();
        let type_name = enum_ident.to_string();
        let marker_xdr = encode_spec_marker(SPEC_MARKER_ERROR, lib_str, &type_name);
        let marker_len = marker_xdr.len();
        let marker_lit = proc_macro2::Literal::byte_string(&marker_xdr);
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #enum_ident::spec_xdr();

            impl #enum_ident {
                pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                    *#spec_xdr_lit
                }

                #[doc(hidden)]
                #[inline(always)]
                fn __include_spec() {
                    #[cfg(target_family = "wasm")]
                    {
                        // Marker in regular data section (subject to DCE).
                        // A post-build tool will find these markers and keep only
                        // the corresponding specs in contractspecv0.
                        static #marker_ident: [u8; #marker_len] = *#marker_lit;
                        let _ = unsafe { ::core::ptr::read_volatile(#marker_ident.as_ptr()) };
                    }
                }
            }
        })
    } else {
        None
    };

    // IncludeSpec implementation - only generated when spec is true.
    // Types with export=false should not be used at external boundaries.
    let include_spec_impl = if spec {
        Some(quote! {
            impl #path::IncludeSpec for #enum_ident {
                #[doc(hidden)]
                #[inline(always)]
                fn __include_spec() {
                    <#enum_ident>::__include_spec();
                }
            }
        })
    } else {
        None
    };

    // Output.
    quote! {
        #spec_gen

        #include_spec_impl

        impl TryFrom<#path::Error> for #enum_ident {
            type Error = #path::Error;
            #[inline(always)]
            fn try_from(error: #path::Error) -> Result<Self, #path::Error> {
                if error.is_type(#path::xdr::ScErrorType::Contract) {
                    let discriminant = error.get_code();
                    Ok(match discriminant {
                        #(#try_froms,)*
                        _ => return Err(error),
                    })
                } else {
                    Err(error)
                }
            }
        }

        impl TryFrom<&#path::Error> for #enum_ident {
            type Error = #path::Error;
            #[inline(always)]
            fn try_from(error: &#path::Error) -> Result<Self, #path::Error> {
                <_ as TryFrom<#path::Error>>::try_from(*error)
            }
        }

        impl From<#enum_ident> for #path::Error {
            #[inline(always)]
            fn from(val: #enum_ident) -> #path::Error {
                <_ as From<&#enum_ident>>::from(&val)
            }
        }

        impl From<&#enum_ident> for #path::Error {
            #[inline(always)]
            fn from(val: &#enum_ident) -> #path::Error {
                match val {
                    #(#into_errors,)*
                }
            }
        }

        impl TryFrom<#path::InvokeError> for #enum_ident {
            type Error = #path::InvokeError;
            #[inline(always)]
            fn try_from(error: #path::InvokeError) -> Result<Self, #path::InvokeError> {
                match error {
                    #path::InvokeError::Abort => Err(error),
                    #path::InvokeError::Contract(code) => Ok(match code {
                        #(#try_froms,)*
                        _ => return Err(error),
                    }),
                }
            }
        }

        impl TryFrom<&#path::InvokeError> for #enum_ident {
            type Error = #path::InvokeError;
            #[inline(always)]
            fn try_from(error: &#path::InvokeError) -> Result<Self, #path::InvokeError> {
                <_ as TryFrom<#path::InvokeError>>::try_from(*error)
            }
        }

        impl From<#enum_ident> for #path::InvokeError {
            #[inline(always)]
            fn from(val: #enum_ident) -> #path::InvokeError {
                <_ as From<&#enum_ident>>::from(&val)
            }
        }

        impl From<&#enum_ident> for #path::InvokeError {
            #[inline(always)]
            fn from(val: &#enum_ident) -> #path::InvokeError {
                match val {
                    #(#into_invoke_errors,)*
                }
            }
        }

        impl #path::TryFromVal<#path::Env, #path::Val> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::TryIntoVal;
                let error: #path::Error = val.try_into_val(env)?;
                error.try_into().map_err(|_| #path::ConversionError)
            }
        }
        impl #path::TryFromVal<#path::Env, #enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#enum_ident) -> Result<Self, #path::ConversionError> {
                let error: #path::Error = val.into();
                Ok(error.into())
            }
        }

        impl #path::TryFromVal<#path::Env, &#enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &&#enum_ident) -> Result<Self, #path::ConversionError> {
                <_ as #path::TryFromVal<#path::Env, #enum_ident>>::try_from_val(env, *val)
            }
        }
    }
}
