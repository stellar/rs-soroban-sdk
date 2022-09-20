use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::ScSpecUdtEnumV0;
use syn::{spanned::Spanned, DataEnum, Error, ExprLit, Ident, Lit};

use stellar_xdr::{ScSpecEntry, ScSpecUdtEnumCaseV0, VecM, WriteXdr};

// TODO: Add conversions to/from ScVal types.

pub fn derive_type_enum_int(
    enum_ident: &Ident,
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
            let spec_case = ScSpecUdtEnumCaseV0 {
                name: name.try_into().unwrap_or_else(|_| VecM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into = quote! { #enum_ident::#ident => #discriminant.into_val(env) };
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
        let spec_entry = ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: enum_ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", enum_ident.to_string().to_uppercase());
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

        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::RawVal> for #enum_ident {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::RawVal) -> Result<Self, Self::Error> {
                use soroban_sdk::TryIntoVal;
                let discriminant: u32 = val.try_into_val(env)?;
                Ok(match discriminant {
                    #(#try_froms,)*
                    _ => Err(soroban_sdk::ConversionError{})?,
                })
            }
        }

        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #enum_ident> for soroban_sdk::RawVal {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#enum_ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for #enum_ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                match &self {
                    #(#intos,)*
                }
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for &#enum_ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                match self {
                    #(#intos,)*
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScVal) -> Result<Self, Self::Error> {
                let discriminant: u32 = val.try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                Ok(match discriminant {
                    #(#try_froms,)*
                    _ => Err(soroban_sdk::xdr::Error::Invalid)?,
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #enum_ident> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#enum_ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVal> for &#enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, Self::Error> {
                Ok((*self as u32).into())
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVal> for #enum_ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, Self::Error> {
                Ok((self as u32).into())
            }
        }
    }
}
