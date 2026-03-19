use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    AngleBracketedGenericArguments, Attribute, GenericArgument, LitStr, Path, PathArguments,
    PathSegment, ReturnType, Signature, Token, TypePath,
};
use syn::{
    spanned::Spanned, token::And, Error, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, ItemTrait,
    Lifetime, Pat, PatIdent, PatType, TraitItem, TraitItemFn, Type, TypeReference, Visibility,
};

/// Gets methods from the implementation that have public visibility. For
/// methods that are inherently implemented this is methods that have a pub
/// visibility keyword. For methods that are implementing a trait the pub is
/// assumed and so all methods are returned.
pub fn impl_pub_methods(imp: &ItemImpl) -> Vec<ImplItemFn> {
    imp.items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Fn(m) => Some(m.clone()),
            _ => None,
        })
        .filter(|m| imp.trait_.is_some() || matches!(m.vis, Visibility::Public(_)))
        .collect()
}

/// Gets methods from the trait.
pub fn trait_methods(imp: &ItemTrait) -> impl Iterator<Item = &TraitItemFn> {
    imp.items.iter().filter_map(|i| match i {
        TraitItem::Fn(m) => Some(m),
        _ => None,
    })
}

/// Returns the ident of the function argument, if it has one.
pub fn fn_arg_ident(arg: &FnArg) -> Result<Ident, Error> {
    if let FnArg::Typed(pat_type) = arg {
        if let Pat::Ident(pat_ident) = *pat_type.pat.clone() {
            return Ok(pat_ident.ident);
        }
    }
    Err(Error::new(
        arg.span(),
        "argument in this form is not supported, use simple named arguments only",
    ))
}

/// Validate that the function argument type is not a mutable reference.
///
/// Returns an error indicating that contract functions arguments cannot be a mutable reference.
/// Even though reference (&) are supported, mutable references (&mut) are not, because it is
/// semanatically confusing for a contract function to receive an external input that it looks like
/// it could mutate.
pub fn fn_arg_type_validate_no_mut(ty: &Type) -> Result<(), Error> {
    match ty {
        Type::Reference(TypeReference { mutability: Some(_), .. }) => {
            Err(Error::new(ty.span(), "mutable references (&mut) are not supported in contract function parameters, use immutable references (&) instead"))
        }
        _ => Ok(()),
    }
}

/// Modifies a Pat removing any 'mut' on an Ident.
pub fn pat_unwrap_mut(p: Pat) -> Pat {
    match p {
        Pat::Ident(PatIdent {
            attrs,
            by_ref,
            mutability: Some(_),
            ident,
            subpat,
        }) => Pat::Ident(PatIdent {
            attrs,
            by_ref,
            mutability: None,
            ident,
            subpat,
        }),
        _ => p,
    }
}

/// Unwraps a reference, returning the type within the reference.
///
/// If the type is not a reference, returns the type as-is.
pub fn type_unwrap_ref(t: Type) -> Type {
    match t {
        Type::Reference(TypeReference { elem, .. }) => *elem,
        _ => t,
    }
}

/// Returns a clone of the type from the FnArg, converted into an immutable reference to the type
/// with the given lifetime.
pub fn fn_arg_ref_type(arg: &FnArg, lifetime: Option<&Lifetime>) -> Result<Type, Error> {
    if let FnArg::Typed(pat_type) = arg {
        Ok(Type::Reference(TypeReference {
            and_token: And::default(),
            lifetime: lifetime.cloned(),
            mutability: None,
            elem: Box::new(type_unwrap_ref(*pat_type.ty.clone())),
        }))
    } else {
        Err(Error::new(
            arg.span(),
            "argument in this form is not supported, use simple named arguments only",
        ))
    }
}

