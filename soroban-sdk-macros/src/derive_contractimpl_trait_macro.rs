use crate::{
    attribute::{is_attr_cfg, is_attr_cfg_attr, is_attr_doc, reject_items},
    default_crate_path,
};
use darling::{ast::NestedMeta, Error, FromMeta};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{
    ext::IdentExt as _, parse2, parse_quote, GenericArgument, ImplItemFn, ItemTrait, Path,
    PathArguments, ReturnType, Signature, TraitItem, TraitItemFn, Type, TypePath,
};

/// The root of the placeholder paths that stand in for the types of default
/// fns in the trait macro, e.g. `__contracttrait_type::Address`. The generated
/// code for the default fns replaces each with the type captured where the
/// trait is defined.
pub const TYPE_PLACEHOLDER: &str = "__contracttrait_type";

/// Name of the hidden trait, and of its associated type, that captures a type
/// of the trait's default fns where the trait is defined.
pub fn type_capture_ident(trait_name: &str, name: &Ident) -> Ident {
    format_ident!("__{trait_name}_{name}")
}

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
    let trait_macro = derive(&mut input);

    Ok(quote! {
        #trait_macro
        #input
    }
    .into())
}

/// Generates the trait machinery unconditionally, rolling any errors into the
/// generated code rather than emitting them in place of it, so that downstream
/// impls still resolve against the trait and its macro.
fn derive(input: &mut ItemTrait) -> TokenStream2 {
    let trait_ident = input.ident.clone();
    let trait_name = trait_ident.unraw().to_string();
    let mut captures = TypeCaptures::default();

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
            let mut sig = sig.clone();
            captures.capture_sig(&mut sig);
            fns.push(quote!(#(#doc_attrs)* #sig).to_token_stream().to_string());
        }
    }

    // Each type of the default fns is captured, where the trait is defined, in
    // the associated type of a hidden trait that is a supertrait of the trait,
    // so that wherever the trait is implemented the type can be named through
    // the trait, rather than by the name it has where the trait is defined.
    let vis = &input.vis;
    let capture_traits = captures.types.iter().map(|(name, ty)| {
        let ident = type_capture_ident(&trait_name, name);
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #vis trait #ident {
                type #ident;
            }
            impl<T: ?Sized> #ident for T {
                type #ident = #ty;
            }
        }
    });
    let capture_traits = quote!(#(#capture_traits)*);
    for (name, _) in &captures.types {
        let ident = type_capture_ident(&trait_name, name);
        input.colon_token.get_or_insert_with(Default::default);
        input.supertraits.push(parse_quote!(#ident));
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
                    trait_name = #trait_name,
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
        #capture_traits
        #output
        #error_tokens
    }
}

/// The distinct types of the default fns, each with the name of the
/// associated type that captures it.
#[derive(Default)]
struct TypeCaptures {
    types: Vec<(Ident, Type)>,
}

impl TypeCaptures {
    /// Replaces the types of the signature with placeholders for captured
    /// types. References are kept on the placeholder because the captured type
    /// cannot hold their lifetimes, and the error and ok types of a `Result`
    /// are captured separately because the client is generated from the
    /// `Result` in the signature.
    fn capture_sig(&mut self, sig: &mut Signature) {
        for input in sig.inputs.iter_mut() {
            if let syn::FnArg::Typed(pat_type) = input {
                self.capture(&mut pat_type.ty);
            }
        }
        if let ReturnType::Type(_, ty) = &mut sig.output {
            let args = result_args(ty);
            if args.is_empty() {
                self.capture(ty);
            } else {
                args.into_iter().for_each(|t| self.capture(t));
            }
        }
    }

    fn capture(&mut self, ty: &mut Type) {
        if let Type::Reference(r) = ty {
            return self.capture(&mut r.elem);
        }
        // Types that refer to `Self` are left in place, because `Self` only
        // has meaning within the trait and its impls.
        let mentions_self = ty
            .to_token_stream()
            .into_iter()
            .any(|t| matches!(t, proc_macro2::TokenTree::Ident(i) if i == "Self"));
        if mentions_self {
            return;
        }
        let name = self.name(ty);
        let placeholder = format_ident!("{TYPE_PLACEHOLDER}");
        *ty = parse_quote!(#placeholder::#name);
    }

    /// The name of the captured type, which is the last segment of its path,
    /// so that the generated code can still recognize types by that name, e.g.
    /// `Env` and `MuxedAddress`.
    fn name(&mut self, ty: &Type) -> Ident {
        let ty_str = ty.to_token_stream().to_string();
        if let Some((name, _)) = self
            .types
            .iter()
            .find(|(_, t)| t.to_token_stream().to_string() == ty_str)
        {
            return name.clone();
        }
        let base = match ty {
            Type::Path(TypePath { path, .. }) => path.segments.last().map(|s| s.ident.unraw()),
            _ => None,
        }
        .unwrap_or_else(|| format_ident!("Type"));
        let mut name = base.clone();
        let mut i = 1;
        while self.types.iter().any(|(n, _)| *n == name) {
            name = format_ident!("{base}_{i}");
            i += 1;
        }
        self.types.push((name.clone(), ty.clone()));
        name
    }
}

/// The ok and error types of a `Result`, or none if the type is not a `Result`.
fn result_args(ty: &mut Type) -> Vec<&mut Type> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return Vec::new();
    };
    let Some(segment) = path.segments.last_mut() else {
        return Vec::new();
    };
    let PathArguments::AngleBracketed(args) = &mut segment.arguments else {
        return Vec::new();
    };
    if segment.ident != "Result" || args.args.len() != 2 {
        return Vec::new();
    }
    args.args
        .iter_mut()
        .filter_map(|a| match a {
            GenericArgument::Type(t) => Some(t),
            _ => None,
        })
        .collect()
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
