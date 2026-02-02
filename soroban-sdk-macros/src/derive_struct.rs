use itertools::Itertools;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct, Error, Ident, Path, Visibility};

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, WriteXdr,
};

use crate::{
    doc::docs_from_attrs, map_type::is_option_type, map_type::map_type, DEFAULT_XDR_RW_LIMITS,
};

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

    // Collect field data including whether each field is an Option type
    let (spec_fields, field_idents, field_names, field_idx_lits, field_is_options, try_from_xdrs, try_into_xdr_stmts, try_into_xdr_exprs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .sorted_by_key(|field| field.ident.as_ref().unwrap().to_string())
        .enumerate()
        .map(|(field_num, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_idx_lit = Literal::usize_unsuffixed(field_num);
            let field_is_option = is_option_type(&field.ty);
            let spec_field = ScSpecUdtStructFieldV0 {
                doc: docs_from_attrs(&field.attrs),
                name: field_name.clone().try_into().unwrap_or_else(|_| {
                    const MAX: u32 = 30;
                    errors.push(Error::new(field_ident.span(), format!("struct field name is too long: {}, max is {MAX}", field_name.len())));
                    StringM::<MAX>::default()
                }),
                type_: match map_type(&field.ty,false, false) {
                    Ok(t) => t,
                    Err(e) => {
                        errors.push(e);
                        ScSpecTypeDef::I32
                    }
                },
            };

            // Generate XDR conversion code based on whether field is Option
            let try_from_xdr = if field_is_option {
                // For Option fields: handle missing keys gracefully
                quote! {
                    #field_ident: {
                        let key: #path::xdr::ScVal = #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into();
                        match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                            Ok(idx) => {
                                let rv: #path::Val = (&map[idx].val.clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                                // Handle backwards compat: VOID -> None
                                if rv.is_void() {
                                    None
                                } else {
                                    Some(rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?)
                                }
                            }
                            Err(_) => None // Missing key -> None
                        }
                    }
                }
            } else {
                // For non-Option fields: key must exist
                quote! {
                    #field_ident: {
                        let key: #path::xdr::ScVal = #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into();
                        let idx = map.binary_search_by_key(&key, |entry| entry.key.clone()).map_err(|_| #path::xdr::Error::Invalid)?;
                        let rv: #path::Val = (&map[idx].val.clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                        rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                    }
                }
            };

            // Statement form: for structs WITH Option fields (uses __entries.push())
            let try_into_xdr_stmt = if field_is_option {
                // For Option fields: only add entry if Some
                quote! {
                    if let Some(ref __inner) = val.#field_ident {
                        __entries.push(#path::xdr::ScMapEntry {
                            key: #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into(),
                            val: __inner.try_into().map_err(|_| #path::xdr::Error::Invalid)?,
                        });
                    }
                }
            } else {
                // For non-Option fields: always add entry
                quote! {
                    __entries.push(#path::xdr::ScMapEntry {
                        key: #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into(),
                        val: (&val.#field_ident).try_into().map_err(|_| #path::xdr::Error::Invalid)?,
                    });
                }
            };

            // Expression form: for structs WITHOUT Option fields (used in vec![...])
            let try_into_xdr_expr = quote! {
                #path::xdr::ScMapEntry {
                    key: #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into(),
                    val: (&val.#field_ident).try_into().map_err(|_| #path::xdr::Error::Invalid)?,
                }
            };

            (spec_field, field_ident, field_name, field_idx_lit, field_is_option, try_from_xdr, try_into_xdr_stmt, try_into_xdr_expr)
        })
        .multiunzip();

    // Check if struct has any Option fields
    let has_any_option_fields = field_is_options.iter().any(|&is_opt| is_opt);

    // Count required (non-Option) fields for validation
    let required_field_count: usize = field_is_options.iter().filter(|&&is_opt| !is_opt).count();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = if spec {
        let spec_entry = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: docs_from_attrs(attrs),
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: ident.to_string().try_into().unwrap(),
            fields: spec_fields.try_into().unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
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

    // Generate serialization and deserialization code based on whether struct has Option fields
    let (from_val_impl, to_val_impl) = if has_any_option_fields {
        // For structs WITH Option fields: use dynamic map building

        // Generate field extraction code for deserialization
        let field_extracts: Vec<_> = field_idents
            .iter()
            .zip(field_names.iter())
            .zip(field_is_options.iter())
            .map(|((field_ident, field_name), &is_option)| {
                if is_option {
                    // For Option fields: check if key exists, handle VOID for backwards compat
                    quote! {
                        #field_ident: {
                            let __sym = #path::Symbol::new(env, #field_name);
                            if __map.contains_key(__sym.clone()) {
                                let __v: Val = __map.get_unchecked(__sym);
                                if __v.is_void() {
                                    None // Backwards compat: VOID -> None
                                } else {
                                    Some(__v.try_into_val(env).map_err(|_| ConversionError)?)
                                }
                            } else {
                                None // Missing key -> None
                            }
                        }
                    }
                } else {
                    // For non-Option fields: key must exist
                    quote! {
                        #field_ident: {
                            let __sym = #path::Symbol::new(env, #field_name);
                            __map.get_unchecked(__sym).try_into_val(env).map_err(|_| ConversionError)?
                        }
                    }
                }
            })
            .collect();

        // Generate field insertion code for serialization
        let field_inserts: Vec<_> = field_idents
            .iter()
            .zip(field_names.iter())
            .zip(field_is_options.iter())
            .map(|((field_ident, field_name), &is_option)| {
                if is_option {
                    // For Option fields: only insert if Some
                    quote! {
                        if let Some(ref __inner) = val.#field_ident {
                            __map.set(
                                #path::Symbol::new(env, #field_name),
                                __inner.try_into_val(env).map_err(|_| ConversionError)?
                            );
                        }
                    }
                } else {
                    // For non-Option fields: always insert
                    quote! {
                        __map.set(
                            #path::Symbol::new(env, #field_name),
                            (&val.#field_ident).try_into_val(env).map_err(|_| ConversionError)?
                        );
                    }
                }
            })
            .collect();

        let from_val = quote! {
            impl #path::TryFromVal<#path::Env, #path::Val> for #ident {
                type Error = #path::ConversionError;
                fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                    use #path::{TryIntoVal, ConversionError, Map, Symbol, Val};
                    let __map: Map<Symbol, Val> = val.try_into_val(env).map_err(|_| ConversionError)?;
                    Ok(Self {
                        #(#field_extracts,)*
                    })
                }
            }
        };

        let to_val = quote! {
            impl #path::TryFromVal<#path::Env, #ident> for #path::Val {
                type Error = #path::ConversionError;
                fn try_from_val(env: &#path::Env, val: &#ident) -> Result<Self, #path::ConversionError> {
                    use #path::{TryIntoVal, ConversionError, Map, Symbol, Val};
                    let mut __map: Map<Symbol, Val> = Map::new(env);
                    #(#field_inserts)*
                    Ok(__map.into())
                }
            }
        };

        (from_val, to_val)
    } else {
        // For structs WITHOUT Option fields: use original optimized approach
        let from_val = quote! {
            impl #path::TryFromVal<#path::Env, #path::Val> for #ident {
                type Error = #path::ConversionError;
                fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                    use #path::{TryIntoVal,EnvBase,ConversionError,Val,MapObject};
                    const KEYS: [&'static str; #field_count_usize] = [#(#field_names),*];
                    let mut vals: [Val; #field_count_usize] = [Val::VOID.to_val(); #field_count_usize];
                    let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                    env.map_unpack_to_slice(map, &KEYS, &mut vals).map_err(|_| ConversionError)?;
                    Ok(Self {
                        #(#field_idents: vals[#field_idx_lits].try_into_val(env).map_err(|_| #path::ConversionError)?,)*
                    })
                }
            }
        };

        let to_val = quote! {
            impl #path::TryFromVal<#path::Env, #ident> for #path::Val {
                type Error = #path::ConversionError;
                fn try_from_val(env: &#path::Env, val: &#ident) -> Result<Self, #path::ConversionError> {
                    use #path::{TryIntoVal,EnvBase,ConversionError,Val};
                    const KEYS: [&'static str; #field_count_usize] = [#(#field_names),*];
                    let vals: [Val; #field_count_usize] = [
                        #((&val.#field_idents).try_into_val(env).map_err(|_| ConversionError)?),*
                    ];
                    Ok(env.map_new_from_slices(&KEYS, &vals).map_err(|_| ConversionError)?.into())
                }
            }
        };

        (from_val, to_val)
    };

    // Output.
    let mut output = quote! {
        #spec_gen

        #from_val_impl

        #to_val_impl

        impl #path::TryFromVal<#path::Env, &#ident> for #path::Val {
            type Error = #path::ConversionError;
            #[inline(always)]
            fn try_from_val(env: &#path::Env, val: &&#ident) -> Result<Self, #path::ConversionError> {
                <_ as #path::TryFromVal<#path::Env, #ident>>::try_from_val(env, *val)
            }
        }
    };

    // Additional output when testutils are enabled.
    if cfg!(feature = "testutils") {
        let arbitrary_tokens = crate::arbitrary::derive_arbitrary_struct(path, vis, ident, data);

        // Generate the map length check based on whether we have Option fields
        let map_len_check = if has_any_option_fields {
            // For structs with Option fields: require at least required_field_count, at most field_count_usize
            quote! {
                if map.len() < #required_field_count || map.len() > #field_count_usize {
                    return Err(#path::xdr::Error::Invalid);
                }
            }
        } else {
            // For structs without Option fields: exact count required
            quote! {
                if map.len() != #field_count_usize {
                    return Err(#path::xdr::Error::Invalid);
                }
            }
        };

        // Generate the TryFrom<&Struct> for ScMap based on whether we have Option fields
        let try_from_struct_for_scmap = if has_any_option_fields {
            // For structs with Option fields: build entries dynamically
            quote! {
                impl TryFrom<&#ident> for #path::xdr::ScMap  {
                    type Error = #path::xdr::Error;
                    #[inline(always)]
                    fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                        extern crate alloc;
                        use #path::TryFromVal;
                        let mut __entries = alloc::vec::Vec::new();
                        #(#try_into_xdr_stmts)*
                        #path::xdr::ScMap::sorted_from(__entries)
                    }
                }
            }
        } else {
            // For structs without Option fields: original vec! macro approach
            quote! {
                impl TryFrom<&#ident> for #path::xdr::ScMap  {
                    type Error = #path::xdr::Error;
                    #[inline(always)]
                    fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                        extern crate alloc;
                        use #path::TryFromVal;
                        #path::xdr::ScMap::sorted_from(alloc::vec![
                            #(#try_into_xdr_exprs,)*
                        ])
                    }
                }
            }
        };

        output.extend(quote!{
            impl #path::TryFromVal<#path::Env, #path::xdr::ScMap> for #ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScMap) -> Result<Self, #path::xdr::Error> {
                    use #path::xdr::Validate;
                    use #path::TryIntoVal;
                    let map = val;
                    #map_len_check
                    map.validate()?;
                    Ok(Self{
                        #(#try_from_xdrs,)*
                    })
                }
            }

            impl #path::TryFromVal<#path::Env, #path::xdr::ScVal> for #ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScVal) -> Result<Self, #path::xdr::Error> {
                    if let #path::xdr::ScVal::Map(Some(map)) = val {
                        <_ as #path::TryFromVal<_, _>>::try_from_val(env, map)
                    } else {
                        Err(#path::xdr::Error::Invalid)
                    }
                }
            }

            #try_from_struct_for_scmap

            impl TryFrom<#ident> for #path::xdr::ScMap {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: #ident) -> Result<Self, #path::xdr::Error> {
                    (&val).try_into()
                }
            }

            impl TryFrom<&#ident> for #path::xdr::ScVal  {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                    Ok(#path::xdr::ScVal::Map(Some(val.try_into()?)))
                }
            }

            impl TryFrom<#ident> for #path::xdr::ScVal {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: #ident) -> Result<Self, #path::xdr::Error> {
                    (&val).try_into()
                }
            }

            #arbitrary_tokens
        });
    }
    output
}
