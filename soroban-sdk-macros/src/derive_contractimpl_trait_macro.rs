use crate::{
    attribute::{is_attr_cfg, is_attr_cfg_attr, is_attr_doc, reject_items},
    default_crate_path, syn_ext,
};
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{
    ext::IdentExt as _, parse2, parse_quote, FnArg, GenericArgument, ImplItemFn, ItemTrait, Path,
    PathArguments, ReturnType, TraitItem, TraitItemFn, Type, TypePath, TypeReference,
};

// See soroban-sdk/docs/contracttrait.md for documentation on how this works.

#[derive(Debug, FromMeta)]
struct Args {
    // Unused, accepted for backwards compatibility. The generated trait macro
    // uses the crate_path passed by `#[contractimpl]` instead.
    #[allow(dead_code)]
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
    Args::from_list(&args)?;
    let mut input = parse2(input)?;
    let trait_macro = derive(&input);
    let fn_types = derive_fn_types(&mut input);

    Ok(quote! {
        #trait_macro
        #fn_types
        #input
    }
    .into())
}

/// Generates a hidden supertrait of the trait whose associated types are the
/// types named in the signatures of the trait's default fns, as the trait
/// resolves them where it is defined. The code generated for the default fns
/// where the trait is implemented names the types through these associated
/// types, because the type names written in the signatures may resolve to
/// different types, or not at all, where the trait is implemented.
fn derive_fn_types(input: &mut ItemTrait) -> TokenStream2 {
    let types: Vec<_> = input
        .items
        .iter()
        .filter_map(|i| match i {
            TraitItem::Fn(TraitItemFn {
                default: Some(_),
                sig,
                ..
            }) => Some(sig),
            _ => None,
        })
        .flat_map(|sig| {
            let mut f = syn_ext::Fn {
                ident: sig.ident.clone(),
                attrs: vec![],
                inputs: sig.inputs.clone(),
                output: sig.output.clone(),
            };
            fn_types(&mut f)
                .into_iter()
                .map(|(ident, ty)| (ident, ty.clone()))
                .collect::<Vec<_>>()
        })
        .collect();
    if types.is_empty() {
        return quote! {};
    }

    let trait_ident = &input.ident;
    let vis = &input.vis;
    let types_ident = format_ident!("__{}FnTypes", trait_ident.unraw());
    let (idents, tys): (Vec<_>, Vec<_>) = types.into_iter().unzip();

    input.colon_token.get_or_insert_with(Default::default);
    input.supertraits.push(parse_quote!(#types_ident));

    quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #vis trait #types_ident {
            #(type #idents;)*
        }

        impl<T: #trait_ident + ?Sized> #types_ident for T {
            #(type #idents = #tys;)*
        }
    }
}

/// The types in the fn's signature that are captured as associated types of
/// the trait's fn types supertrait, with the associated type each is captured
/// as. The types are those behind any references, so that replacing them keeps
/// inputs passed by reference.
pub fn fn_types(f: &mut syn_ext::Fn) -> Vec<(Ident, &mut Type)> {
    let fn_name = f.ident.unraw();
    let mut types = Vec::new();

    for (i, input) in f.inputs.iter_mut().enumerate() {
        if let FnArg::Typed(pat_type) = input {
            types.push((
                format_ident!("__{fn_name}__arg_{i}"),
                unref_mut(&mut pat_type.ty),
            ));
        }
    }

    if let ReturnType::Type(_, ty) = &mut f.output {
        // Checked before borrowing the ok and error types, because the borrow
        // checker would otherwise hold that borrow over the fallback too.
        if result_types_mut(ty).is_none() {
            types.push((format_ident!("__{fn_name}__ret"), unref_mut(ty)));
        } else if let Some((ok, err)) = result_types_mut(ty) {
            types.push((format_ident!("__{fn_name}__ok"), unref_mut(ok)));
            types.push((format_ident!("__{fn_name}__err"), unref_mut(err)));
        }
    }

    types
}

/// The Rust type behind any number of references.
fn unref_mut(t: &mut Type) -> &mut Type {
    match t {
        Type::Reference(TypeReference { elem, .. }) => unref_mut(elem),
        t => t,
    }
}

/// The ok and error types of a `Result`, identified the same way that the
/// client identifies them, by name.
fn result_types_mut(t: &mut Type) -> Option<(&mut Type, &mut Type)> {
    let Type::Path(TypePath { qself: None, path }) = t else {
        return None;
    };
    let segment = path.segments.last_mut()?;
    if segment.ident.unraw() != "Result" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &mut segment.arguments else {
        return None;
    };
    if args.args.len() != 2 {
        return None;
    }
    let mut args = args.args.iter_mut();
    match (args.next()?, args.next()?) {
        (GenericArgument::Type(ok), GenericArgument::Type(err)) => Some((ok, err)),
        _ => None,
    }
}

/// Generates the trait machinery unconditionally, rolling any errors into the
/// generated code rather than emitting them in place of it, so that downstream
/// impls still resolve against the trait and its macro.
fn derive(input: &ItemTrait) -> TokenStream2 {
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
                [$($crate_path:tt)+],
                $trait_ident:path,
                $impl_ident:ty,
                $impl_fns:expr,
                $client_name:literal,
                $args_name:literal,
                $spec_name:literal $(,)?
            ) => {
                $($crate_path)+::contractimpl_trait_default_fns_not_overridden!(
                    crate_path = $($crate_path)+,
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
    crate_path: &Path,
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
            [#crate_path],
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
