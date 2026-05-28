use itertools::Itertools;
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
        .join("\n");
    // Truncate on a char boundary to avoid splitting multi-byte UTF-8 codepoints.
    let max = DOCS_MAX_LEN as usize;
    let safe_len = docs.floor_char_boundary(max);
    docs.truncate(safe_len);
    docs.into_bytes().try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_truncation_does_not_split_multibyte_utf8() {
        // (DOCS_MAX_LEN - 1) ASCII bytes followed by 'é' (2 bytes: 0xC3
        // 0xA9) = DOCS_MAX_LEN + 1 bytes. Truncation at DOCS_MAX_LEN keeps
        // the 0xC3 but drops the 0xA9, producing invalid UTF-8.
        let padding = "a".repeat(DOCS_MAX_LEN as usize - 1);
        let doc_value = format!("{padding}é");
        let attr: Attribute = parse_quote!(#[doc = #doc_value]);
        let result = docs_from_attrs(&[attr]);
        let bytes: Vec<u8> = result.into();
        assert!(
            std::str::from_utf8(&bytes).is_ok(),
            "truncation produced invalid UTF-8: trailing bytes {:?}",
            &bytes[bytes.len().saturating_sub(4)..]
        );
    }
}
