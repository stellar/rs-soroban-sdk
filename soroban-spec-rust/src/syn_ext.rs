use std::collections::{HashMap, HashSet};

use proc_macro2::Ident;
use stellar_xdr::{ScSymbol, StringM};

use crate::types::GenerateError;

pub trait IntoIdent {
    /// The name as written in the spec.
    fn to_name(&self) -> Result<String, GenerateError>;

    fn into_ident(&self) -> Result<Ident, GenerateError> {
        let s = self.to_name()?;
        syn::parse_str::<Ident>(&s).map_err(|_| GenerateError::InvalidIdent(s))
    }
}

impl IntoIdent for str {
    fn to_name(&self) -> Result<String, GenerateError> {
        Ok(self.to_string())
    }
}

impl<const N: u32> IntoIdent for StringM<N> {
    fn to_name(&self) -> Result<String, GenerateError> {
        self.to_utf8_string()
            .map_err(|_| GenerateError::InvalidUtf8)
    }
}

impl IntoIdent for ScSymbol {
    fn to_name(&self) -> Result<String, GenerateError> {
        self.0.to_name()
    }
}

/// Creates a Rust identifier from a string or spec name, returning an error if
/// it contains invalid UTF-8 or is not a valid identifier.
pub fn str_to_ident(s: &(impl IntoIdent + ?Sized)) -> Result<Ident, GenerateError> {
    s.into_ident()
}

/// The last `::`-separated segment of a fully qualified spec type name.
fn last_segment(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name)
}

/// The Rust identifiers that name the user-defined types of a spec.
///
/// A spec names a user-defined type by its fully qualified name
/// (`crate::module::Type`), which is not a Rust identifier, so generated
/// bindings name it by its last segment. Two types defined in different modules
/// can share a last segment, so a segment already claimed is numbered.
#[derive(Debug, Default)]
pub struct TypeNames {
    renamed: HashMap<String, String>,
}

impl TypeNames {
    /// Resolves the identifiers for the fully qualified names of every
    /// user-defined type a spec defines.
    pub fn new<'a>(defined: impl IntoIterator<Item = &'a str>) -> Self {
        // The first type to claim a last segment keeps it, so a type only ever
        // loses its own name to one that came before it, never to a number
        // handed to a type that collided with something else.
        let mut taken = HashSet::new();
        let colliding: Vec<&str> = defined
            .into_iter()
            .filter(|name| !taken.insert(last_segment(name).to_string()))
            .collect();

        let mut renamed = HashMap::new();
        for name in colliding {
            let base = last_segment(name);
            let mut n = 1u32;
            let ident = loop {
                n += 1;
                let ident = format!("{base}{n}");
                if taken.insert(ident.clone()) {
                    break ident;
                }
            };
            renamed.insert(name.to_string(), ident);
        }
        Self { renamed }
    }

    /// The Rust identifier naming the user-defined type the given fully
    /// qualified spec name refers to.
    ///
    /// A name this was not built from resolves to its own last segment, which is
    /// what a spec containing only that one type would produce.
    pub fn ident(&self, name: &(impl IntoIdent + ?Sized)) -> Result<Ident, GenerateError> {
        let name = name.to_name()?;
        match self.renamed.get(&name) {
            Some(ident) => str_to_ident(ident.as_str()),
            None => str_to_ident(last_segment(&name)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::TypeNames;

    #[test]
    fn names_by_last_segment() {
        let names = TypeNames::new(["a::b::Thing", "a::c::Other"]);
        assert_eq!(names.ident("a::b::Thing").unwrap(), "Thing");
        assert_eq!(names.ident("a::c::Other").unwrap(), "Other");
    }

    #[test]
    fn numbers_a_last_segment_already_claimed() {
        // The first to claim `Thing` keeps it; the rest are numbered in order.
        let names = TypeNames::new(["a::b::Thing", "a::c::Thing", "a::d::Thing"]);
        assert_eq!(names.ident("a::b::Thing").unwrap(), "Thing");
        assert_eq!(names.ident("a::c::Thing").unwrap(), "Thing2");
        assert_eq!(names.ident("a::d::Thing").unwrap(), "Thing3");
    }

    #[test]
    fn skips_a_number_taken_by_a_type_of_that_name() {
        // `Thing2` is a type in its own right, so the collision numbering steps
        // over it rather than colliding with it in turn.
        let names = TypeNames::new(["a::b::Thing", "a::c::Thing2", "a::d::Thing"]);
        assert_eq!(names.ident("a::b::Thing").unwrap(), "Thing");
        assert_eq!(names.ident("a::c::Thing2").unwrap(), "Thing2");
        assert_eq!(names.ident("a::d::Thing").unwrap(), "Thing3");
    }

    #[test]
    fn unknown_name_falls_back_to_its_last_segment() {
        let names = TypeNames::default();
        assert_eq!(names.ident("a::b::Thing").unwrap(), "Thing");
        assert_eq!(names.ident("Thing").unwrap(), "Thing");
    }
}
