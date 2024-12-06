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
    Lifetime, Pat, PatType, TraitItem, TraitItemFn, Type, TypeReference, Visibility,
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

/// Returns a clone of the type from the FnArg.
pub fn fn_arg_ref_type(arg: &FnArg, lifetime: Option<&Lifetime>) -> Result<Type, Error> {
    if let FnArg::Typed(pat_type) = arg {
        if !matches!(*pat_type.ty, Type::Reference(_)) {
            Ok(Type::Reference(TypeReference {
                and_token: And::default(),
                lifetime: lifetime.cloned(),
                mutability: None,
                elem: pat_type.ty.clone(),
            }))
        } else {
            Ok((*pat_type.ty).clone())
        }
    } else {
        Err(Error::new(
            arg.span(),
            "argument in this form is not supported, use simple named arguments only",
        ))
    }
}

/// Returns a clone of FnArg with the type as a reference if the arg is a typed
/// arg and its type is not already a reference.
pub fn fn_arg_make_ref(arg: &FnArg, lifetime: Option<&Lifetime>) -> FnArg {
    if let FnArg::Typed(pat_type) = arg {
        if !matches!(*pat_type.ty, Type::Reference(_)) {
            return FnArg::Typed(PatType {
                attrs: pat_type.attrs.clone(),
                pat: pat_type.pat.clone(),
                colon_token: pat_type.colon_token,
                ty: Box::new(Type::Reference(TypeReference {
                    and_token: And::default(),
                    lifetime: lifetime.cloned(),
                    mutability: None,
                    elem: pat_type.ty.clone(),
                })),
            });
        }
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

    pub fn fns(&'_ self) -> Vec<Fn> {
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
