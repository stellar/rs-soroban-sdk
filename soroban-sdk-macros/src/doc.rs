use itertools::Itertools;
use syn::{Attribute, Lit, Meta, MetaNameValue};

pub fn docs_from_attrs(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter(|a| a.path.is_ident("doc"))
        .filter_map(|a| match a.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue {
                lit: Lit::Str(s), ..
            })) => Some(s.value()),
            _ => None,
        })
        .map(|s| s.trim().to_string())
        .join("\n")
}
