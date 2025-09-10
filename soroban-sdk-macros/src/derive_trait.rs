use crate::default_crate_path;
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse2, ItemTrait, Path};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
    add_impl_type: Option<bool>,
    spec_name: Option<String>,
    args_name: Option<String>,
    client_name: Option<String>,
}

pub fn derive_trait(metadata: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match derive_or_err(metadata, input) {
        Ok(tokens) => tokens,
        Err(err) => err.write_errors(),
    }
}

fn derive_or_err(metadata: TokenStream2, input: TokenStream2) -> Result<TokenStream2, Error> {
    let args = NestedMeta::parse_meta_list(metadata.into())?;
    let args = Args::from_list(&args)?;
    let input: ItemTrait = parse2(input)?;

    let path = &args.crate_path;
    let spec_name = args.spec_name.unwrap_or(format!("{}Spec", input.ident));
    let spec_ident = format_ident!("{spec_name}");
    let args_name = args.args_name.unwrap_or(format!("{}Args", input.ident));
    let client_name = args.client_name.unwrap_or(format!("{}Client", input.ident));
    let add_impl_type = args.add_impl_type.unwrap_or_default();

    Ok(quote! {
        pub struct #spec_ident;
        #[#path::contractspecfn(name = #spec_name, export = false)]
        #[#path::contractargs(name = #args_name)]
        #[#path::contractclient(crate_path = #path, name = #client_name)]
        #[#path::contractimpl_trait_macro(crate_path = #path, add_impl_type = #add_impl_type)]
        #input
    }
    .into())
}
