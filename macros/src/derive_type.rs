use itertools::MultiUnzip;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct, Error, Ident, Visibility};

use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, ScSpecUdtUnionCaseV0,
    ScSpecUdtUnionV0, VecM, WriteXdr,
};

use crate::map_type::map_type;

// TODO: Add field attribute for including/excluding fields in types.
// TODO: Better handling of partial types and types without all their fields and
// types with private fields.

pub fn derive_type_struct(ident: &Ident, data: &DataStruct, spec: bool) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let fields = &data.fields;
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
            let spec_field = ScSpecUdtStructFieldV0 {
                name: name.clone().try_into().unwrap_or_else(|_| {
                    errors.push(Error::new(ident.span(), "struct field name too long"));
                    VecM::default()
                }),
                type_: match map_type(&f.ty) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                },
            };
            let map_key = quote! { // TODO: Handle field names longer than a symbol. Hash the name? Truncate the name?
                { const k: stellar_contract_sdk::Symbol = stellar_contract_sdk::Symbol::from_str(#name); k }
            };
            let try_from = quote! {
                #ident: if let Some(Ok(val)) = map.get(#map_key) {
                    val.try_into_val(env)?
                } else {
                    Err(stellar_contract_sdk::ConversionError)?
                }
            };
            let into = quote! { map.set(#map_key, stellar_contract_sdk::EnvVal { env: env.clone(), val: self.#ident.into_val(env) }) };
            let try_from_xdr = quote! {
                #ident: {
                    let key = &#name.try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
                    let idx = map.binary_search_by_key(key, |entry| entry.key.clone()).map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
                    let ev: stellar_contract_sdk::EnvVal = stellar_contract_sdk::EnvVal{
                        env: ev.env.clone(),
                        val: (&map[idx].val.clone()).try_into_val(&ev.env).map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?
                    };
                    ev.try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?
                }
            };
            let into_xdr = quote! {
                stellar_contract_sdk::xdr::ScMapEntry {
                    key: #name.try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
                    val: (&self.#ident).try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
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
                use stellar_contract_sdk::TryIntoVal;
                let map: stellar_contract_sdk::Map<stellar_contract_sdk::Symbol, stellar_contract_sdk::EnvVal> = ev.try_into()?;
                let env = map.env();
                Ok(Self{
                    #(#try_froms,)*
                })
            }
        }

        impl stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal> for #ident {
            #[inline(always)]
            fn into_val(self, env: &stellar_contract_sdk::Env) -> stellar_contract_sdk::RawVal {
                let mut map = stellar_contract_sdk::Map::<stellar_contract_sdk::Symbol, stellar_contract_sdk::EnvVal>::new(env);
                #(#intos;)*
                map.into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScMap>> for #ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScMap>) -> Result<Self, Self::Error> {
                use stellar_contract_sdk::xdr::Validate;
                use stellar_contract_sdk::EnvType;
                use stellar_contract_sdk::TryIntoVal;
                let map = ev.val;
                map.validate()?;
                Ok(Self{
                    #(#try_from_xdrs,)*
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl stellar_contract_sdk::TryIntoVal<stellar_contract_sdk::Env, #ident> for stellar_contract_sdk::xdr::ScMap {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &stellar_contract_sdk::Env) -> Result<#ident, Self::Error> {
                stellar_contract_sdk::EnvType{ env: env.clone(), val: self }.try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScObject>> for #ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScObject>) -> Result<Self, Self::Error> {
                if let stellar_contract_sdk::xdr::ScObject::Map(map) = ev.val {
                    stellar_contract_sdk::EnvType{ env: ev.env, val: map }.try_into()
                } else {
                    Err(stellar_contract_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl stellar_contract_sdk::TryIntoVal<stellar_contract_sdk::Env, #ident> for stellar_contract_sdk::xdr::ScObject {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &stellar_contract_sdk::Env) -> Result<#ident, Self::Error> {
                stellar_contract_sdk::EnvType{ env: env.clone(), val: self }.try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVal>> for #ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVal>) -> Result<Self, Self::Error> {
                if let stellar_contract_sdk::xdr::ScVal::Object(Some(obj)) = ev.val {
                    stellar_contract_sdk::EnvType{ env: ev.env, val: obj }.try_into()
                } else {
                    Err(stellar_contract_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl stellar_contract_sdk::TryIntoVal<stellar_contract_sdk::Env, #ident> for stellar_contract_sdk::xdr::ScVal {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &stellar_contract_sdk::Env) -> Result<#ident, Self::Error> {
                stellar_contract_sdk::EnvType{ env: env.clone(), val: self }.try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScMap> for &#ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScMap, Self::Error> {
                extern crate alloc;
                stellar_contract_sdk::xdr::ScMap::sorted_from(alloc::vec![
                    #(#into_xdrs,)*
                ])
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScMap> for #ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScMap, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScObject> for &#ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScObject, Self::Error> {
                Ok(stellar_contract_sdk::xdr::ScObject::Map(self.try_into()?))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScObject> for #ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScObject, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScVal> for &#ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVal, Self::Error> {
                Ok(stellar_contract_sdk::xdr::ScVal::Object(Some(self.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScVal> for #ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVal, Self::Error> {
                (&self).try_into()
            }
        }
    }
}

pub fn derive_type_enum(enum_ident: &Ident, data: &DataEnum, spec: bool) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let variants = &data.variants;
    let (spec_cases, discriminant_consts, try_froms, intos, try_from_xdrs, into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = variants
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
                let spec_case = ScSpecUdtUnionCaseV0 {
                    name: name.try_into().unwrap_or_else(|_| {
                        errors.push(Error::new(ident.span(), "union case name too long"));
                        VecM::default()
                    }),
                    type_: Some(match map_type(&f.ty) {
                        Ok(t) => t,
                        Err(e) => {
                            errors.push(e);
                            ScSpecTypeDef::I32
                        }
                    }),
                };
                let try_from = quote! { #discriminant_const_u64_ident => Self::#ident(value.try_into_val(&env)?) };
                let into = quote! { Self::#ident(value) => (#discriminant_const_sym_ident, value).into_val(env) };
                let try_from_xdr = quote! {
                    #name => Self::#ident(
                        stellar_contract_sdk::EnvVal {
                            env: ev.env.clone(),
                            val: (&value).try_into_val(&ev.env).map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
                        }
                        .try_into()
                        .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?
                    )
                };
                let into_xdr = quote! { #enum_ident::#ident(value) => (#name, value).try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)? };
                (spec_case, discriminant_const, try_from, into, try_from_xdr, into_xdr)
            } else {
                let spec_case = ScSpecUdtUnionCaseV0 {
                    name: name.try_into().unwrap_or_else(|_| {
                        errors.push(Error::new(ident.span(), "union case name too long"));
                        VecM::default()
                    }),
                    type_: None,
                };
                let try_from = quote! { #discriminant_const_u64_ident => Self::#ident };
                let into = quote! { Self::#ident => (#discriminant_const_sym_ident, ()).into_val(env) };
                let try_from_xdr = quote! { #name => Self::#ident };
                let into_xdr = quote! { #enum_ident::#ident => (#name, ()).try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)? };
                (spec_case, discriminant_const, try_from, into, try_from_xdr, into_xdr)
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
        let spec_entry = ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            name: enum_ident.to_string().try_into().unwrap(),
            cases: spec_cases.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_{}", enum_ident.to_string().to_uppercase());
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

        impl TryFrom<stellar_contract_sdk::EnvVal> for #enum_ident {
            type Error = stellar_contract_sdk::ConversionError;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvVal) -> Result<Self, Self::Error> {
                use stellar_contract_sdk::TryIntoVal;
                #(#discriminant_consts)*
                let env = ev.env.clone();
                let (discriminant, value): (stellar_contract_sdk::Symbol, stellar_contract_sdk::EnvVal) = ev.try_into()?;
                Ok(match discriminant.to_raw().get_payload() {
                    #(#try_froms,)*
                    _ => Err(stellar_contract_sdk::ConversionError{})?,
                })
            }
        }

        impl stellar_contract_sdk::IntoVal<stellar_contract_sdk::Env, stellar_contract_sdk::RawVal> for #enum_ident {
            #[inline(always)]
            fn into_val(self, env: &stellar_contract_sdk::Env) -> stellar_contract_sdk::RawVal {
                #(#discriminant_consts)*
                match self {
                    #(#intos,)*
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVec>> for #enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVec>) -> Result<Self, Self::Error> {
                use stellar_contract_sdk::xdr::Validate;
                use stellar_contract_sdk::EnvType;
                use stellar_contract_sdk::TryIntoVal;
                let (discriminant, value): (stellar_contract_sdk::xdr::ScSymbol, stellar_contract_sdk::xdr::ScVal) = ev.val.try_into().map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
                let discriminant_name: &str = &discriminant.to_string()?;
                Ok(match discriminant_name {
                    #(#try_from_xdrs,)*
                    _ => Err(stellar_contract_sdk::xdr::Error::Invalid)?,
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl stellar_contract_sdk::TryIntoVal<stellar_contract_sdk::Env, #enum_ident> for stellar_contract_sdk::xdr::ScVec {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &stellar_contract_sdk::Env) -> Result<#enum_ident, Self::Error> {
                stellar_contract_sdk::EnvType{ env: env.clone(), val: self }.try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScObject>> for #enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScObject>) -> Result<Self, Self::Error> {
                if let stellar_contract_sdk::xdr::ScObject::Vec(vec) = ev.val {
                    stellar_contract_sdk::EnvType{ env: ev.env, val: vec }.try_into()
                } else {
                    Err(stellar_contract_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl stellar_contract_sdk::TryIntoVal<stellar_contract_sdk::Env, #enum_ident> for stellar_contract_sdk::xdr::ScObject {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &stellar_contract_sdk::Env) -> Result<#enum_ident, Self::Error> {
                stellar_contract_sdk::EnvType{ env: env.clone(), val: self }.try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVal>> for #enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_from(ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVal>) -> Result<Self, Self::Error> {
                if let stellar_contract_sdk::xdr::ScVal::Object(Some(obj)) = ev.val {
                    stellar_contract_sdk::EnvType{ env: ev.env, val: obj }.try_into()
                } else {
                    Err(stellar_contract_sdk::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl stellar_contract_sdk::TryIntoVal<stellar_contract_sdk::Env, #enum_ident> for stellar_contract_sdk::xdr::ScVal {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into_val(self, env: &stellar_contract_sdk::Env) -> Result<#enum_ident, Self::Error> {
                stellar_contract_sdk::EnvType{ env: env.clone(), val: self }.try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScVec> for &#enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVec, Self::Error> {
                extern crate alloc;
                Ok(match self {
                    #(#into_xdrs,)*
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScVec> for #enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVec, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScObject> for &#enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScObject, Self::Error> {
                Ok(stellar_contract_sdk::xdr::ScObject::Vec(self.try_into()?))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScObject> for #enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScObject, Self::Error> {
                (&self).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScVal> for &#enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVal, Self::Error> {
                Ok(stellar_contract_sdk::xdr::ScVal::Object(Some(self.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryInto<stellar_contract_sdk::xdr::ScVal> for #enum_ident {
            type Error = stellar_contract_sdk::xdr::Error;
            #[inline(always)]
            fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVal, Self::Error> {
                (&self).try_into()
            }
        }
    }
}
