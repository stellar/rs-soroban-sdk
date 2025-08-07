use crate::default_crate_path;
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{parse2, ImplItemFn, ItemTrait, Path, TraitItem, TraitItemFn, Type};

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
}

pub fn derive_trait_macro(metadata: TokenStream2, input: TokenStream2) -> TokenStream2 {
    match derive_or_err(metadata, input) {
        Ok(tokens) => tokens,
        Err(err) => err.write_errors(),
    }
}

fn derive_or_err(metadata: TokenStream2, input: TokenStream2) -> Result<TokenStream2, Error> {
    let args = NestedMeta::parse_meta_list(metadata.into())?;
    let args = Args::from_list(&args)?;
    let input = parse2(input)?;

    let trait_macro = derive(&args, &input)?;

    Ok(quote! {
        #trait_macro
        #input
    }
    .into())
}

fn derive(args: &Args, input: &ItemTrait) -> Result<TokenStream2, Error> {
    let path = &args.crate_path;

    let trait_ident = &input.ident;

    let fns = input.items.iter().filter_map(|i| match i {
        TraitItem::Fn(TraitItemFn {
            default: Some(_),
            sig,
            ..
        }) => Some(sig.to_token_stream().to_string()),
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
