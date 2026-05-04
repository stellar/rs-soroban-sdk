use crate::{
    attribute::remove_attributes_from_item, default_crate_path, doc::docs_from_attrs,
    map_type::map_type, shaking, symbol, DEFAULT_XDR_RW_LIMITS,
};
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::curr::{
    ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0, ScSpecEventParamV0,
    ScSpecEventV0, ScSymbol, StringM, WriteXdr,
};
use syn::{ext::IdentExt as _, parse2, spanned::Spanned, Data, DeriveInput, Fields, LitStr, Path};

#[derive(Debug, FromMeta)]
struct ContractEventArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    lib: Option<String>,
    export: Option<bool>,
    #[darling(default)]
    topics: Option<Vec<LitStr>>,
    #[darling(default)]
    data_format: DataFormat,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum DataFormat {
    SingleValue,
    Vec,
    #[default]
    Map,
}

impl FromMeta for DataFormat {
    fn from_string(v: &str) -> Result<Self, Error> {
        match v {
            "single-value" => Ok(Self::SingleValue),
            "vec" => Ok(Self::Vec),
            "map" => Ok(Self::Map),
            _ => Err(Error::custom(format!(
                r#"data_format {v} must be one of: "single-value", "vec", or "map"."#
            ))),
        }
    }
}

impl From<DataFormat> for ScSpecEventDataFormat {
    fn from(data_format: DataFormat) -> Self {
        match data_format {
            DataFormat::SingleValue => ScSpecEventDataFormat::SingleValue,
            DataFormat::Vec => ScSpecEventDataFormat::Vec,
            DataFormat::Map => ScSpecEventDataFormat::Map,
        }
    }
}

pub fn derive_event(metadata: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match derive_event_or_err(metadata, input) {
        Ok(tokens) => tokens,
        Err(err) => err.write_errors(),
    }
}

fn derive_event_or_err(metadata: TokenStream2, input: TokenStream2) -> Result<TokenStream2, Error> {
    let args = NestedMeta::parse_meta_list(metadata.into())?;
    let args = ContractEventArgs::from_list(&args)?;
    let input = parse2::<DeriveInput>(input)?;
    let derived = derive_impls(&args, &input)?;
    let mut input = input;
    remove_attributes_from_item(&mut input.data, &["topic", "data"]);
    Ok(quote! {
        #input
        #derived
    }
    .into())
}

