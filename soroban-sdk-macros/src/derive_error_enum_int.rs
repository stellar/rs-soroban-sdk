use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{ScSpecEntry, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, StringM, WriteXdr};
use syn::{spanned::Spanned, Attribute, DataEnum, Error, ExprLit, Ident, Lit, Path};

use crate::{doc::docs_from_attrs, DEFAULT_XDR_RW_LIMITS};

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
    let (spec_cases, try_froms, intos): (Vec<_>, Vec<_>, Vec<_>) = variants
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
                doc: docs_from_attrs(&v.attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into =
                quote! { #enum_ident::#ident => #path::Error::from_contract_error(#discriminant) };
            (spec_case, try_from, into)
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
            doc: docs_from_attrs(attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: enum_ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_TYPE_{}", enum_ident.to_string().to_uppercase());
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #enum_ident::spec_xdr();

            impl #enum_ident {
                pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                    *#spec_xdr_lit
                }
            }
        })
    } else {
        None
    };

    // Output.
    quote! {
        #spec_gen

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
                match val {
                    #(#intos,)*
                }
            }
        }

        impl From<&#enum_ident> for #path::Error {
            #[inline(always)]
            fn from(val: &#enum_ident) -> #path::Error {
                <_ as From<#enum_ident>>::from(*val)
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
    }
}
