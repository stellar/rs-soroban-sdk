use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    AngleBracketedGenericArguments, Attribute, GenericArgument, Path, PathArguments, PathSegment,
    ReturnType, Token, TypePath,
};
use syn::{
    spanned::Spanned, token::And, Error, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, ItemTrait,
    Lifetime, Pat, PatIdent, PatType, TraitItem, TraitItemFn, Type, TypeReference, Visibility,
};

/// Gets methods from the implementation that have public visibility. For
/// methods that are inherently implemented this is methods that have a pub
/// visibility keyword. For methods that are implementing a trait the pub is
/// assumed and so all methods are returned.
pub fn impl_pub_methods(imp: &ItemImpl) -> impl Iterator<Item = &ImplItemFn> {
    imp.items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Fn(m) => Some(m),
            _ => None,
        })
        .filter(|m| imp.trait_.is_some() || matches!(m.vis, Visibility::Public(_)))
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

    pub fn fns(&self) -> Vec<Fn<'_>> {
        match self {
            HasFnsItem::Trait(t) => trait_methods(t)
                .map(|m| Fn {
                    ident: &m.sig.ident,
                    attrs: &m.attrs,
                    inputs: &m.sig.inputs,
                    output: &m.sig.output,
                })
                .collect(),
            HasFnsItem::Impl(i) => impl_pub_methods(i)
                .map(|m| Fn {
                    ident: &m.sig.ident,
                    attrs: &m.attrs,
                    inputs: &m.sig.inputs,
                    output: &m.sig.output,
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
            flatten_associated_items_in_impl_fns(&mut imp);
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

pub struct Fn<'a> {
    pub ident: &'a Ident,
    pub attrs: &'a [Attribute],
    pub inputs: &'a Punctuated<FnArg, Comma>,
    pub output: &'a ReturnType,
}

impl<'a> Fn<'a> {
    pub fn output(&self) -> Type {
        let t = match self.output {
            ReturnType::Default => quote!(()),
            ReturnType::Type(_, typ) => match unpack_result(typ) {
                Some((t, _)) => quote!(#t),
                None => quote!(#typ),
            },
        };
        Type::Verbatim(t)
    }
    pub fn try_output(&self, crate_path: &Path) -> Type {
        let (t, e) = match self.output {
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

fn flatten_associated_items_in_impl_fns(imp: &mut ItemImpl) {
    // TODO: Flatten associated consts used in functions.
    // Flatten associated types used in functions.
    let associated_types = imp
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Type(i) => Some((i.ident.clone(), i.ty.clone())),
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    let fn_input_types = imp
        .items
        .iter_mut()
        .filter_map(|item| match item {
            ImplItem::Fn(f) => Some(f.sig.inputs.iter_mut().filter_map(|input| match input {
                FnArg::Typed(t) => Some(&mut t.ty),
                _ => None,
            })),
            _ => None,
        })
        .flatten();
    for t in fn_input_types {
        if let Type::Path(TypePath { qself: None, path }) = t.as_mut() {
            let segments = &path.segments;
            if segments.len() == 2
                && segments.first() == Some(&PathSegment::from(format_ident!("Self")))
            {
                if let Some(PathSegment {
                    arguments: PathArguments::None,
                    ident,
                }) = segments.get(1)
                {
                    if let Some(resolved_ty) = associated_types.get(ident) {
                        *t.as_mut() = resolved_ty.clone();
                    }
                }
            }
        }
    }
}

pub fn ty_to_safe_ident_str(ty: &Type) -> String {
    quote!(#ty).to_string().replace(' ', "").replace(':', "_")
}
