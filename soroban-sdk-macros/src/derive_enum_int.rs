use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use soroban_env_common::Symbol;
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
    let (spec_cases, discriminant_consts, try_froms, intos): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .map(|v| {
            // TODO: Choose discriminant type based on repr type of enum.
            // TODO: Should we use variants explicit discriminant? Probably not.
            // Should have a separate derive for those types of enums that maps
            // to an integer type only.
            // TODO: Use attributes tagged on variant to control whether field is included.
            // TODO: Support multi-field enum variants.
            // TODO: Or, error on multi-field enum variants.
            // TODO: Handle field names longer than a symbol. Hash the name? Truncate the name?
            let ident = &v.ident;
            let name = &ident.to_string();
            if let Err(e) = Symbol::try_from_str(name) {
                errors.push(Error::new(ident.span(), format!("enum variant name {}", e)));
            }
            let discriminant: u32 = if let syn::Expr::Lit(ExprLit { lit: Lit::Int(ref lit_int), .. }) = v.discriminant.as_ref().unwrap().1 {
                lit_int.base10_parse().unwrap_or_else(|_| {
                errors.push(Error::new(lit_int.span(), "unsupported discriminant value on enum variant, must be parseable as u32"));
                0
            })
            } else {
                errors.push(Error::new(v.discriminant.as_ref().unwrap().1.span(), "unsupported discriminant value on enum variant"));
                0
            };
            let spec_case = ScSpecUdtEnumCaseV0 {
                name: name.try_into().unwrap_or_else(|_| VecM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into = quote! { #enum_ident::#ident => #discriminant.into_val(env) };
            (spec_case, discriminant, try_from, into)
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
                #(#discriminant_consts)*
                let vec: soroban_sdk::Vec<soroban_sdk::RawVal> = val.try_into_val(env)?;
                let mut iter = vec.iter();
                let discriminant = iter.next().ok_or(soroban_sdk::ConversionError)??;
                Ok(match discriminant.get_payload() {
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
                #(#discriminant_consts)*
                match &self {
                    #(#intos,)*
                }
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for &#enum_ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                #(#discriminant_consts)*
                match self {
                    #(#intos,)*
                }
            }
        }
    }
}
