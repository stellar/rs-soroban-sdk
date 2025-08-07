use crate::{
    default_crate_path,
    syn_ext::{self, fn_arg_ident},
};
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{parse2, parse_quote, ImplItemFn, ItemTrait, Path, TraitItem, TraitItemFn, Type};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    add_impl_type: Option<bool>,
}

pub fn derive_contractimpl_trait_macro(
    metadata: TokenStream2,
    input: TokenStream2,
) -> TokenStream2 {
    match derive_or_err(metadata, input) {
        Ok(tokens) => tokens,
        Err(err) => err.write_errors(),
    }
}

fn derive_or_err(metadata: TokenStream2, input: TokenStream2) -> Result<TokenStream2, Error> {
    let args = NestedMeta::parse_meta_list(metadata.into())?;
    let args = Args::from_list(&args)?;

    let mut input = parse2(input)?;
    if args.add_impl_type.unwrap_or_default() {
        maybe_add_default_methods(&mut input)?;
        maybe_add_impl_type(&mut input);
    }

    let trait_macro = derive(&args, &input)?;
    remove_internal_attrs(&mut input);

    Ok(quote! {
        #trait_macro
        #input
    }
    .into())
}

fn derive(args: &Args, input: &ItemTrait) -> Result<TokenStream2, Error> {
    let path = syn_ext::path_in_macro_rules(&args.crate_path);

    let trait_ident = &input.ident;

    let mut internal_fns = Vec::new();

    let fns = input.items.iter().filter_map(|i| match i {
        TraitItem::Fn(TraitItemFn {
            default: Some(_),
            sig,
            attrs,
            ..
        }) => {
            if has_attr(&attrs, "internal") {
                internal_fns.push(sig.to_token_stream().to_string());
                None
            } else {
                Some(sig.to_token_stream().to_string())
            }
        }
        _ => None,
    });

    let macro_ident = macro_ident(&input.ident);

    let output = quote! {
        #[doc(hidden)]
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! #macro_ident {
            (
                $impl_ident:ty,
                $impl_fns:expr,
                $client_name:literal,
                $args_name:literal,
                $spec_name:literal $(,)?
            ) => {
                #path::contractimpl_trait_default_fns_not_overridden!(
                    trait_ident = #trait_ident,
                    trait_default_fns = [#(#fns),*],
                    impl_ident = $impl_ident,
                    impl_fns = $impl_fns,
                    internal_fns = [#(#internal_fns),*],
                    client_name = $client_name,
                    args_name = $args_name,
                    spec_name = $spec_name,
                );
            }
        }

        /// Macro for `contractimpl`ing the default functions of the trait that are not overriden.
        pub use #macro_ident as #trait_ident;
    };

    Ok(output)
}

pub fn generate_call_to_contractimpl_for_trait(
    trait_ident: &Ident,
    impl_ident: &Type,
    pub_methods: &Vec<&ImplItemFn>,
    client_ident: &str,
    args_ident: &str,
    spec_ident: &str,
) -> TokenStream2 {
    let impl_fns = pub_methods
        .iter()
        .map(|f| f.sig.to_token_stream().to_string());
    quote! {
        #trait_ident!(
            #impl_ident,
            [#(#impl_fns),*],
            #client_ident,
            #args_ident,
            #spec_ident,
        );
    }
}

fn macro_ident(trait_ident: &Ident) -> Ident {
    let lower = trait_ident.to_string().to_snake_case();
    format_ident!("__contractimpl_for_{lower}")
}

fn remove_internal_attrs(trait_: &mut ItemTrait) {
    trait_.items.iter_mut().for_each(|item| {
        if let TraitItem::Fn(func) = item {
            func.attrs.retain(|attr| !attr.path().is_ident("internal"))
        }
    });
}

fn maybe_add_default_methods(trait_: &mut ItemTrait) -> Result<(), syn::Error> {
    trait_
        .items
        .iter_mut()
        .filter_map(|item| match item {
            TraitItem::Fn(func) => Some(func),
            _ => None,
        })
        .try_for_each(|func| -> Result<(), syn::Error> {
            if func.default.is_none() {
                let args = func
                    .sig
                    .inputs
                    .iter()
                    .map(fn_arg_ident)
                    .collect::<Result<Vec<_>, _>>()?;
                let name = func.sig.ident.clone();
                func.default = Some(parse_quote! {
                    {
                        Self::Impl::#name(#(#args),*)
                    }
                });
            }
            Ok(())
        })?;
    Ok(())
}

fn maybe_add_impl_type(trait_: &mut ItemTrait) {
    if !trait_.items.iter().any(syn_ext::is_trait_item_type) {
        let trait_ident = trait_.ident.clone();
        trait_.items.insert(
            0,
            syn::parse_quote! {
                type Impl: #trait_ident;
            },
        );
    }
}

pub(crate) fn has_attr(attrs: &[syn::Attribute], ident_str: &str) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident(ident_str))
}
