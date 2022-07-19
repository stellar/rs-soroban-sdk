use quote::format_ident;
use syn::{LitStr, NestedMeta};

// TODO: Consider replacing this with crates.io/crates/darling.

pub fn get_str(args: &Vec<NestedMeta>, name: &str) -> Option<LitStr> {
    args.iter().find_map(|m| match m {
        syn::NestedMeta::Meta(m) => match m {
            syn::Meta::NameValue(nv) => {
                if nv.path.is_ident(&format_ident!("{}", name)) {
                    match &nv.lit {
                        syn::Lit::Str(s) => Some(s.clone()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            syn::Meta::Path(_) | syn::Meta::List(_) => None,
        },
        syn::NestedMeta::Lit(_) => None,
    })
}
