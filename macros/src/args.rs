use quote::format_ident;
use syn::{Lit, LitStr, Meta, NestedMeta};

// TODO: Consider replacing this with crates.io/crates/darling.

pub fn get_str(args: &Vec<NestedMeta>, name: &str) -> Option<LitStr> {
    args.iter().find_map(|m| match m {
        NestedMeta::Meta(m) => match m {
            Meta::NameValue(nv) => {
                if nv.path.is_ident(&format_ident!("{}", name)) {
                    match &nv.lit {
                        Lit::Str(s) => Some(s.clone()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Meta::Path(_) | Meta::List(_) => None,
        },
        NestedMeta::Lit(_) => None,
    })
}
