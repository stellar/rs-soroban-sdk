use crate::{
    default_crate_path,
    derive_args::derive_args_impl,
    derive_client::derive_client_impl,
    derive_fn::{derive_contract_function_registration_ctor, derive_pub_fns},
    derive_spec_fn::derive_fns_spec,
    syn_ext::{self, ident_to_type},
};
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use std::{collections::HashMap, iter};
use syn::{
    ext::IdentExt as _, Attribute, GenericArgument, GenericParam, Generics, LitStr, Path,
    PathArguments, Type,
};

// See soroban-sdk/docs/contracttrait.md for documentation on how this works.

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    trait_ident: Path,
    trait_default_fns: Vec<LitStr>,
    trait_generics: Option<LitStr>,
    impl_ident: Ident,
    impl_fns: Vec<LitStr>,
    client_name: String,
    args_name: String,
    spec_name: Type,
    spec_export: Option<bool>,
}

pub fn derive_contractimpl_trait_default_fns_not_overridden(
    metadata: TokenStream2,
) -> TokenStream2 {
    match derive_or_err(metadata) {
        Ok(tokens) => tokens,
        Err(err) => err.write_errors(),
    }
}

fn derive_or_err(metadata: TokenStream2) -> Result<TokenStream2, Error> {
    let args = NestedMeta::parse_meta_list(metadata.into())?;
    let args = Args::from_list(&args)?;
    let derived = derive(&args)?;
    Ok(quote! {
        #derived
    }
    .into())
}

fn derive(args: &Args) -> Result<TokenStream2, Error> {
    let impl_ty = ident_to_type(args.impl_ident.clone());
    let trait_ident = &args.trait_ident;
    let spec_export = args.spec_export.unwrap_or(true);

    let mut trait_default_fns = syn_ext::strs_to_fns(&args.trait_default_fns)?;
    if let Some(trait_generics) = &args.trait_generics {
        let type_params = trait_type_params(&trait_generics.parse()?, trait_ident);
        for f in &mut trait_default_fns {
            syn_ext::fn_substitute_type_params(f, &type_params);
        }
    }
    let impl_fns = syn_ext::strs_to_fns(&args.impl_fns)?;

    // Filter the list of default fns down to only default fns that have not been redefined /
    // overridden in the input fns.
    let fns = trait_default_fns
        .into_iter()
        .filter_map(|mut f| {
            let matching_impl_fns = impl_fns
                .iter()
                .filter(|impl_fn| impl_fn.ident.unraw() == f.ident.unraw())
                .collect::<Vec<_>>();
            if matching_impl_fns.is_empty() {
                return Some(Ok(f));
            }

            for impl_fn in matching_impl_fns {
                match cfg_condition(&impl_fn.attrs) {
                    Ok(Some(cfg)) => {
                        f.attrs.push(syn::parse_quote!(#[cfg(not(#cfg))]));
                    }
                    Ok(None) => return None,
                    Err(err) => return Some(Err(err)),
                }
            }

            Some(Ok(f))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // With no default fns left there is nothing to generate, and the empty
    // registration ctor would otherwise clash with an empty impl's own.
    if fns.is_empty() {
        return Ok(quote! {});
    }

    let mut output = quote! {};
    output.extend(derive_pub_fns(
        &args.crate_path,
        &impl_ty,
        &fns,
        Some(trait_ident),
        &args.client_name,
    ));
    output.extend(derive_fns_spec(
        &args.crate_path,
        &args.spec_name,
        &fns,
        spec_export,
    ));
    output.extend(derive_client_impl(
        &args.crate_path,
        &args.client_name,
        &fns,
    ));
    output.extend(derive_args_impl(&args.args_name, &fns));
    output.extend(derive_contract_function_registration_ctor(
        &args.crate_path,
        &impl_ty,
        Some(trait_ident),
        &fns,
    ));

    Ok(output)
}

/// Maps the trait's generic type params to the args the impl gives them, or to
/// their defaults where the impl leaves them out, such as `T` to `u32` for
/// `Echo<u32>`.
fn trait_type_params(generics: &Generics, trait_ident: &Path) -> HashMap<Ident, Type> {
    let args = trait_ident
        .segments
        .last()
        .into_iter()
        .flat_map(|s| match &s.arguments {
            PathArguments::AngleBracketed(args) => args.args.iter().collect(),
            _ => vec![],
        })
        .filter(|a| !matches!(a, GenericArgument::Lifetime(_)))
        .map(Some)
        .chain(iter::repeat(None));
    generics
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Lifetime(_)))
        .zip(args)
        .filter_map(|(param, arg)| match (param, arg) {
            (GenericParam::Type(param), Some(GenericArgument::Type(ty))) => {
                Some((param.ident.clone(), ty.clone()))
            }
            (GenericParam::Type(param), None) => param
                .default
                .as_ref()
                .map(|ty| (param.ident.clone(), ty.clone())),
            _ => None,
        })
        .collect()
}

fn cfg_condition(attrs: &[Attribute]) -> Result<Option<TokenStream2>, Error> {
    let cfgs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg"))
        .map(|attr| attr.parse_args::<TokenStream2>().map_err(Error::from))
        .collect::<Result<Vec<_>, Error>>()?;

    Ok(match cfgs.as_slice() {
        [] => None,
        [cfg] => Some(quote! { #cfg }),
        _ => Some(quote! { all(#(#cfgs),*) }),
    })
}
