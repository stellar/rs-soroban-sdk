use itertools::Itertools;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, Attribute, DataStruct, Error, Ident, Path, Visibility};

use stellar_xdr::{ScSpecTypeDef, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM};

use crate::{
    doc::docs_from_attrs,
    map_type::{const_view_string, const_view_type_def, map_type, spec_name_gen},
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
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();
    let fields = &data.fields;
    let field_count_usize: usize = fields.len();
    let (spec_fields, field_idents, field_names, field_idx_lits, field_types, try_from_xdrs, try_into_xdrs): (Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .sorted_by_key(|field| field.ident.as_ref().unwrap().unraw().to_string())
        .enumerate()
        .map(|(field_num, field)| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.unraw().to_string();
            let field_idx_lit = Literal::usize_unsuffixed(field_num);
            let field_type = &field.ty;
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
            let try_from_xdr = quote! {
                #field_ident: {
                    let key: #path::xdr::ScVal = #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into();
                    // Fields absent from the map are void, so that they convert
                    // to None for Option fields.
                    let val = match map.binary_search_by_key(&key, |entry| entry.key.clone()) {
                        Ok(idx) => map[idx].val.clone(),
                        Err(_) => #path::xdr::ScVal::Void,
                    };
                    let rv: #path::Val = (&val).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                }
            };
            let try_into_xdr = quote! {
                #path::xdr::ScMapEntry {
                    key: #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into(),
                    val: (&val.#field_ident).try_into().map_err(|_| #path::xdr::Error::Invalid)?,
                }
            };
            (spec_field, field_ident, field_name, field_idx_lit, field_type, try_from_xdr, try_into_xdr)
        })
        .multiunzip();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Build the spec entry once.
    let spec = ScSpecUdtStructV0 {
        doc: docs_from_attrs(attrs),
        // set to empty string always because the field is no longer used
        lib: StringM::default(),
        name: ident.unraw().to_string().try_into().unwrap(),
        fields: spec_fields.try_into().unwrap(),
    };

    // The fully qualified name the spec knows this type by, emitted for every
    // type so that a reference to it from anywhere can reach it.
    let spec_name = spec_name_gen(ident, None, None, None);

    // Generated code spec. The spec entry is rendered as the equivalent const
    // ScSpecEntryView, which the contract crate encodes to XDR at compile time.
    let spec_gen = {
        let doc = const_view_string(path, &spec.doc);
        let lib = const_view_string(path, &spec.lib);
        let name = quote!(#path::xdr::StringMView::try_from_str_or_panic(#ident::spec_name()));
        // Each field's Rust type, so a reference to a user-defined type in a
        // field resolves to the name that type reports for itself.
        let fields = spec
            .fields
            .iter()
            .zip(field_types.iter().copied())
            .map(|(f, rust)| {
            let doc = const_view_string(path, &f.doc);
            let name = const_view_string(path, &f.name);
                let type_ = const_view_type_def(path, &f.type_, Some(rust));
                quote!(#path::xdr::ScSpecUdtStructFieldV0View { doc: #doc, name: #name, type_: #type_ })
            });
        let spec_view = quote! {
            #path::xdr::ScSpecEntryView::UdtStructV0(#path::xdr::ScSpecUdtStructV0View {
                doc: #doc,
                lib: #lib,
                name: #name,
                fields: #path::xdr::VecMView::try_from_slice_or_panic(&[#(#fields),*]),
            })
        };
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[doc(hidden)]
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #ident::spec_xdr_len()] = #ident::spec_xdr();

            impl #ident {
                const __SPEC_XDR_ENTRY: #path::xdr::ScSpecEntryView<'static> = #spec_view;

                pub const fn spec_xdr_len() -> usize {
                    const { #ident::__SPEC_XDR_ENTRY.const_xdr_len() }
                }

                pub const fn spec_xdr() -> [u8; #ident::spec_xdr_len()] {
                    const { #ident::__SPEC_XDR_ENTRY.const_to_xdr() }
                }
            }
        }
    };

    // Output.
    let mut output = quote! {
        #spec_name

        #spec_gen

        impl #path::TryFromVal<#path::Env, #path::Val> for #ident {
            type Error = #path::ConversionError;
            fn try_from_val(env: &#path::Env, val: &#path::Val) -> Result<Self, #path::ConversionError> {
                use #path::{TryIntoVal,EnvBase,ConversionError,Val,MapObject};
                const KEYS: [&'static str; #field_count_usize] = [#(#field_names),*];
                let mut vals: [Val; #field_count_usize] = [Val::VOID.to_val(); #field_count_usize];
                let map: MapObject = val.try_into().map_err(|_| ConversionError)?;
                env.sparse_map_unpack_to_slice(map, &KEYS, &mut vals).map_err(|_| ConversionError)?;
                Ok(Self {
                    #(#field_idents: vals[#field_idx_lits].try_into_val(env).map_err(|_| #path::ConversionError)?,)*
                })
            }
        }

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
        output.extend(quote!{
            impl #path::TryFromVal<#path::Env, #path::xdr::ScMap> for #ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ScMap) -> Result<Self, #path::xdr::Error> {
                    use #path::xdr::Validate;
                    use #path::TryIntoVal;
                    let map = val;
                    // The host requires the keys of a struct's map be symbols,
                    // and traps otherwise, so reject them here too.
                    if map.iter().any(|entry| !matches!(entry.key, #path::xdr::ScVal::Symbol(_))) {
                        return Err(#path::xdr::Error::Invalid);
                    }
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

            impl TryFrom<&#ident> for #path::xdr::ScMap  {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                    extern crate alloc;
                    use #path::TryFromVal;
                    #path::xdr::ScMap::sorted_from(alloc::vec![
                        #(#try_into_xdrs,)*
                    ])
                }
            }

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