/// Returns a clone of FnArg, converted into an immutable reference with the given lifetime.
/// Mutability from the ident is stripped.
pub fn fn_arg_make_ref(arg: &FnArg, lifetime: Option<&Lifetime>) -> FnArg {
    if let FnArg::Typed(pat_type) = arg {
        return FnArg::Typed(PatType {
            attrs: pat_type.attrs.clone(),
            pat: Box::new(pat_unwrap_mut(*pat_type.pat.clone())),
            colon_token: pat_type.colon_token,
            ty: Box::new(Type::Reference(TypeReference {
                and_token: And::default(),
                lifetime: lifetime.cloned(),
                mutability: None,
                elem: Box::new(type_unwrap_ref(*pat_type.ty.clone())),
            })),
        });
    }
    arg.clone()
}

/// Returns a clone of FnArg with the type as an Into if the arg is a typed
/// arg. Mutability from the ident is stripped.
pub fn fn_arg_make_into(arg: &FnArg) -> FnArg {
    if let FnArg::Typed(pat_type) = arg {
        let ty = &pat_type.ty;
        return FnArg::Typed(PatType {
            attrs: pat_type.attrs.clone(),
            pat: Box::new(pat_unwrap_mut(*pat_type.pat.clone())),
            colon_token: pat_type.colon_token,
            ty: Box::new(syn::parse_quote! { impl Into<#ty> }),
        });
    }
    arg.clone()
}

pub enum HasFnsItem {
    Trait(ItemTrait),
    Impl(ItemImpl),
}

impl HasFnsItem {
    pub fn name(&'_ self) -> String {
        match self {
            HasFnsItem::Trait(t) => t.ident.to_string(),
            HasFnsItem::Impl(i) => {
                let ty = &i.self_ty;
                quote!(#ty).to_string()
            }
        }
    }

    pub fn fns(&self) -> Vec<Fn> {
        match self {
            HasFnsItem::Trait(t) => trait_methods(t)
                .map(|m| Fn {
                    ident: m.sig.ident.clone(),
                    attrs: m.attrs.clone(),
                    inputs: m.sig.inputs.clone(),
                    output: m.sig.output.clone(),
                })
                .collect(),
            HasFnsItem::Impl(i) => impl_pub_methods(i)
                .iter()
                .map(|m| Fn {
                    ident: m.sig.ident.clone(),
                    attrs: m.attrs.clone(),
                    inputs: m.sig.inputs.clone(),
                    output: m.sig.output.clone(),
                })
                .collect(),
        }
    }
}

impl Parse for HasFnsItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.call(Attribute::parse_outer);
        _ = input.parse::<Token![pub]>();
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![trait]) {
            let t = input.parse()?;
            Ok(HasFnsItem::Trait(t))
        } else if lookahead.peek(Token![impl]) {
            let mut imp = input.parse()?;
            flatten_associated_items_in_impl_fns(&mut imp)?;
            Ok(HasFnsItem::Impl(imp))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for HasFnsItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HasFnsItem::Trait(t) => t.to_tokens(tokens),
            HasFnsItem::Impl(i) => i.to_tokens(tokens),
        }
    }
}

pub struct Fn {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    pub inputs: Punctuated<FnArg, Comma>,
    pub output: ReturnType,
}

