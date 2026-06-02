use crate::{
    attribute::{is_attr_cfg, is_attr_cfg_attr, is_attr_doc, reject_items},
    default_crate_path, syn_ext,
};
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, parse2, ImplItemFn, ItemTrait, Path, TraitItem, TraitItemFn, Type};

// See soroban-sdk/docs/contracttrait.md for documentation on how this works.

#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default = "default_crate_path")]
    crate_path: Path,
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
    let input = parse2(input)?;
    let trait_macro = derive(&args, &input);

    Ok(quote! {
        #trait_macro
        #input
    }
    .into())
}

/// Generates the trait machinery unconditionally, rolling any errors into the
/// generated code rather than emitting them in place of it, so that downstream
/// impls still resolve against the trait and its macro.
fn derive(args: &Args, input: &ItemTrait) -> TokenStream2 {
    let path = syn_ext::path_in_macro_rules(&args.crate_path);

    let trait_ident = &input.ident;

    let mut errors: Option<syn::Error> = None;
    let mut fns = Vec::new();
    for i in &input.items {
        if let TraitItem::Fn(TraitItemFn {
            default: Some(_),
            sig,
            attrs,
            ..
        }) = i
        {
            if let Err(err) = reject_items(
                attrs.iter().filter(|a| is_attr_cfg(a) || is_attr_cfg_attr(a)),
                "`cfg` and `cfg_attr` are not supported on `#[contracttrait]` default functions because they would be evaluated where the default implementation is generated, not where the trait is defined",
            ) {
                match errors {
                    Some(ref mut acc) => acc.combine(err),
                    None => errors = Some(err),
                }
            }
            let doc_attrs: Vec<_> = attrs.iter().filter(|a| is_attr_doc(a)).collect();
            fns.push(quote!(#(#doc_attrs)* #sig).to_token_stream().to_string());
        }
    }

    let macro_ident = macro_ident(&input.ident);

    let output = quote! {
        #[doc(hidden)]
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! #macro_ident {
            (
                $trait_ident:path,
                $impl_ident:ty,
                $impl_fns:expr,
                $client_name:literal,
                $args_name:literal,
                $spec_name:literal $(,)?
            ) => {
                #path::contractimpl_trait_default_fns_not_overridden!(
                    trait_ident = $trait_ident,
                    trait_default_fns = [#(#fns),*],
                    impl_ident = $impl_ident,
                    impl_fns = $impl_fns,
                    client_name = $client_name,
                    args_name = $args_name,
                    spec_name = $spec_name,
                );
            }
        }

        /// Macro for `contractimpl`ing the default functions of the trait that are not overridden.
        pub use #macro_ident as #trait_ident;
    };

    let error_tokens = errors.map(|err| err.to_compile_error());
    quote! {
        #output
        #error_tokens
    }
}

pub fn generate_call_to_contractimpl_for_trait(
    trait_ident: &Path,
    impl_ident: &Type,
    pub_methods: &[ImplItemFn],
    client_ident: &str,
    args_ident: &str,
    spec_ident: &str,
) -> Result<TokenStream2, syn::Error> {
    for method in pub_methods {
        reject_items(
            method.attrs.iter().filter(|a| is_attr_cfg_attr(a)),
            "`cfg_attr` is not supported on `#[contractimpl(contracttrait)]` methods because the generated helper only supports direct `cfg` attrs for default override matching",
        )?;
    }
    let impl_fns = pub_methods.iter().map(|f| {
        let cfg_attrs = f.attrs.iter().filter(|attr| is_attr_cfg(attr));
        let ident = &f.sig.ident;
        // Serialize only the cfg attrs and method name. The generated
        // `contractimpl_trait_default_fns_not_overridden!` helper parses this as
        // function-shaped metadata for override matching.
        quote!(#(#cfg_attrs)* fn #ident())
            .to_token_stream()
            .to_string()
    });
    Ok(quote! {
        #trait_ident!(
            #trait_ident,
            #impl_ident,
            [#(#impl_fns),*],
            #client_ident,
            #args_ident,
            #spec_ident,
        );
    })
}

fn macro_ident(trait_ident: &Ident) -> Ident {
    let lower = trait_ident.unraw().to_string().to_snake_case();
    format_ident!("__contractimpl_for_{lower}")
}
