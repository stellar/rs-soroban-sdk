//! The id a user-defined type is known by in the contract spec.
//!
//! A type's definition entry carries the id, and every reference to the type
//! carries the same id, so a reference can be matched to its definition
//! exactly even when two types share a name.

use crate::spec_shaking::sha256;

/// The 8-byte id of a user-defined type in the contract spec: the SHA-256 of
/// the type's fully qualified name, truncated.
///
/// The fully qualified name is the type's crate, the modules it is defined in,
/// then its own name (`my_crate::my_module::MyType`), so the id is unique
/// within a build across the crates that make it up, even when types share a
/// name. The derive macros assemble the name from `module_path!()` expanded
/// where the type is defined, combined with the type's ident, which is the
/// string `core::any::type_name` reports for the type; `type_name` itself is
/// not yet callable in const contexts on stable Rust, which is why the name is
/// assembled rather than asked of the compiler.
///
/// This is a `const fn` so that macro-generated code derives the id while the
/// generated code compiles, which is the only time the fully qualified name
/// is known; a proc macro sees the tokens of the type it expands and not the
/// module those tokens sit in.
#[doc(hidden)]
#[must_use]
pub const fn spec_type_id(fully_qualified_type_name: &str) -> [u8; 8] {
    let h = sha256(fully_qualified_type_name.as_bytes());
    [h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]]
}

#[cfg(test)]
mod test {
    use super::spec_type_id;

    /// Evaluatable at compile time, which is the whole point.
    #[test]
    fn spec_type_id_is_const() {
        const ID: [u8; 8] = spec_type_id(concat!(module_path!(), "::", "Flag"));
        assert_eq!(ID, spec_type_id("soroban_sdk::spec_type_id::test::Flag"));
    }

    /// Two types sharing a name must not share an id.
    #[test]
    fn distinct_for_same_ident_in_different_modules() {
        assert_ne!(spec_type_id("a::Flag"), spec_type_id("b::Flag"));
    }

    /// The id is the truncated SHA-256 of the fully qualified name, which the
    /// tooling can recompute host-side.
    #[test]
    fn matches_sha2_of_name() {
        use sha2::{Digest, Sha256};
        let h: [u8; 32] = Sha256::digest(b"a::Flag").into();
        assert_eq!(spec_type_id("a::Flag"), h[..8]);
    }
}
