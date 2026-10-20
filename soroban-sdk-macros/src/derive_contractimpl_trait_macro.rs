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
    let capture_traits = captures.types.iter().map(|Capture { name, params, ty }| {
        let ident = type_capture_ident(&trait_name, name);
        quote! {
            #[doc(hidden)]
            #[allow(non_camel_case_types)]
            #vis trait #ident {
                type #ident<#(#params),*>;
            }
            impl<__T: ?Sized> #ident for __T {
                type #ident<#(#params),*> = #ty;
            }
        }
    });
    let capture_traits = quote!(#(#capture_traits)*);
    for Capture { name, .. } in &captures.types {
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

/// A type of the default fns, captured where the trait is defined. A generic
/// type is captured without its generic arguments, as a generic associated type
/// with a parameter for each argument, and each argument is captured on its
/// own, so that the type keeps its arguments wherever it is named, e.g.
/// `Vec<Point>`.
struct Capture {
    /// The name of the associated type, which is the last segment of the
    /// type's path, so that docs show the type's name and the generated code
    /// can still recognize the `Env` by its name.
    name: Ident,
    /// The parameters of the associated type, one for each generic argument.
    params: Vec<TokenStream2>,
    /// The type, with its generic arguments replaced by the parameters.
    ty: Type,
}

/// The distinct types of the default fns.
#[derive(Default)]
struct TypeCaptures {
    types: Vec<Capture>,
}

impl TypeCaptures {
    /// Replaces the types of the signature with placeholders for captured
    /// types. References and tuples are kept on the placeholder because the
    /// captured type cannot hold the lifetimes of references, and so that docs
    /// show them.
    fn capture_sig(&mut self, sig: &mut Signature) {
        for input in sig.inputs.iter_mut() {
            if let syn::FnArg::Typed(pat_type) = input {
                self.capture(&mut pat_type.ty);
            }
        }
        if let ReturnType::Type(_, ty) = &mut sig.output {
            self.capture(ty);
        }
    }

    fn capture(&mut self, ty: &mut Type) {
        match ty {
            Type::Reference(r) => return self.capture(&mut r.elem),
            Type::Tuple(t) => return t.elems.iter_mut().for_each(|e| self.capture(e)),
            _ => {}
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
        let placeholder = format_ident!("{TYPE_PLACEHOLDER}");
        // A generic type has its arguments captured on their own, unless it has
        // arguments other than types and consts, in which case it is captured
        // whole.
        if let Type::Path(TypePath { qself: None, path }) = ty {
            let segment = path.segments.last_mut().unwrap();
            if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                let generic = args
                    .args
                    .iter()
                    .all(|a| matches!(a, GenericArgument::Type(_) | GenericArgument::Const(_)));
                if generic {
                    let mut params = Vec::new();
                    let mut param_args = Vec::new();
                    for (i, arg) in args.args.iter_mut().enumerate() {
                        match arg {
                            GenericArgument::Type(t) => {
                                self.capture(t);
                                let p = format_ident!("T{i}");
                                params.push(quote!(#p));
                                param_args.push(quote!(#p));
                            }
                            _ => {
                                // Const generics of contract types are lengths.
                                let p = format_ident!("N{i}");
                                params.push(quote!(const #p: usize));
                                param_args.push(quote!(#p));
                            }
                        }
                    }
                    let args = args.clone();
                    let mut base = path.clone();
                    base.segments.last_mut().unwrap().arguments = PathArguments::None;
                    let name = self.name(parse_quote!(#base<#(#param_args),*>), params);
                    *ty = parse_quote!(#placeholder::#name #args);
                    return;
                }
            }
        }
        let name = self.name(ty.clone(), Vec::new());
        *ty = parse_quote!(#placeholder::#name);
    }

    fn name(&mut self, ty: Type, params: Vec<TokenStream2>) -> Ident {
        let ty_str = ty.to_token_stream().to_string();
        if let Some(c) = self
            .types
            .iter()
            .find(|c| c.ty.to_token_stream().to_string() == ty_str)
        {
            return c.name.clone();
        }
        let base = match &ty {
            Type::Path(TypePath { path, .. }) => path.segments.last().map(|s| s.ident.unraw()),
            _ => None,
        }
        .unwrap_or_else(|| format_ident!("Type"));
        let mut name = base.clone();
        let mut i = 1;
        while self.types.iter().any(|c| c.name == name) {
            name = format_ident!("{base}_{i}");
            i += 1;
        }
        self.types.push(Capture {
            name: name.clone(),
            params,
            ty,
        });
        name
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
