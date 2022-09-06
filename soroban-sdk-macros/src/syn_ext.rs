use syn::{
    spanned::Spanned, token::And, Error, FnArg, Ident, ImplItem, ImplItemMethod, ItemImpl,
    ItemTrait, Pat, PatType, TraitItem, TraitItemMethod, Type, TypeReference, Visibility,
};

/// Gets methods from the implementation that have public visibility. For
/// methods that are inherently implemented this is methods that have a pub
/// visibility keyword. For methods that are implementing a trait the pub is
/// assumed and so all methods are returned.
pub fn impl_pub_methods(imp: &ItemImpl) -> impl Iterator<Item = &ImplItemMethod> {
    imp.items
        .iter()
        .filter_map(|i| match i {
            ImplItem::Method(m) => Some(m),
            _ => None,
        })
        .filter(|m| imp.trait_.is_some() || matches!(m.vis, Visibility::Public(_)))
}

/// Gets methods from the trait.
pub fn trait_methods(imp: &ItemTrait) -> impl Iterator<Item = &TraitItemMethod> {
    imp.items.iter().filter_map(|i| match i {
        TraitItem::Method(m) => Some(m),
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
    Err(Error::new(arg.span(), "argument not supported"))
}

/// Returns a clone of FnArg with the type as a reference if the arg is a typed
/// arg and its type is not already a reference.
pub fn fn_arg_make_ref(arg: &FnArg) -> FnArg {
    if let FnArg::Typed(pat_type) = arg {
        if !matches!(*pat_type.ty, Type::Reference(_)) {
            return FnArg::Typed(PatType {
                attrs: pat_type.attrs.clone(),
                pat: pat_type.pat.clone(),
                colon_token: pat_type.colon_token,
                ty: Box::new(Type::Reference(TypeReference {
                    and_token: And::default(),
                    lifetime: None,
                    mutability: None,
                    elem: pat_type.ty.clone(),
                })),
            });
        }
    }
    arg.clone()
}
