use itertools::Itertools;
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::StringM;
use syn::{Attribute, Expr, ExprLit, Lit, Meta, MetaNameValue};

const DOCS_MAX_LEN: u32 = 1024;

pub fn docs_from_attrs(attrs: &[Attribute]) -> StringM<DOCS_MAX_LEN> {
    let mut docs = attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .filter_map(|a| match &a.meta {
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ..
            }) => Some(s.value()),
            _ => None,
        })
        .map(|s| s.trim().to_string())
        .join("\n")
        .as_bytes()
        .to_vec();
    docs.truncate(DOCS_MAX_LEN as usize);
    docs.try_into().unwrap()
}