impl Fn {
    pub fn output(&self) -> Type {
        let t = match &self.output {
            ReturnType::Default => quote!(()),
            ReturnType::Type(_, typ) => match unpack_result(typ) {
                Some((t, _)) => quote!(#t),
                None => quote!(#typ),
            },
        };
        Type::Verbatim(t)
    }
    pub fn try_output(&self, crate_path: &Path) -> Type {
        let (t, e) = match &self.output {
            ReturnType::Default => (quote!(()), quote!(#crate_path::Error)),
            ReturnType::Type(_, typ) => match unpack_result(typ) {
                Some((t, e)) => (quote!(#t), quote!(#e)),
                None => (quote!(#typ), quote!(#crate_path::Error)),
            },
        };
        Type::Verbatim(quote! {
            Result<
                Result<#t, <#t as #crate_path::TryFromVal<#crate_path::Env, #crate_path::Val>>::Error>,
                Result<#e, #crate_path::InvokeError>
            >
        })
    }
}

impl From<&ImplItemFn> for Fn {
    fn from(m: &ImplItemFn) -> Self {
        Self {
            ident: m.sig.ident.clone(),
            attrs: m.attrs.clone(),
            inputs: m.sig.inputs.clone(),
            output: m.sig.output.clone(),
        }
    }
}

impl Parse for Fn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let sig: Signature = input.parse()?;
        Ok(Fn {
            ident: sig.ident.clone(),
            attrs,
            inputs: sig.inputs.clone(),
            output: sig.output.clone(),
        })
    }
}

/// Converts a Vec<LitStr> containing "attrs + signature" strings into Vec<Fn>.
pub fn strs_to_fns(fn_strs: &[LitStr]) -> Result<Vec<Fn>, Error> {
    fn_strs
        .iter()
        .map(|f| {
            syn::parse_str::<Fn>(&f.value())
                .map_err(|e| Error::new(f.span(), format!("failed to parse function: {e}")))
        })
        .collect()
}

fn unpack_result(typ: &Type) -> Option<(Type, Type)> {
    match &typ {
        Type::Path(TypePath { path, .. }) => {
            if let Some(PathSegment {
                ident,
                arguments:
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
            }) = path.segments.last()
            {
                let args = args.iter().collect::<Vec<_>>();
                match (&ident.to_string()[..], args.as_slice()) {
                    ("Result", [GenericArgument::Type(t), GenericArgument::Type(e)]) => {
                        Some((t.clone(), e.clone()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn flatten_associated_items_in_impl_fns(imp: &mut ItemImpl) -> Result<(), Error> {
    // TODO: Flatten associated consts used in functions.
    // Flatten associated types used in functions.
    let associated_types: HashMap<Ident, Type> = imp
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Type(i) => Some((i.ident.clone(), i.ty.clone())),
            _ => None,
        })
        .collect();

    // Resolve Self::* in function input types and return types, including
    // inside generic arguments like Vec<Self::Val>, Result<Self::Val, Error>, or &Self::Val.
    // Uses default 128 depth limit for types. This is a somewhat arbitrary limit if it needs
    // to be increased in the future.
    for item in imp.items.iter_mut() {
        if let ImplItem::Fn(f) = item {
            for input in f.sig.inputs.iter_mut() {
                if let FnArg::Typed(t) = input {
                    resolve_self_types(&mut t.ty, &associated_types, 128)?;
                }
            }
            if let ReturnType::Type(_, ty) = &mut f.sig.output {
                resolve_self_types(ty, &associated_types, 128)?;
            }
        }
    }

    Ok(())
}

/// Recursively resolve `Self::Ident` types within a type, including inside
/// generic arguments like `Vec<Self::Val>`, `Result<Self::Val, Error>`, or `&Self::Val`.
///
/// ### Errors
/// If we cannot resolve the type or any unresolved `Self::Ident` remains after resolution.
fn resolve_self_types(
    ty: &mut Type,
    associated_types: &HashMap<Ident, Type>,
    depth: usize,
) -> Result<(), Error> {
    if depth == 0 {
        return Err(Error::new(
            ty.span(),
            "unable to resolve type; type depth limit exceeded",
        ));
    }

    if let Some(ident) = self_type_ident(ty)? {
        if let Some(resolved) = associated_types.get(ident).cloned() {
            *ty = resolved;
            return resolve_self_types(ty, associated_types, depth - 1);
        }
        return Err(Error::new(
            ty.span(),
            format!("unresolved associated type `Self::{ident}`; use a concrete type instead"),
        ));
    }

    match ty {
        // Reject qualified Self paths like `<Self as Trait>::Foo`.
        Type::Path(TypePath { qself: Some(qself), .. })
            if matches!(qself.ty.as_ref(), Type::Path(TypePath { qself: None, path }) if path.is_ident("Self")) =>
        {
            Err(Error::new(
                ty.span(),
                "qualified associated types like `<Self as Trait>::Type` are not supported; use a concrete type instead",
            ))
        }
        // Recurse into generic arguments of path types.
        Type::Path(TypePath { path, .. }) => {
            for segment in path.segments.iter_mut() {
                if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    for arg in args.args.iter_mut() {
                        if let GenericArgument::Type(inner_ty) = arg {
                            resolve_self_types(inner_ty, associated_types, depth - 1)?;
                        }
                    }
                }
            }
            Ok(())
        }
        // Recurse into reference types like &Self::Val.
        Type::Reference(TypeReference { elem, .. }) => {
            resolve_self_types(elem, associated_types, depth - 1)
        }
        _ => Ok(()),
    }
}

/// If the type is `Self::Ident`, return the `Ident`. Otherwise return `None`.
///
/// ### Errors
/// If the type is a generic associated type like `Self::Foo<T>`.
fn self_type_ident(ty: &Type) -> Result<Option<&Ident>, Error> {
    if let Type::Path(TypePath { qself: None, path }) = ty {
        let segments = &path.segments;
        if segments.len() == 2
            && segments.first() == Some(&PathSegment::from(format_ident!("Self")))
        {
            if let Some(seg) = segments.get(1) {
                return match seg.arguments {
                    PathArguments::None => Ok(Some(&seg.ident)),
                    _ => Err(Error::new(
                        path.span(),
                        format!("generic associated types like `Self::{}<..>` are not supported; use a concrete type instead", seg.ident),
                    )),
                };
            }
        }
    }
    Ok(None)
}

pub fn ty_to_safe_ident_str(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "").replace(':', "_")
}

pub fn ident_to_type(ident: Ident) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::from_iter([PathSegment {
                ident,
                arguments: PathArguments::None,
            }]),
        },
    })
}

/// Converts a path for use inside a declarative macro_rules.
///
/// If the first segment of the path is `crate`, converts it to `$crate`, otherwise returns the
/// path unaltered.
///
/// The return value is a TokenStream because while $crate is a special token that acts as a path
/// in a macro_rules it is not a valid identifier and syn's Ident type, used in Path, does not
/// permit it.
pub fn path_in_macro_rules(p: &Path) -> TokenStream {
    let leading_colon = &p.leading_colon;
    let mut segments = p.segments.iter();
    let first = segments.next();
    if leading_colon == &None
        && first
            == Some(&PathSegment {
                ident: Ident::new("crate", Span::call_site()),
                arguments: PathArguments::None,
            })
    {
        quote! { $crate #(::#segments)* }
    } else {
        quote! { #leading_colon #first #(::#segments)* }
    }
}

#[cfg(test)]
mod test_path_in_macro_rules {
    use crate::syn_ext::*;
    use quote::quote;
    use syn::parse2;

    fn assert_paths_eq(input: TokenStream, expected: TokenStream) {
        assert_eq!(
            path_in_macro_rules(&parse2(input).unwrap()).to_string(),
            expected.to_string(),
        );
    }

    #[test]
    fn test_unaltered_paths() {
        let input = quote!(path::to::module);
        let expected = quote!(path::to::module);
        assert_paths_eq(input, expected);
    }

    #[test]
    fn test_unaltered_global_paths() {
        let input = quote!(::path::to::module);
        let expected = quote!(::path::to::module);
        assert_paths_eq(input, expected);
    }

    #[test]
    fn test_crate() {
        let input = quote!(crate);
        let expected = quote!($crate);
        assert_paths_eq(input, expected);
    }

    #[test]
    fn test_crate_with_path() {
        let input = quote!(crate::path::to);
        let expected = quote!($crate::path::to);
        assert_paths_eq(input, expected);
    }

    #[test]
    fn test_crate_with_invalid_global() {
        let input = quote!(::crate);
        let expected = quote!(::crate);
        assert_paths_eq(input, expected);
    }
}

#[cfg(test)]
mod test_fns_parse {
    use super::*;
    use quote::quote;
    use syn::parse2;

    /// Parse an impl block through HasFnsItem and return the resolved fns.
    fn parse_fns(input: TokenStream) -> syn::Result<Vec<Fn>> {
        parse2::<HasFnsItem>(input).map(|item| item.fns())
    }

    /// Parse an impl block and return the string representation of the nth
    /// fn's input types (excluding self) and return type.
    fn parsed_fn_sig(input: TokenStream, n: usize) -> (Vec<String>, String) {
        let fns = parse_fns(input).expect("parse failed");
        let f = &fns[n];
        let inputs: Vec<String> = f
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(t) => Some(quote!(#t).to_string()),
                _ => None,
            })
            .collect();
        let output = match &f.output {
            ReturnType::Default => "()".to_string(),
            ReturnType::Type(_, ty) => quote!(#ty).to_string(),
        };
        (inputs, output)
    }

    #[test]
    fn test_no_associated_types() {
        let input = quote! {
            impl MyContract {
                pub fn hello(x: u32) -> u64 {}
            }
        };
        let (inputs, output) = parsed_fn_sig(input, 0);
        assert_eq!(inputs, vec!["x : u32"]);
        assert_eq!(output, "u64");
    }

    #[test]
    fn test_basic_param_and_return() {
        let input = quote! {
            impl MyContract {
                type Val = u64;
                pub fn get(x: Self::Val) -> Self::Val {}
            }
        };
        let (inputs, output) = parsed_fn_sig(input, 0);
        assert_eq!(inputs, vec!["x : u64"]);
        assert_eq!(output, "u64");
    }

    #[test]
    fn test_chained_two_step() {
        let input = quote! {
            impl MyContract {
                type A = u32;
                type B = Self::A;
                pub fn get(x: Self::B) {}
            }
        };
        let (inputs, _) = parsed_fn_sig(input, 0);
        assert_eq!(inputs, vec!["x : u32"]);
    }

    #[test]
    fn test_wrapped_option() {
        let input = quote! {
            impl MyContract {
                type A = u64;
                pub fn get(x: Option<Self::A>) {}
            }
        };
        let (inputs, _) = parsed_fn_sig(input, 0);
        assert_eq!(inputs, vec!["x : Option < u64 >"]);
    }

    #[test]
    fn test_double_wrapped_result_vec() {
        let input = quote! {
            impl MyContract {
                type A = u64;
                pub fn get(x: Result<Vec<Self::A>, Error>) {}
            }
        };
        let (inputs, _) = parsed_fn_sig(input, 0);
        assert_eq!(inputs, vec!["x : Result < Vec < u64 > , Error >"]);
    }

    #[test]
    fn test_reject_qualified_self_path() {
        let input = quote! {
            impl MyContract {
                pub fn get(x: <Self as Trait>::A) {}
            }
        };
        let Err(err) = parse_fns(input) else {
            panic!("expected error");
        };
        assert!(
            err.to_string().contains("qualified associated types"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_reject_generic_associated_type() {
        let input = quote! {
            impl MyContract {
                pub fn get(x: Self::Foo<u32>) {}
            }
        };
        let Err(err) = parse_fns(input) else {
            panic!("expected error");
        };
        assert!(
            err.to_string().contains("generic associated types"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_reject_buried_qualified_self_path() {
        let input = quote! {
            impl MyContract {
                pub fn get(x: Result<Vec<<Self as Trait>::A>, Error>) {}
            }
        };
        let Err(err) = parse_fns(input) else {
            panic!("expected error");
        };
        assert!(
            err.to_string().contains("qualified associated types"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_reject_unresolved_type() {
        let input = quote! {
            impl MyContract {
                pub fn get(x: Self::Elsewhere) {}
            }
        };
        let Err(err) = parse_fns(input) else {
            panic!("expected error");
        };
        assert!(
            err.to_string()
                .contains("unresolved associated type `Self::Elsewhere`"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_reject_recursive_cycle() {
        let input = quote! {
            impl MyContract {
                type A = Self::B;
                type B = Self::A;
                pub fn get(x: Self::A) {}
            }
        };
        let Err(err) = parse_fns(input) else {
            panic!("expected error");
        };
        assert!(
            err.to_string().contains("depth limit exceeded"),
            "unexpected error: {err}"
        );
    }
}