fn derive_impls(args: &ContractEventArgs, input: &DeriveInput) -> Result<TokenStream2, Error> {
    // Collect errors as they are encountered and emit them at the end.
    let mut errors = Error::accumulator();

    let ident = &input.ident;
    let (gen_impl, gen_types, gen_where) = input.generics.split_for_impl();
    let path = &args.crate_path;

    // Check event name length
    const EVENT_NAME_LENGTH: u32 = 32;
    let event_name = input.ident.unraw().to_string();
    let event_name_len = event_name.len();
    let event_name: StringM<EVENT_NAME_LENGTH> = errors
        .handle(event_name.try_into().map_err(|_| {
            Error::custom(format!(
                "event name has length {event_name_len} greater than length limit of {EVENT_NAME_LENGTH}"
            ))
            .with_span(&input.ident.span())
        }))
        .unwrap_or_default();

    let prefix_topics = if let Some(prefix_topics) = &args.topics {
        prefix_topics.iter().map(|t| t.value()).collect()
    } else {
        vec![input.ident.unraw().to_string().to_snake_case()]
    };

    let fields =
        match &input.data {
            Data::Struct(struct_) => match &struct_.fields {
                Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
                Fields::Unnamed(_) => Err(Error::custom(
                    "structs with unnamed fields are not supported as contract events",
                )
                .with_span(&struct_.fields.span()))?,
                Fields::Unit => Err(Error::custom(
                    "structs with no fields are not supported as contract events",
                )
                .with_span(&struct_.fields.span()))?,
            },
            Data::Enum(_) => Err(Error::custom("enums are not supported as contract events")
                .with_span(&input.span()))?,
            Data::Union(_) => Err(Error::custom("unions are not supported as contract events")
                .with_span(&input.span()))?,
        };

    // Map each field of the struct to a spec for a param, keeping the original Ident
    // alongside so it can still be used for `self.#ident` field access in the generated
    // code (raw identifiers like `r#type` need to stay raw for Rust, while the spec name
    // is the unraw form).
    let params_with_idents = fields
        .iter()
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let is_topic = field.attrs.iter().any(|a| a.path().is_ident("topic"));
            let location = if is_topic {
                ScSpecEventParamLocationV0::TopicList
            } else {
                ScSpecEventParamLocationV0::Data
            };
            let doc = docs_from_attrs(&field.attrs);
            const NAME_LENGTH: u32 = 30;
            let name = ident.unraw().to_string();
            let name_len = name.len();
            let name: StringM<NAME_LENGTH> = errors
                .handle(name.try_into().map_err(|_| {
                    Error::custom(format!(
                        "event field name has length {name_len} greater than length limit of {NAME_LENGTH}"
                    ))
                    .with_span(&field.ident.span())
                }))
                .unwrap_or_default();
            let type_ = errors
                .handle_in(|| Ok(map_type(&field.ty, true, false)?))
                .unwrap_or_default();
            let type_id_refs = shaking::type_id_refs(path, &field.ty);
            (
                ident.clone(),
                ScSpecEventParamV0 {
                    location,
                    doc,
                    name,
                    type_,
                },
                type_id_refs,
            )
        })
        .collect::<Vec<_>>();

    // If errors have occurred, return them.
    let mut errors = errors.checkpoint()?;

    // Generated code spec.
    let export = args.export.unwrap_or(true);
    let export_gen = if export {
        Some(quote! { #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")] })
    } else {
        None
    };
    let spec_entry = ScSpecEntry::EventV0(ScSpecEventV0 {
        data_format: args.data_format.into(),
        doc: docs_from_attrs(&input.attrs),
        lib: args.lib.as_deref().unwrap_or_default().try_into().unwrap(),
        name: ScSymbol(event_name),
        prefix_topics: prefix_topics
            .iter()
            .map(|t| t.try_into().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        params: params_with_idents
            .iter()
            .map(|(_, p, _)| p.clone())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    });
    let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
    let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
    let spec_xdr_len = spec_xdr.len();
    let spec_ident = format_ident!(
        "__SPEC_XDR_EVENT_{}",
        input.ident.unraw().to_string().to_uppercase()
    );
    let spec_shaking_marker = if export && cfg!(feature = "experimental_spec_shaking_v2") {
        Some(shaking::generate_marker_block(&spec_xdr))
    } else {
        None
    };
    let spec_shaking_graph = if export && cfg!(feature = "experimental_spec_shaking_v2") {
        let graph_ident = format_ident!(
            "__SPEC_GRAPH_EVENT_{}",
            input.ident.unraw().to_string().to_uppercase()
        );
        Some(shaking::generate_graph_record(
            path,
            &graph_ident,
            input.ident.span(),
            soroban_spec_markers::SpecGraphEntryKind::Event,
            &spec_xdr,
            params_with_idents
                .iter()
                .flat_map(|(_, _, refs)| refs.clone())
                .collect(),
        ))
    } else {
        None
    };

    // Generated code spec.
    let spec_gen = quote! {
        #export_gen
        pub static #spec_ident: [u8; #spec_xdr_len] = #ident::spec_xdr();

        impl #gen_impl #ident #gen_types #gen_where {
            pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                *#spec_xdr_lit
            }
        }

        #spec_shaking_graph
    };

    // Prepare Topics Conversion to Vec<Val>.
    let prefix_topics_symbols = prefix_topics.iter().map(|t| {
        symbol::short_or_long(
            &args.crate_path,
            quote!(env),
            &LitStr::new(&t, Span::call_site()),
        )
    });
    let topic_idents = params_with_idents
        .iter()
        .filter(|(_, p, _)| p.location == ScSpecEventParamLocationV0::TopicList)
        .map(|(ident, _, _)| ident.clone())
        .collect::<Vec<_>>();
    let topics_to_vec_val = quote! {
        use #path::IntoVal;
        (
            #(&#prefix_topics_symbols,)*
            #({ let v: #path::Val = self.#topic_idents.into_val(env); v },)*
        ).into_val(env)
    };

    // Prepare Data Conversion to Val.
    let data_params = params_with_idents
        .iter()
        .filter(|(_, p, _)| p.location == ScSpecEventParamLocationV0::Data)
        .collect::<Vec<_>>();
    let data_params_count = data_params.len();
    let data_idents = data_params
        .iter()
        .map(|(ident, _, _)| ident.clone())
        .collect::<Vec<_>>();
    let data_to_val = match args.data_format {
        DataFormat::SingleValue if data_params_count == 0 => quote! {
            #path::Val::VOID.to_val()
        },
        DataFormat::SingleValue => {
            if data_params_count > 1 {
                errors.push(Error::custom(
                    "data_format = \"single-value\" requires exactly 0 or 1 data fields, but found more",
                ));
            }
            quote! {
                use #path::IntoVal;
                #(self.#data_idents.into_val(env))*
            }
        }
        DataFormat::Vec if data_params_count == 0 => quote! {
            use #path::IntoVal;
            #path::Vec::<#path::Val>::new(env).into_val(env)
        },
        DataFormat::Vec => quote! {
            use #path::IntoVal;
            (
                #({ let v: #path::Val = self.#data_idents.into_val(env); v },)*
            ).into_val(env)
        },
        DataFormat::Map => {
            // Must be sorted for map_new_from_slices. Sort by the spec name (the
            // Soroban-facing Symbol string), and carry the original Ident alongside so
            // that `self.#ident` still uses the raw form where needed.
            let mut data_params_sorted = data_params.clone();
            data_params_sorted.sort_by_key(|(_, p, _)| p.name.to_string());
            let data_idents_sorted = data_params_sorted
                .iter()
                .map(|(ident, _, _)| ident.clone())
                .collect::<Vec<_>>();
            let data_strs_sorted = data_params_sorted
                .iter()
                .map(|(_, p, _)| p.name.to_string())
                .collect::<Vec<_>>();
            quote! {
                use #path::{EnvBase,IntoVal,unwrap::UnwrapInfallible};
                const KEYS: [&'static str; #data_params_count] = [#(#data_strs_sorted),*];
                let vals: [#path::Val; #data_params_count] = [
                    #(self.#data_idents_sorted.into_val(env)),*
                ];
                env.map_new_from_slices(&KEYS, &vals).unwrap_infallible().into()
            }
        }
    };

    // Output.
    let output = quote! {
        #spec_gen

        impl #gen_impl #path::Event for #ident #gen_types #gen_where {
            fn topics(&self, env: &#path::Env) -> #path::Vec<#path::Val> {
                #topics_to_vec_val
            }
            fn data(&self, env: &#path::Env) -> #path::Val {
                #data_to_val
            }
        }

        impl #gen_impl #ident #gen_types #gen_where {
            pub fn publish(&self, env: &#path::Env) {
                #spec_shaking_marker
                <_ as #path::Event>::publish(self, env);
            }
        }
    };

    errors.finish_with(output)
}
