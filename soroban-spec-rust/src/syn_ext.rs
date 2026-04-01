use proc_macro2::Ident;

use crate::types::GenerateError;

/// Creates a Rust identifier from a string, returning an error if it is not a
/// valid identifier.
pub fn str_to_ident(s: &str) -> Result<Ident, GenerateError> {
    syn::parse_str::<Ident>(s).map_err(|_| GenerateError::InvalidIdent(s.to_string()))
}
