use crate::{
    default_crate_path,
    derive_args::derive_args_impl,
    derive_client::derive_client_impl,
    derive_contractimpl_trait_macro::fn_types,
    derive_fn::{derive_contract_function_registration_ctor, derive_pub_fns},
    derive_spec_fn::derive_fns_spec,
    syn_ext::{self, ident_to_type, ty_to_safe_ident_str},
};
use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{ext::IdentExt as _, parse_quote, Attribute, LitStr, Path, Type, TypePath};

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

    // The types named in the default fns' signatures are written as the trait
    // sees them, and may resolve to different types, or not at all, here where
    // the trait is implemented. Name them instead through the associated types
    // of the trait's fn types supertrait, which carry the types as the trait
    // resolves them. The associated types are only reachable through the trait
    // in a generic context, so each is projected through a local trait
    // implemented for the impl type. Each type gets its own local trait, so
    // that its associated type can keep the name of the type it replaces, as
    // the generated code recognizes some types, such as `Env` and
    // `MuxedAddress`, by name.
    let trait_str = trait_ident
        .segments
        .iter()
        .map(|s| s.ident.unraw().to_string())
        .collect::<Vec<_>>()
        .join("_");
    let impl_ty_str = ty_to_safe_ident_str(&impl_ty);
    let mut output = quote! {};
    let fns: Vec<_> = fns
        .into_iter()
        .map(|mut f| {
            for (ident, ty) in fn_types(&mut f) {
                let types_ident = format_ident!("__{impl_ty_str}__{trait_str}{ident}");
                let name = type_name(ty);
                output.extend(quote! {
                    #[doc(hidden)]
                    #[allow(non_camel_case_types)]
                    pub trait #types_ident {
                        type #name;
                    }

                    impl<T: #trait_ident + ?Sized> #types_ident for T {
                        type #name = T::#ident;
                    }
                });
                *ty = parse_quote!(<#impl_ty as #types_ident>::#name);
            }
            f
        })
        .collect();

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

/// The name of the type, the last segment of its path, or `Type` for a type
/// without one, such as a tuple, or whose name can't name an associated type,
/// such as `Self`.
fn type_name(ty: &Type) -> Ident {
    match ty {
        Type::Path(TypePath { path, .. }) => path
            .segments
            .last()
            .map(|s| s.ident.clone())
            .filter(|i| i != "Self")
            .unwrap_or_else(|| format_ident!("Type")),
        _ => format_ident!("Type"),
    }
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
