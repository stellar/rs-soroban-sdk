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
use syn::{parse_quote, Attribute, LitStr, Path, Type};

// See soroban-sdk/docs/contracttrait.md for documentation on how this works.

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    trait_ident: Path,
    trait_default_fns: Vec<LitStr>,
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

    let trait_default_fns = syn_ext::strs_to_fns(&args.trait_default_fns)?;
    let impl_fns = syn_ext::strs_to_fns(&args.impl_fns)?;

    // Filter the list of default fns down to only default fns that have not been redefined /
    // overridden in the input fns.
    let fns = trait_default_fns
        .into_iter()
        .filter_map(|mut f| {
            // No matching impl means keep the default. An unconditional impl drops it.
            // Cfg-gated impls only override the default when their cfg is active, so the
            // default is kept behind `not(cfg)` to remain available when the impl is absent.
            let impl_fns = impl_fns
                .iter()
                .filter(|impl_fn| impl_fn.ident == f.ident)
                .collect::<Vec<_>>();
            if impl_fns.is_empty() {
                return Some(Ok(f));
            }

            let mut cfgs = Vec::new();
            for impl_fn in impl_fns {
                match cfg_condition(&impl_fn.attrs) {
                    Ok(Some(cfg)) => cfgs.push(cfg),
                    Ok(None) => return None,
                    Err(e) => return Some(Err(e)),
                }
            }
            f.attrs
                .extend(cfgs.into_iter().map(|cfg| parse_quote!(#[cfg(not(#cfg))])));
            Some(Ok(f))
        })
        .collect::<Result<Vec<_>, Error>>()?;
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
    output.extend(derive_fns_spec(&args.spec_name, &fns, spec_export));
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

fn cfg_condition(attrs: &[Attribute]) -> Result<Option<TokenStream2>, Error> {
    let cfgs = attrs
        .iter()
        .filter(|a| a.path().is_ident("cfg"))
        .map(|a| a.parse_args::<TokenStream2>().map_err(Error::from))
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(match cfgs.as_slice() {
        [] => None,
        [cfg] => Some(quote! { #cfg }),
        _ => Some(quote! { all(#(#cfgs),*) }),
    })
}
