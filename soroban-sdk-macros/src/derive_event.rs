use itertools::Itertools;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct, Error, Ident, Path, Visibility};

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEntry, ScSpecTypeDef, ScSpecEventFieldV0, ScSpecEventV0,
    StringM, WriteXdr,
};

use crate::{doc::docs_from_attrs, map_type::map_type, DEFAULT_XDR_RW_LIMITS};

pub fn derive_event(
    path: &Path,
    vis: &Visibility,
    ident: &Ident,
    attrs: &[Attribute],
    struct_: &DataStruct,
    data_format: &str,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();
    let fields = &struct_.fields;
    let field_count_usize: usize = fields.len();
    let fields = fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_type = field.ty;
            let is_topic = field.attrs.iter().any(|a| a.path().is_ident("topic"));
            let field_spec = ScSpecEventFieldV0 {
                doc: docs_from_attrs(&field.attrs),
                name: field_name.clone().try_into().unwrap_or_else(|_| {
                    const MAX: u32 = 30;
                    errors.push(Error::new(field_ident.span(), format!("event field name is too long: {}, max is {MAX}", field_name.len())));
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
            (is_topic, field_ident, field_type, field_spec)
        })
        .collect::<Vec<_>>();

    // If errors have occurred, render them instead.
    if !errors.is_empty() {
        let compile_errors = errors.iter().map(Error::to_compile_error);
        return quote! { #(#compile_errors)* };
    }

    // Generated code spec.
    let spec_gen = {
        let spec_entry = ScSpecEntry::EventV0(ScSpecEventV0 {
            doc: docs_from_attrs(attrs),
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: ident.to_string().try_into().unwrap(),
            topics: spec_fields.try_into().unwrap(),
            data_format: ScSpecEventDataFormat::SingleValue,
            data: spec_fields.try_into().unwrap(),
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

        impl #path::Event for #ident {
            type Topics = ();
            type Data = ();
            fn topics(&self) -> &Self::Topics {
                todo!()
            }
            fn data(&self) -> &Self::Data {
                todo!()
            }
        }
    };

    output
}
