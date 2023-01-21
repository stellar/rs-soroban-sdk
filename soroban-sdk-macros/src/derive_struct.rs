use itertools::Itertools;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct, Error, Ident, Path, Visibility};

use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, WriteXdr,
    SCSYMBOL_LIMIT,
};

use crate::{doc::docs_from_attrs, map_type::map_type};

// TODO: Add field attribute for including/excluding fields in types.
// TODO: Better handling of partial types and types without all their fields and
// types with private fields.

pub fn derive_type_struct(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    attrs: &[Attribute],
    data: &DataStruct,
    spec: bool,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    let fields = &data.fields;
    let field_count_usize: usize = fields.len();
    let (spec_fields, field_idents, field_names, field_idx_lits, try_from_xdrs, try_into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .sorted_by_key(|field| field.ident.as_ref().unwrap().to_string())
        .enumerate()
        .map(|(field_num, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_idx_lit = Literal::usize_unsuffixed(field_num);

            if field_name.len() > SCSYMBOL_LIMIT as usize {
                errors.push(Error::new(field_ident.span(), format!("struct field name is too long: {}, max is {}", field_name.len(), SCSYMBOL_LIMIT)));
            }
            let spec_field = ScSpecUdtStructFieldV0 {
                doc: docs_from_attrs(&field.attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
                name: field_name.clone().try_into().unwrap_or_else(|_| StringM::default()),
                type_: match map_type(&field.ty) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                },
            };
            let try_from_xdr = quote! {
                #field_ident: {
                    let key = &#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?;
                    let idx = map.binary_search_by_key(key, |entry| entry.key.clone()).map_err(|_| #path::xdr::Error::Invalid)?;
                    let rv: #path::RawVal = (&map[idx].val.clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                }
            };
            let try_into_xdr = quote! {
                #path::xdr::ScMapEntry {
                    key: #field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?,
                    val: (&val.#field_ident).try_into().map_err(|_| #path::xdr::Error::Invalid)?,
                }
            };
            (spec_field, field_ident, field_name, field_idx_lit, try_from_xdr, try_into_xdr)
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
            doc: docs_from_attrs(attrs).try_into().unwrap(), // TODO: Truncate docs, or display friendly compile error.
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: ident.to_string().try_into().unwrap(),
            fields: spec_fields.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr().unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_TYPE_{}", ident.to_string().to_uppercase());
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

    let arbitrary_tokens = crate::arbitrary::derive_arbitrary_struct(path, vis, ident, data);

    // Output.
    quote! {
        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::RawVal> for #ident {
            type Error = #path::ConversionError;
            fn try_from_val(env: &#path::Env, val: &#path::RawVal) -> Result<Self, Self::Error> {
                use #path::{TryIntoVal,EnvBase,ConversionError,RawVal,MapObject};
                const KEYS: [&'static str; #field_count_usize] = [#(#field_names),*];
                let mut vals: [RawVal; #field_count_usize] = [RawVal::VOID.to_raw(); #field_count_usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.map_unpack_to_slice(map, &KEYS, &mut vals).map_err(|_| ConversionError)?;
                Ok(Self {
                    #(#field_idents: vals[#field_idx_lits].try_into_val(env).map_err(|_| #path::ConversionError)?,)*
                })
            }
        }

        impl #path::TryFromVal<#path::Env, #ident> for #path::RawVal {
            type Error = #path::ConversionError;
            fn try_from_val(env: &#path::Env, val: &#ident) -> Result<Self, Self::Error> {
                use #path::{TryIntoVal,EnvBase,ConversionError,RawVal};
                const KEYS: [&'static str; #field_count_usize] = [#(#field_names),*];
                let vals: [RawVal; #field_count_usize] = [
                    #((&val.#field_idents).try_into_val(env).map_err(|_| ConversionError)?),*
                ];
                Ok(env.map_new_from_slices(&KEYS, &vals).map_err(|_| ConversionError)?.into())
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScMap> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::xdr::ScMap) -> Result<Self, Self::Error> {
                use #path::xdr::Validate;
                use #path::TryIntoVal;
                let map = val;
                if map.len() != #field_count_usize {
                    return Err(#path::xdr::Error::Invalid);
                }
                map.validate()?;
                Ok(Self{
                    #(#try_from_xdrs,)*
                })
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #ident {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVal) -> Result<Self, Self::Error> {
                if let #path::xdr::ScVal::Map(Some(map)) = val {
                    <_ as #path::TryFromVal<_, _>>::try_from_val(env, map)
                } else {
                    Err(#path::xdr::Error::Invalid)
                }
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<&#ident> for #path::xdr::ScMap  {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: &#ident) -> Result<Self, Self::Error> {
                extern crate alloc;
                use #path::TryFromVal;
                #path::xdr::ScMap::sorted_from(alloc::vec![
                    #(#try_into_xdrs,)*
                ])
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<#ident> for #path::xdr::ScMap {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: #ident) -> Result<Self, Self::Error> {
                (&val).try_into()
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<&#ident> for #path::xdr::ScVal  {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: &#ident) -> Result<Self, Self::Error> {
                Ok(#path::xdr::ScVal::Map(Some(val.try_into()?)))
            }
        }

        #[cfg(any(test, feature = "testutils"))]
        impl TryFrom<#ident> for #path::xdr::ScVal {
            type Error = #path::xdr::Error;
            #[inline(always)]
            fn try_from(val: #ident) -> Result<Self, Self::Error> {
                (&val).try_into()
            }
        }

        #arbitrary_tokens
    }
}
