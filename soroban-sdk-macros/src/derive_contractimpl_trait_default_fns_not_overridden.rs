use crate::{
    default_crate_path, derive_args::derive_args_impl, derive_client::derive_client_impl,
    derive_fn::derive_pub_fn, derive_spec_fn::derive_fn_spec, syn_ext,
};
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{LitStr, Path, Type};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    trait_ident: Ident,
    trait_default_fns: Vec<LitStr>,
    internal_fns: Vec<LitStr>,
    impl_ident: Ident,
    impl_fns: Vec<LitStr>,
    client_name: String,
    args_name: String,
    spec_name: Type,
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
    let ident = &args.impl_ident;
    let trait_ident = &args.trait_ident;

    let trait_default_fns = syn_ext::strs_to_signatures(&args.trait_default_fns);
    let impl_fns = syn_ext::strs_to_signatures(&args.impl_fns);
    let internal_fns = syn_ext::strs_to_signatures(&args.internal_fns);

    // Filter the list of default fns down to only default fns that have not been redefined /
    // overridden in the input fns.
    let fns = trait_default_fns
        .into_iter()
        .filter(|f| {
            !impl_fns
                .iter()
                .chain(internal_fns.iter())
                .any(|o| f.ident == o.ident)
        })
        .collect::<Vec<_>>();

    let mut output = quote! {};
    for f in &fns {
        output.extend(derive_pub_fn(
            &args.crate_path,
            ident.to_token_stream(),
            &f.ident,
            &[],
            &f.inputs,
            Some(trait_ident),
            &args.client_name,
        ));
        output.extend(derive_fn_spec(
            &args.spec_name,
            &f.ident,
            &[],
            &f.inputs,
            &f.output,
            true, // TODO: pass down the 'export' parameter
        ));
    }
    output.extend(derive_client_impl(
        &args.crate_path,
        &args.client_name,
        fns.iter()
            .map(Into::into)
            .collect::<Vec<syn_ext::Fn>>()
            .as_slice(),
    ));
    output.extend(derive_args_impl(
        &args.args_name,
        fns.iter()
            .map(Into::into)
            .collect::<Vec<syn_ext::Fn>>()
            .as_slice(),
    ));

    Ok(output)
}
