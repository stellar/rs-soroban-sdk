use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{ScSpecUdtEnumV0, StringM};
use syn::{spanned::Spanned, Attribute, DataEnum, Error, ExprLit, Ident, Lit, Path, Visibility};

use stellar_xdr::{ScSpecEntry, ScSpecUdtEnumCaseV0, WriteXdr};

use crate::{doc::docs_from_attrs, DEFAULT_XDR_RW_LIMITS};

// TODO: Add conversions to/from ScVal types.

pub fn derive_type_enum_int(
    path: &Path,
    vis: &Visibility,
    enum_ident: &Ident,
    attrs: &[Attribute],
    data: &DataEnum,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let variants = &data.variants;
    let (spec_cases, try_froms, try_intos): (Vec<_>, Vec<_>, Vec<_>) = variants
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
            let spec_case = ScSpecUdtEnumCaseV0 {
                doc: docs_from_attrs(&v.attrs),
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let try_into = quote! { #enum_ident::#ident => #discriminant.into() };
            (spec_case, try_from, try_into)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            doc: docs_from_attrs(attrs),
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
    let mut output = quote! {
        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::Val> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::TryIntoVal;
                let discriminant: u32 = val.try_into_val(env)?;
                Ok(match discriminant {
                    #(#try_froms,)*
                    _ => Err(#path::ConversionError{})?,
                })
            }
        }

        impl #path::TryFromVal<#path::Env, #enum_ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#enum_ident) -> Result<Self, #path::ConversionError> {
                Ok(match val {
                    #(#try_intos,)*
                })
            }
        }
    };

    // Additional output when testutils are enabled.
    if cfg!(feature = "testutils") {
        let arbitrary_tokens =
            crate::arbitrary::derive_arbitrary_enum_int(path, vis, enum_ident, data);
        output.extend(quote! {
            impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVal) -> Result<Self, #path::xdr::Error> {
                    if let #path::xdr::ScVal::U32(discriminant) = val {
                        Ok(match *discriminant {
                            #(#try_froms,)*
                            _ => Err(#path::xdr::Error::Invalid)?,
                        })
                    } else {
                        Err(#path::xdr::Error::Invalid)
                    }
                }
            }

            impl TryInto<#path::xdr::ScVal> for &#enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_into(self) -> Result<#path::xdr::ScVal, #path::xdr::Error> {
                    Ok(match self {
                        #(#try_intos,)*
                    })
                }
            }

            impl TryInto<#path::xdr::ScVal> for #enum_ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_into(self) -> Result<#path::xdr::ScVal, #path::xdr::Error> {
                    Ok(match self {
                        #(#try_intos,)*
                    })
                }
            }

            #arbitrary_tokens
        });
    }
    output
}
