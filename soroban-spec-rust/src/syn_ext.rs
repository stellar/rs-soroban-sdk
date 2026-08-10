use proc_macro2::Ident;
use stellar_xdr::{ScSymbol, StringM};

use crate::types::GenerateError;

pub trait IntoIdent {
    fn into_ident(&self) -> Result<Ident, GenerateError>;
}

impl IntoIdent for str {
    fn into_ident(&self) -> Result<Ident, GenerateError> {
        syn::parse_str::<Ident>(self).map_err(|_| GenerateError::InvalidIdent(self.to_string()))
    }
}

impl<const N: u32> IntoIdent for StringM<N> {
    fn into_ident(&self) -> Result<Ident, GenerateError> {
        let s = self
            .to_utf8_string()
            .map_err(|_| GenerateError::InvalidUtf8)?;
        s.as_str().into_ident()
    }
}

impl IntoIdent for ScSymbol {
    fn into_ident(&self) -> Result<Ident, GenerateError> {
        self.0.into_ident()
    }
}

/// Creates a Rust identifier from a string or spec name, returning an error if
/// it contains invalid UTF-8 or is not a valid identifier.
pub fn str_to_ident(s: &(impl IntoIdent + ?Sized)) -> Result<Ident, GenerateError> {
    s.into_ident()
}

/// The Rust identifiers naming the user-defined types of a spec, resolved by
/// type id.
///
/// A spec entry defining a user-defined type carries an id hashed from the
/// type's fully qualified name, and a reference carries the id of the type it
/// refers to, so a reference is married up with its definition by id rather
/// than by name. Two types can share a name: the first keeps it, and each
/// later one is declared under the name suffixed with a counter (`Flag`,
/// `Flag2`, ...), so every type stays reachable by a distinct identifier and a
/// reference always resolves to the type it was written against.
#[derive(Debug, Default)]
pub struct TypeIds {
    by_id: std::collections::HashMap<[u8; 8], String>,
}

impl TypeIds {
    /// Resolves the identifiers of every user-defined type a spec defines,
    /// given each type's name and id in spec order.
    pub fn new(defined: impl IntoIterator<Item = (String, [u8; 8])>) -> Self {
        let mut by_id = std::collections::HashMap::new();
        let mut taken = std::collections::HashSet::new();
        for (name, id) in defined {
            let mut candidate = name.clone();
            let mut n = 1usize;
            while !taken.insert(candidate.clone()) {
                n += 1;
                candidate = format!("{name}{n}");
            }
            by_id.insert(id, candidate);
        }
        Self { by_id }
    }

    /// The identifier of the type with the given id, falling back to the given
    /// name when the id is not one the spec defines, such as a reference to a
    /// type whose definition entry is not present.
    pub fn ident(&self, id: &[u8; 8], name: &str) -> Result<Ident, GenerateError> {
        match self.by_id.get(id) {
            Some(ident) => str_to_ident(ident.as_str()),
            None => str_to_ident(name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::TypeIds;

    fn s(v: &str) -> String {
        v.to_string()
    }

    #[test]
    fn first_type_keeps_its_name() {
        let ids = TypeIds::new([(s("Flag"), [1; 8])]);
        assert_eq!(ids.ident(&[1; 8], "Flag").unwrap(), "Flag");
    }

    #[test]
    fn same_named_types_stay_distinct() {
        let ids = TypeIds::new([(s("Flag"), [1; 8]), (s("Flag"), [2; 8])]);
        assert_eq!(ids.ident(&[1; 8], "Flag").unwrap(), "Flag");
        assert_eq!(ids.ident(&[2; 8], "Flag").unwrap(), "Flag2");
    }

    #[test]
    fn suffixed_name_does_not_take_a_real_name() {
        let ids = TypeIds::new([
            (s("Flag"), [1; 8]),
            (s("Flag"), [2; 8]),
            (s("Flag2"), [3; 8]),
        ]);
        assert_eq!(ids.ident(&[2; 8], "Flag").unwrap(), "Flag2");
        assert_eq!(ids.ident(&[3; 8], "Flag2").unwrap(), "Flag22");
    }

    #[test]
    fn unknown_id_falls_back_to_name() {
        let ids = TypeIds::new([]);
        assert_eq!(ids.ident(&[9; 8], "Missing").unwrap(), "Missing");
    }
}
