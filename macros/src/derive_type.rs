use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct, Error, Ident, Visibility};

use stellar_xdr::{
    SpecEntry, SpecUdtUnionV0, SpecUdtStructV0, SpecTypeDef,
    SpecUdtStructFieldV0, SpecUdtUnionCaseV0, VecM, WriteXdr,
};

use crate::map_type::map_type;

// TODO: In enums replace use of index integers with symbols.
// TODO: Add field attribute for including/excluding fields in types.
// TODO: Better handling of partial types and types without all their fields and
// types with private fields.

pub fn derive_type_struct(ident: &Ident, data: &DataStruct, spec: bool) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let fields = &data.fields;
    let (spec_fields, try_froms, intos): (Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|f| matches!(f.vis, Visibility::Public(_)))
        .enumerate()
        .map(|(i, f)| {
            let ident = f
                .ident
                .as_ref()
                .map_or_else(|| format_ident!("{}", i), Ident::clone);
            let name = ident.to_string();
            let spec_field = SpecUdtStructFieldV0 {
                name: name.clone().try_into().unwrap_or_else(|_| {
                    errors.push(Error::new(ident.span(), "struct field name too long"));
                    VecM::default()
                }),
                type_: match map_type(&f.ty) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        SpecTypeDef::I32
                    }
                },
            };
            let map_key = quote! { // TODO: Handle field names longer than a symbol. Hash the name? Truncate the name?
                { const k: stellar_contract_sdk::Symbol = stellar_contract_sdk::Symbol::from_str(#name); k }
            };
            let try_from = quote! {
                #ident: map
                    .get(#map_key)
                    .map_err(|_| ConversionError)?
                    .try_into()?
            };
            let into = quote! { map.insert(#map_key, self.#ident.into_env_val(env)) };
            (spec_field, try_from, into)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = SpecEntry::UdtStructV0(SpecUdtStructV0 {
            name: ident.to_string().try_into().unwrap(),
            fields: spec_fields.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = *#spec_xdr_lit;
        })
    } else {
        None
    };

    // Output.
    quote! {
        #spec_gen

        impl TryFrom<stellar_contract_sdk::EnvVal> for #ident {
            type Error = stellar_contract_sdk::ConversionError;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvVal) -> Result<Self, Self::Error> {
                let map: stellar_contract_sdk::Map<stellar_contract_sdk::Symbol, stellar_contract_sdk::EnvVal> = ev.try_into()?;
                Ok(Self{
                    #(#try_froms,)*
                })
            }
        }

        impl IntoEnvVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal> for #ident {
            #[inline(always)]
            fn into_env_val(self, env: &stellar_contract_sdk::Env) -> stellar_contract_sdk::EnvVal {
                let mut map = stellar_contract_sdk::Map::<stellar_contract_sdk::Symbol, stellar_contract_sdk::EnvVal>::new(env);
                #(#intos;)*
                map.into()
            }
        }
    }
}

pub fn derive_type_enum(ident: &Ident, data: &DataEnum, spec: bool) -> TokenStream2 {
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
            let name = ident.to_string();
            let field = v.fields.iter().next();
            let discriminant_const_sym_ident = format_ident!("DISCRIMINANT_SYM_{}", name.to_uppercase());
            let discriminant_const_u64_ident = format_ident!("DISCRIMINANT_U64_{}", name.to_uppercase());
            let discriminant_const_sym = quote! {
                const #discriminant_const_sym_ident: stellar_contract_sdk::Symbol = stellar_contract_sdk::Symbol::from_str(#name);
            };
            let discriminant_const_u64 = quote! {
                const #discriminant_const_u64_ident: u64 = #discriminant_const_sym_ident.to_raw().get_payload();
            };
            let discriminant_const = quote! {
                #discriminant_const_sym
                #discriminant_const_u64
            };
            if let Some(f) = field {
                let spec_case = SpecUdtUnionCaseV0 {
                    name: name.try_into().unwrap_or_else(|_| {
                        errors.push(Error::new(ident.span(), "union case name too long"));
                        VecM::default()
                    }),
                    type_: Some(match map_type(&f.ty) {
                        Ok(t) => t,
                        Err(e) => {
                            errors.push(e);
                            SpecTypeDef::I32
                        }
                    }),
                };
                let try_from = quote! { #discriminant_const_u64_ident => Self::#ident(value.try_into()?) };
                let into = quote! { Self::#ident(value) => (#discriminant_const_sym_ident, value).into_env_val(env) };
                (spec_case, discriminant_const, try_from, into)
            } else {
                let spec_case = SpecUdtUnionCaseV0 {
                    name: name.try_into().unwrap_or_else(|_| {
                        errors.push(Error::new(ident.span(), "union case name too long"));
                        VecM::default()
                    }),
                    type_: None,
                };
                let try_from = quote! { #discriminant_const_u64_ident => Self::#ident };
                let into = quote! { Self::#ident => (#discriminant_const_sym_ident, ()).into_env_val(env) };
                (spec_case, discriminant_const, try_from, into)
            }
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = SpecEntry::UdtUnionV0(SpecUdtUnionV0 {
            name: ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = *#spec_xdr_lit;
        })
    } else {
        None
    };

    // Output.
    quote! {
        #spec_gen

        impl TryFrom<stellar_contract_sdk::EnvVal> for #ident {
            type Error = stellar_contract_sdk::ConversionError;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvVal) -> Result<Self, Self::Error> {
                #(#discriminant_consts)*
                let (discriminant, value): (stellar_contract_sdk::Symbol, stellar_contract_sdk::EnvVal) = ev.try_into()?;
                Ok(match discriminant.to_raw().get_payload() {
                    #(#try_froms,)*
                    _ => Err(stellar_contract_sdk::ConversionError{})?,
                })
            }
        }

        impl stellar_contract_sdk::IntoEnvVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal> for #ident {
            #[inline(always)]
            fn into_env_val(self, env: &stellar_contract_sdk::Env) -> stellar_contract_sdk::EnvVal {
                #(#discriminant_consts)*
                match self {
                    #(#intos,)*
                }
            }
        }
    }
}
