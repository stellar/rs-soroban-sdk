use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct, Error, Ident, Visibility};

use stellar_xdr::{
    SpecEntry, SpecEntryUdt, SpecEntryUdtV0, SpecTypeDef, SpecUdtDef, SpecUdtStruct,
    SpecUdtStructField, SpecUdtUnion, SpecUdtUnionCase, VecM, WriteXdr,
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
            let spec_field = SpecUdtStructField {
                name: name.clone().try_into().unwrap_or_else(|_| {
                    errors.push(Error::new(ident.span(), "struct field name too long"));
                    VecM::default()
                }),
                type_: Box::new(match map_type(&f.ty) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        SpecTypeDef::I32
                    }
                }),
            };
            let map_key = quote! { stellar_contract_sdk::Symbol::from_str(#name) }; // TODO: Handle field names longer than a symbol. Hash the name? Truncate the name?
            let try_from = quote! { #ident: map.get::<_, EnvVal>(#map_key).try_into()? };
            let into = quote! { map.put(#map_key, self.#ident.into_env_val(env)) };
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
        let spec_entry_udt = SpecEntryUdtV0 {
            name: ident.to_string().try_into().unwrap(),
            typ: SpecUdtDef::Struct(Box::new(SpecUdtStruct {
                fields: spec_fields.try_into().unwrap(),
            })),
        };
        let spec_entry = SpecEntry::Udt(SpecEntryUdt::V0(spec_entry_udt));
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

        impl TryFrom<EnvVal> for #ident {
            type Error = ConversionError;
            #[inline(always)]
            fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
                let map: stellar_contract_sdk::Map = ev.try_into()?;
                Ok(Self{
                    #(#try_froms,)*
                })
            }
        }

        impl IntoEnvVal<Env, RawVal> for #ident {
            #[inline(always)]
            fn into_env_val(self, env: &Env) -> EnvVal {
                let mut map = stellar_contract_sdk::Map::new(env);
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
    let (spec_cases, try_froms, intos): (Vec<_>, Vec<_>, Vec<_>) = variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let i: u32 = i.try_into().unwrap();
            // TODO: Choose discriminant type based on repr type of enum.
            // TODO: Should we use variants explicit discriminant? Probably not.
            // Should have a separate derive for those types of enums that maps
            // to an integer type only.
            // TODO: Use attributes tagged on variant to control whether field is included.
            // TODO: Support multi-field enum variants.
            // TODO: Or, error on multi-field enum variants.
            let ident = &v.ident;
            let name = ident.to_string();
            let field = v.fields.iter().next();
            if let Some(f) = field {
                let spec_case = SpecUdtUnionCase {
                    name: name.try_into().unwrap_or_else(|_| {
                        errors.push(Error::new(ident.span(), "union case name too long"));
                        VecM::default()
                    }),
                    type_: Some(Box::new(match map_type(&f.ty) {
                        Ok(t) => t,
                        Err(e) => {
                            errors.push(e);
                            SpecTypeDef::I32
                        }
                    })),
                };
                let try_from = quote! { #i => Self::#ident(value.try_into()?) };
                let into = quote! { Self::#ident(value) => (#i, value).into_env_val(env) };
                (spec_case, try_from, into)
            } else {
                let spec_case = SpecUdtUnionCase {
                    name: name.try_into().unwrap_or_else(|_| {
                        errors.push(Error::new(ident.span(), "union case name too long"));
                        VecM::default()
                    }),
                    type_: None,
                };
                let try_from = quote! { #i => Self::#ident };
                let into = quote! { Self::#ident => (#i, ()).into_env_val(env) };
                (spec_case, try_from, into)
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
        let spec_entry_udt = SpecEntryUdtV0 {
            name: ident.to_string().try_into().unwrap(),
            typ: SpecUdtDef::Union(Box::new(SpecUdtUnion {
                cases: spec_cases.try_into().unwrap(),
            })),
        };
        let spec_entry = SpecEntry::Udt(SpecEntryUdt::V0(spec_entry_udt));
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

        impl TryFrom<EnvVal> for #ident {
            type Error = ConversionError;
            #[inline(always)]
            fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
                let (discriminant, value): (u32, EnvVal) = ev.try_into()?;
                let v = match discriminant {
                    #(#try_froms,)*
                    _ => Err(ConversionError{})?
                };
                Ok(v)
            }
        }

        impl IntoEnvVal<Env, RawVal> for #ident {
            #[inline(always)]
            fn into_env_val(self, env: &Env) -> EnvVal {
                match self {
                    #(#intos,)*
                }
            }
        }
    }
}
