use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use soroban_env_common::Symbol;
use stellar_xdr::{ScSpecEntry, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, VecM, WriteXdr};
use syn::{spanned::Spanned, DataEnum, Error, ExprLit, Ident, Lit};

pub fn derive_type_error_enum_int(
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
            if let Err(e) = Symbol::try_from_str(name) {
                errors.push(Error::new(ident.span(), format!("enum variant name {}", e)));
            }
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
                name: name.try_into().unwrap_or_else(|_| VecM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into = quote! { #enum_ident::#ident => soroban_sdk::Status::from_contract_error(#discriminant) };
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

        impl TryFrom<soroban_sdk::Status> for #enum_ident {
            type Error = soroban_sdk::Status;
            #[inline(always)]
            fn try_from(status: soroban_sdk::Status) -> Result<Self, Self::Error> {
                if status.is_type(soroban_sdk::xdr::ScStatusType::ContractError) {
                    let discriminant = status.get_code();
                    Ok(match discriminant {
                        #(#try_froms,)*
                        _ => return Err(status),
                    })
                } else {
                    Err(status)
                }
            }
        }

        impl TryFrom<&soroban_sdk::Status> for #enum_ident {
            type Error = soroban_sdk::Status;
            #[inline(always)]
            fn try_from(status: &soroban_sdk::Status) -> Result<Self, Self::Error> {
                <_ as TryFrom<soroban_sdk::Status>>::try_from(*status)
            }
        }

        impl From<#enum_ident> for soroban_sdk::Status {
            #[inline(always)]
            fn from(val: #enum_ident) -> soroban_sdk::Status {
                match val {
                    #(#intos,)*
                }
            }
        }

        impl From<&#enum_ident> for soroban_sdk::Status {
            #[inline(always)]
            fn from(val: &#enum_ident) -> soroban_sdk::Status {
                <_ as From<#enum_ident>>::from(*val)
            }
        }

        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::RawVal> for #enum_ident {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::RawVal) -> Result<Self, Self::Error> {
                use soroban_sdk::TryIntoVal;
                let status: soroban_sdk::Status = val.try_into_val(env)?;
                status.try_into().map_err(|_| soroban_sdk::ConversionError)
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
                let status: Status = self.into();
                status.into_val(env)
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for &#enum_ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                let status: Status = self.into();
                status.into_val(env)
            }
        }
    }
}
