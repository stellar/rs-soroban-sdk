use std::collections::HashSet;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
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

/// The `::`-separated segments of a fully qualified spec type name.
fn segments(name: &str) -> Vec<&str> {
    name.split("::").collect()
}

/// The last `::`-separated segment of a fully qualified spec type name.
fn last_segment(name: &str) -> &str {
    name.rsplit("::").next().unwrap_or(name)
}

/// A Rust identifier for one segment of a qualified name.
///
/// A module of the crate the type came from can be named for a Rust keyword, so
/// a segment that is not an identifier on its own is emitted raw.
fn segment_ident(s: &str) -> Result<Ident, GenerateError> {
    str_to_ident(s).or_else(|_| str_to_ident(format!("r#{s}").as_str()))
}

/// The Rust identifiers that name the user-defined types of a spec.
///
/// A spec names a user-defined type by its fully qualified name
/// (`crate::module::Type`). Generated bindings mirror that name as a module
/// path, so every type is reachable at the path it reports for itself and two
/// types that share a last segment stay distinct.
///
/// For the bindings to stay usable by name, the types are also named at the root
/// of the generated module. Only one type can claim a given last segment there,
/// so the first to claim it is named at the root and the rest are reached only
/// by their full path.
#[derive(Debug, Default)]
pub struct TypeNames {
    defined: Vec<String>,
    aliased: HashSet<String>,
}

impl TypeNames {
    /// Resolves the names of every user-defined type a spec defines.
    pub fn new<'a>(defined: impl IntoIterator<Item = &'a str>) -> Self {
        let defined: Vec<String> = defined.into_iter().map(str::to_string).collect();
        let mut taken = HashSet::new();
        let aliased = defined
            .iter()
            .filter(|name| taken.insert(last_segment(name).to_string()))
            .cloned()
            .collect();
        Self { defined, aliased }
    }

    /// The identifier declaring the type, which is the last segment of its name.
    pub fn ident(&self, name: &(impl IntoIdent + ?Sized)) -> Result<Ident, GenerateError> {
        segment_ident(last_segment(&name.to_name()?))
    }

    /// The path referring to the type from `depth` modules below the root of the
    /// generated module.
    ///
    /// Every reference is written from the root down, reaching it by stepping up
    /// out of the module the reference is written in, so that a reference means
    /// the same thing wherever it appears.
    pub fn path(
        &self,
        name: &(impl IntoIdent + ?Sized),
        depth: usize,
    ) -> Result<TokenStream, GenerateError> {
        let name = name.to_name()?;
        let segments = segments(&name)
            .into_iter()
            .map(segment_ident)
            .collect::<Result<Vec<_>, _>>()?;
        let up = std::iter::repeat_n(quote!(super::), depth);
        Ok(quote! { #(#up)* #(#segments)::* })
    }

    /// The module path a type is defined in, which is its name without its last
    /// segment.
    pub fn module_of(name: &str) -> Vec<&str> {
        let mut s = segments(name);
        s.pop();
        s
    }

    /// The types this defines, in the order they were given.
    pub fn defined(&self) -> impl Iterator<Item = &str> {
        self.defined.iter().map(String::as_str)
    }

    /// Whether the type is the one named at the root for its last segment.
    pub fn is_aliased(&self, name: &str) -> bool {
        self.aliased.contains(name)
    }
}

#[cfg(test)]
mod test {
    use super::TypeNames;

    #[test]
    fn declares_a_type_by_its_last_segment() {
        let names = TypeNames::new(["a::b::Thing"]);
        assert_eq!(names.ident("a::b::Thing").unwrap(), "Thing");
    }

    #[test]
    fn refers_to_a_type_by_its_whole_path() {
        let names = TypeNames::new(["a::b::Thing"]);
        assert_eq!(
            names.path("a::b::Thing", 0).unwrap().to_string(),
            "a :: b :: Thing"
        );
    }

    #[test]
    fn steps_up_out_of_the_module_a_reference_is_written_in() {
        let names = TypeNames::new(["a::b::Thing"]);
        assert_eq!(
            names.path("a::b::Thing", 2).unwrap().to_string(),
            "super :: super :: a :: b :: Thing"
        );
    }

    #[test]
    fn names_a_keyword_segment_raw() {
        let names = TypeNames::new(["a::type::Thing"]);
        assert_eq!(
            names.path("a::type::Thing", 0).unwrap().to_string(),
            "a :: r#type :: Thing"
        );
    }

    #[test]
    fn names_only_the_first_claim_on_a_last_segment_at_the_root() {
        // The rest stay reachable, but only by their whole path.
        let names = TypeNames::new(["a::b::Thing", "a::c::Thing", "a::d::Other"]);
        assert!(names.is_aliased("a::b::Thing"));
        assert!(!names.is_aliased("a::c::Thing"));
        assert!(names.is_aliased("a::d::Other"));
    }

    #[test]
    fn a_module_is_a_name_without_its_last_segment() {
        assert_eq!(TypeNames::module_of("a::b::Thing"), vec!["a", "b"]);
        assert_eq!(TypeNames::module_of("Thing"), Vec::<&str>::new());
    }
}
