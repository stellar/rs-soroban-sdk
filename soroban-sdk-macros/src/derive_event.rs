use itertools::Itertools;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct, Error, Ident, Path, Visibility};

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecEventTopicV0, ScSpecEventDataV0, ScSpecUdtStructV0,
    StringM, WriteXdr,
};

use crate::{doc::docs_from_attrs, map_type::map_type, DEFAULT_XDR_RW_LIMITS};

pub fn derive_event(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    attrs: &[Attribute],
    data: &DataStruct,
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
            let spec_field = ScSpecEventV0 {
                doc: docs_from_attrs(&field.attrs),
                name: field_name.clone().try_into().unwrap_or_else(|_| {
                    const MAX: u32 = 30;
                    errors.push(Error::new(field_ident.span(), format!("struct field name is too long: {}, max is {MAX}", field_name.len())));
                    StringM::<MAX>::default()
                }),
                type_: match map_type(&field.ty, false) {
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
                    let idx = map.binary_search_by_key(&key, |entry| entry.key.clone()).map_err(|_| #path::xdr::Error::Invalid)?;
                    let rv: #path::Val = (&map[idx].val.clone()).try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?;
                    rv.try_into_val(env).map_err(|_| #path::xdr::Error::Invalid)?
                }
            };
            let try_into_xdr = quote! {
                #path::xdr::ScMapEntry {
                    key: #path::xdr::ScSymbol(#field_name.try_into().map_err(|_| #path::xdr::Error::Invalid)?).into(),
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
    let spec_gen = {
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
    };

    // Output.
    let mut output = quote! {
        #spec_gen

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

    output
}
