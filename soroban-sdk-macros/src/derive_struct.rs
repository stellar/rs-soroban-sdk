use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use soroban_env_common::Symbol;
use syn::{spanned::Spanned, DataStruct, Error, Ident, Visibility};

use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, VecM, WriteXdr,
};

use crate::map_type::map_type;

// TODO: Add field attribute for including/excluding fields in types.
// TODO: Better handling of partial types and types without all their fields and
// types with private fields.

pub fn derive_type_struct(
    ident: &Ident,
    data: &DataStruct,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let fields = &data.fields;
    let field_count_usize: usize = fields.len();
    let field_count_u32: u32 = fields.len().try_into().unwrap_or_else(|_| {
        errors.push(Error::new(
            data.struct_token.span(),
            "struct has too many fields exceeding u32::MAX",
        ));
        0
    });
    let (spec_fields, try_froms, intos, try_from_xdrs, into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|f| matches!(f.vis, Visibility::Public(_)))
        .enumerate()
        .map(|(i, f)| {
            let ident = f
                .ident
                .as_ref()
                .map_or_else(|| format_ident!("{}", i), Ident::clone);
            let name = ident.to_string();
            if let Err(e) = Symbol::try_from_str(&name) {
                errors.push(Error::new(ident.span(), format!("struct field name {}", e)));
            }
            let spec_field = ScSpecUdtStructFieldV0 {
                name: name.clone().try_into().unwrap_or_else(|_| VecM::default()),
                type_: match map_type(&f.ty) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                },
            };
            let map_key = quote! { ::soroban_sdk::symbol!(#name) };
            let try_from = quote! {
                #ident: if let Some(Ok(val)) = map.get(#map_key) {
                    val.try_into_val(env)?
                } else {
                    Err(soroban_sdk::ConversionError)?
                }
            };
            let into = quote! { map.set(#map_key, (&self.#ident).into_val(env)) };
            let try_from_xdr = quote! {
                #ident: {
                    let key = &#name.try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let idx = map.binary_search_by_key(key, |entry| entry.key.clone()).map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    let rv: soroban_sdk::RawVal = (&map[idx].val.clone()).try_into_val(env).map_err(|_| soroban_sdk::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| soroban_sdk::xdr::Error::Invalid)?
                }
            };
            let into_xdr = quote! {
                soroban_sdk::xdr::ScMapEntry {
                    key: #name.try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                    val: (&self.#ident).try_into().map_err(|_| soroban_sdk::xdr::Error::Invalid)?,
                }
            };
            (spec_field, try_from, into, try_from_xdr, into_xdr)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: ident.to_string().try_into().unwrap(),
            fields: spec_fields.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", ident.to_string().to_uppercase());
        Some(quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #ident::spec_xdr();

            impl #ident {
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

        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::RawVal> for #ident {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::RawVal) -> Result<Self, Self::Error> {
                use soroban_sdk::TryIntoVal;
                let map: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::RawVal> = val.try_into_val(env)?;
                if map.len() != #field_count_u32 {
                    return Err(soroban_sdk::ConversionError);
                }
                Ok(Self{
                    #(#try_froms,)*
                })
            }
        }

        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #ident> for soroban_sdk::RawVal {
            type Error = soroban_sdk::ConversionError;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for #ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                let mut map = soroban_sdk::Map::<soroban_sdk::Symbol, soroban_sdk::RawVal>::new(env);
                #(#intos;)*
                map.into()
            }
        }

        impl soroban_sdk::IntoVal<soroban_sdk::Env, soroban_sdk::RawVal> for &#ident {
            #[inline(always)]
            fn into_val(self, env: &soroban_sdk::Env) -> soroban_sdk::RawVal {
                let mut map = soroban_sdk::Map::<soroban_sdk::Symbol, soroban_sdk::RawVal>::new(env);
                #(#intos;)*
                map.into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScMap> for #ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScMap) -> Result<Self, Self::Error> {
                use soroban_sdk::xdr::Validate;
                use soroban_sdk::TryIntoVal;
                let map = val;
                if map.len() != #field_count_usize {
                    return Err(soroban_sdk::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self{
                    #(#try_from_xdrs,)*
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #ident> for soroban_sdk::xdr::ScMap {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScObject> for #ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScObject) -> Result<Self, Self::Error> {
                if let soroban_sdk::xdr::ScObject::Map(map) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #ident> for soroban_sdk::xdr::ScObject {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryFromVal<soroban_sdk::Env, soroban_sdk::xdr::ScVal> for #ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &soroban_sdk::Env, val: soroban_sdk::xdr::ScVal) -> Result<Self, Self::Error> {
                if let soroban_sdk::xdr::ScVal::Object(Some(obj)) = val {
                    <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, obj)
                } else {
                    Err(soroban_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl soroban_sdk::TryIntoVal<soroban_sdk::Env, #ident> for soroban_sdk::xdr::ScVal {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &soroban_sdk::Env) -> Result<#ident, Self::Error> {
                <_ as soroban_sdk::TryFromVal<_, _>>::try_from_val(env, self)
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScMap> for &#ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScMap, Self::Error> {
                extern crate alloc;
                use soroban_sdk::TryFromVal;
                soroban_sdk::xdr::ScMap::sorted_from(alloc::vec![
                    #(#into_xdrs,)*
                ])
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScMap> for #ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScMap, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScObject> for &#ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScObject, Self::Error> {
                Ok(soroban_sdk::xdr::ScObject::Map(self.try_into()?))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScObject> for #ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScObject, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVal> for &#ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, Self::Error> {
                Ok(soroban_sdk::xdr::ScVal::Object(Some(self.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<soroban_sdk::xdr::ScVal> for #ident {
            type Error = soroban_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<soroban_sdk::xdr::ScVal, Self::Error> {
                (&self).try_into()
            }
        }
    }
}
