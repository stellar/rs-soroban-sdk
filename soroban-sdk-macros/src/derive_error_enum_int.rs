use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::{ScSpecEntry, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, StringM, WriteXdr};
use syn::{spanned::Spanned, DataEnum, Error, ExprLit, Ident, Lit, Path};

pub fn derive_type_error_enum_int(
    path: &Path,
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
            let spec_case = ScSpecUdtErrorEnumCaseV0 {
                name: name.try_into().unwrap_or_else(|_| StringM::default()),
                value: discriminant,
            };
            let try_from = quote! { #discriminant => Self::#ident };
            let into =
                quote! { #enum_ident::#ident => #path::Status::from_contract_error(#discriminant) };
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

        impl TryFrom<#path::Status> for #enum_ident {
            type Error = #path::Status;
            #[inline(always)]
            fn try_from(status: #path::Status) -> Result<Self, Self::Error> {
                if status.is_type(#path::xdr::ScStatusType::ContractError) {
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

        impl TryFrom<&#path::Status> for #enum_ident {
            type Error = #path::Status;
            #[inline(always)]
            fn try_from(status: &#path::Status) -> Result<Self, Self::Error> {
                <_ as TryFrom<#path::Status>>::try_from(*status)
            }
        }

        impl From<#enum_ident> for #path::Status {
            #[inline(always)]
            fn from(val: #enum_ident) -> #path::Status {
                match val {
                    #(#intos,)*
                }
            }
        }

        impl From<&#enum_ident> for #path::Status {
            #[inline(always)]
            fn from(val: &#enum_ident) -> #path::Status {
                <_ as From<#enum_ident>>::from(*val)
            }
        }

        impl #path::TryFromVal<#path::Env, #path::RawVal> for #enum_ident {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: #path::RawVal) -> Result<Self, Self::Error> {
                use #path::TryIntoVal;
                let status: #path::Status = val.try_into_val(env)?;
                status.try_into().map_err(|_| #path::ConversionError)
            }
        }

        impl #path::TryIntoVal<#path::Env, #enum_ident> for #path::RawVal {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_into_val(self, env: &#path::Env) -> Result<#enum_ident, Self::Error> {
                <_ as #path::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        impl #path::IntoVal<#path::Env, #path::RawVal> for #enum_ident {
            #[inline(always)]
            fn into_val(self, env: &#path::Env) -> #path::RawVal {
                let status: #path::Status = self.into();
                status.into_val(env)
            }
        }

        impl #path::IntoVal<#path::Env, #path::RawVal> for &#enum_ident {
            #[inline(always)]
            fn into_val(self, env: &#path::Env) -> #path::RawVal {
                let status: #path::Status = self.into();
                status.into_val(env)
            }
        }
    }
}
