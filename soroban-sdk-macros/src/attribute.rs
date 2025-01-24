use syn::Attribute;

/// Returns true if the attribute is an attribute that should be preserved and
/// passed through to code generated for the item the attribute is on.
pub fn pass_through_attr_to_gen_code(attr: &Attribute) -> bool {
    attr.path().is_ident("doc")
        || attr.path().is_ident("cfg")
        || attr.path().is_ident("allow")
        || attr.path().is_ident("deny")
}
