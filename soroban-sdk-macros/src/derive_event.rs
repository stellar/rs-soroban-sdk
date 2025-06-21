use crate::{
    attribute::remove_attributes_from_item, default_crate_path, doc::docs_from_attrs,
    map_type::map_type, symbol, DEFAULT_XDR_RW_LIMITS,
};
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use stellar_xdr::curr::{
    ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0, ScSpecEventParamV0,
    ScSpecEventV0, WriteXdr,
};
use syn::{parse2, spanned::Spanned, Data, DeriveInput, Fields, LitStr, Path};

#[derive(Debug, FromMeta)]
struct ContractEventArgs {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    lib: Option<String>,
    export: Option<bool>,
    #[darling(default)]
    data_format: DataFormat,
    #[darling(default)]
    prefix_topics: Option<Vec<LitStr>>,
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

impl Into<ScSpecEventDataFormat> for DataFormat {
    fn into(self) -> ScSpecEventDataFormat {
        match self {
            Self::SingleValue => ScSpecEventDataFormat::SingleValue,
            Self::Vec => ScSpecEventDataFormat::Vec,
            Self::Map => ScSpecEventDataFormat::Map,
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
    let path = &args.crate_path;

    let prefix_topics = if let Some(prefix_topics) = &args.prefix_topics {
        prefix_topics.iter().map(|t| t.value()).collect()
    } else {
        vec![input.ident.to_string().to_snake_case()]
    };

    let fields =
        match &input.data {
            Data::Struct(struct_) => match &struct_.fields {
                Fields::Named(fields) => fields.named.iter(),
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

    // Map each field of the struct to a spec for a param.
    let params = fields
        .map(|field| {
            let ident = field.ident.as_ref().unwrap();
            let is_topic = field.attrs.iter().any(|a| a.path().is_ident("topic"));
            let location = if is_topic {
                ScSpecEventParamLocationV0::TopicList
            } else {
                ScSpecEventParamLocationV0::Data
            };
            let doc = docs_from_attrs(&field.attrs);
            let name = errors
                .handle(ident.to_string().try_into().map_err(|_| {
                    Error::custom("event field name is too long").with_span(&field.ident.span())
                }))
                .unwrap_or_default();
            let type_ = errors
                .handle_in(|| Ok(map_type(&field.ty, false)?))
                .unwrap_or_default();
            ScSpecEventParamV0 {
                location,
                doc,
                name,
                type_,
            }
        })
        .collect::<Vec<_>>();

    // If errors have occurred, return them.
    let errors = errors.checkpoint()?;

    // Generated code spec.

    let export = args.export.unwrap_or(true);
    let spec_gen = if export {
        let spec_entry = ScSpecEntry::EventV0(ScSpecEventV0 {
            data_format: args.data_format.into(),
            doc: docs_from_attrs(&input.attrs),
            lib: args.lib.as_deref().unwrap_or_default().try_into().unwrap(),
            name: input.ident.to_string().try_into().unwrap(),
            prefix_topics: prefix_topics
                .iter()
                .map(|t| t.try_into().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            params: params
                .iter()
                .map(|p| p.clone())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        });
        let spec_xdr = spec_entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!(
            "__SPEC_XDR_EVENT_{}",
            input.ident.to_string().to_uppercase()
        );
        let ident = &input.ident;
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

    // Prepare Topics Conversion to Vec<Val>.
    let prefix_topics_symbols = prefix_topics.iter().map(|t| {
        symbol::short_or_long(
            &args.crate_path,
            quote!(env),
            &LitStr::new(&t, Span::call_site()),
        )
    });
    let topic_idents = params
        .iter()
        .filter(|p| p.location == ScSpecEventParamLocationV0::TopicList)
        .map(|p| format_ident!("{}", p.name.to_string()))
        .collect::<Vec<_>>();
    let topics_to_vec_val = quote! {
        (
            #(&#prefix_topics_symbols,)*
            #(&self.#topic_idents,)*
        ).into_val(env)
    };

    // Prepare Data Conversion to Val.
    let data_params = params
        .iter()
        .filter(|p| p.location == ScSpecEventParamLocationV0::Data)
        .collect::<Vec<_>>();
    let data_params_count = data_params.len();
    let data_idents = data_params
        .iter()
        .map(|p| format_ident!("{}", p.name.to_string()))
        .collect::<Vec<_>>();
    let data_strs = data_idents
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>();
    let data_to_val = match args.data_format {
        DataFormat::SingleValue if data_params_count == 0 => {
            quote! {
                #path::Val::VOID.to_val()
            }
        }
        DataFormat::SingleValue => {
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
            (#(&self.#data_idents,)*).into_val(env)
        },
        DataFormat::Map => quote! {
            use #path::{EnvBase,IntoVal,unwrap::UnwrapInfallible};
            const KEYS: [&'static str; #data_params_count] = [#(#data_strs),*];
            let vals: [#path::Val; #data_params_count] = [
                #(self.#data_idents.into_val(env)),*
            ];
            env.map_new_from_slices(&KEYS, &vals).unwrap_infallible().into()
        },
    };

    // Output.
    let output = quote! {
        #spec_gen

        impl #path::Event for #ident {
            fn topics(&self, env: &#path::Env) -> #path::Vec<#path::Val> {
                #topics_to_vec_val
            }
            fn data(&self, env: &#path::Env) -> #path::Val {
                #data_to_val
            }
        }
    };

    errors.finish_with(output)
}
