use crate::{
    default_crate_path, default_export,
    derive_args::derive_args_impl,
    derive_client::derive_client_impl,
    derive_fn::{derive_contract_function_registration_ctor, derive_pub_fns},
    derive_spec_fn::derive_fns_spec,
    syn_ext::{self, ident_to_type},
};
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{LitStr, Path, Type};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    trait_ident: Ident,
    trait_default_fns: Vec<LitStr>,
    impl_ident: Ident,
    impl_fns: Vec<LitStr>,
    client_name: String,
    args_name: String,
    spec_name: Type,
    #[darling(default = "default_export")]
    spec_export: bool,
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
    let spec_export = args.spec_export;

    let trait_default_fns = syn_ext::strs_to_fns(&args.trait_default_fns)?;
    let impl_fns = syn_ext::strs_to_fns(&args.impl_fns)?;

    // Filter the list of default fns down to only default fns that have not been redefined /
    // overridden in the input fns.
    let fns = trait_default_fns
        .into_iter()
        .filter(|f| !impl_fns.iter().any(|o| f.ident == o.ident))
        .collect::<Vec<_>>();

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
        fns.iter().map(|f| &f.ident),
    ));

    Ok(output)
}
