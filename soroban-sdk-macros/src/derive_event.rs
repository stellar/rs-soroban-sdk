use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, DataStruct, Error, Ident, Path, Type};

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamKindV0, ScSpecEventParamV0, ScSpecEventV0,
    ScSpecTypeDef, StringM, WriteXdr,
};

use crate::{doc::docs_from_attrs, map_type::map_type, DEFAULT_XDR_RW_LIMITS};

#[derive(Clone, Debug, PartialEq, Eq)]
enum ParamKind {
    Topic,
    Data,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Param<'a> {
    kind: ParamKind,

    ident: &'a Ident,
    type_: &'a Type,

    spec: ScSpecEventParamV0,
}

pub fn derive_event(
    path: &Path,
    ident: &Ident,
    attrs: &[Attribute],
    struct_: &DataStruct,
    data_format: &str,
    lib: &Option<String>,
) -> TokenStream2 {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Vec::<Error>::new();

    // Data format.
    let data_format = match data_format {
        "single-value" => ScSpecEventDataFormat::SingleValue,
        "vec" => ScSpecEventDataFormat::Vec,
        "map" => ScSpecEventDataFormat::Map,
        _ => {
            errors.push(Error::new(
                ident.span(),
                format!(
                    r#"data_format {} must be one of: "single-value", "vec", or "map"."#,
                    data_format,
                ),
            ));
            ScSpecEventDataFormat::SingleValue
        }
    };

    // TODO: Figure out prefix topics. If no prefix topics are defined, set the prefix topics to
    // the snake_case version of the event struct name.

    // Map each field of the struct to a param for the event.
    let params = struct_
        .fields
        .iter()
        .map(|field| {
            let field_ident = field.ident.as_ref().unwrap();
            let field_name = field_ident.to_string();
            let field_type = &field.ty;
            let is_topic = field.attrs.iter().any(|a| a.path().is_ident("topic"));
            let param_spec = ScSpecEventParamV0 {
                kind: if is_topic {
                    ScSpecEventParamKindV0::Topic
                } else {
                    ScSpecEventParamKindV0::Data
                },
                doc: docs_from_attrs(&field.attrs),
                name: field_name.clone().try_into().unwrap_or_else(|_| {
                    const MAX: u32 = 30;
                    errors.push(Error::new(
                        field_ident.span(),
                        format!(
                            "event param name is too long: {}, max is {MAX}",
                            field_name.len()
                        ),
                    ));
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
            Param {
                kind: if is_topic {
                    ParamKind::Topic
                } else {
                    ParamKind::Data
                },
                ident: field_ident,
                type_: field_type,
                spec: param_spec,
            }
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
            data_format: data_format,
            doc: docs_from_attrs(attrs),
            lib: lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: ident.to_string().try_into().unwrap(),
            params: params
                .iter()
                .map(|f| f.spec.clone())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_EVENT_{}", ident.to_string().to_uppercase());
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
    let topics = fields.iter().map(|f| todo!()).collect::<Vec<_>>();
    let data_params = fields.iter().map(|f| todo!()).collect::<Vec<_>>();
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

    // Additional output when testutils are enabled for converting to and from the XDR
    // representation of the event.
    if cfg!(feature = "testutils") {
        output.extend(quote!{
            impl #path::TryFromVal<#path::Env, #path::xdr::ContractEventBody> for #ident {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from_val(env: &#path::Env, val: &#path::xdr::ContractEventBody) -> Result<Self, #path::xdr::Error> {
                    todo!()
                }
            }

            impl TryFrom<&#ident> for #path::xdr::ContractEventBody  {
                type Error = #path::xdr::Error;
                #[inline(always)]
                fn try_from(val: &#ident) -> Result<Self, #path::xdr::Error> {
                    todo!()
                }
            }
        });
    }

    output
}
