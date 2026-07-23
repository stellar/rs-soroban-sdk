//! Resolving type hashes in a set of contract spec entries.
//!
//! [`resolve`] enriches a spec so that it is self-describing about the
//! spec-shaking v2 hashes of its types:
//!
//! 1. Each user-defined type and event stores its own hash (the value returned
//!    by [`crate::shaking::hash_for_entry`]) in the `lib` field, alongside any
//!    library name already there.
//! 2. Every reference to a user-defined type ([`ScSpecTypeDef::Udt`]) is
//!    rewritten to [`ScSpecTypeDef::UdtV2`], which carries the referenced
//!    type's hash in addition to its name.
//!
//! The SDK cannot do this at compile time: a proc-macro expanding one type has
//! no access to the definitions of the types it references, so the hashes can
//! only be filled in once the full set of entries is known. This is the same
//! place spec-shaking already runs (e.g. in the stellar-cli), operating on the
//! spec read back out of a compiled contract.
//!
//! `resolve` is idempotent: running it again produces the same result.

use std::collections::HashMap;

use stellar_xdr::{ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdtv2, ScSpecUdtDef, VecM};

use crate::shaking::hash_for_entry;
use crate::visit::walk_entry_type_defs;

/// Returns the library name stored in a spec entry's `lib` field, if any.
///
/// The `lib` field is a list that may hold a library-name entry (set for types
/// imported from another contract) and/or the type's spec hash. This returns
/// the library name, ignoring the hash.
#[must_use]
pub fn lib_name(lib: &VecM<ScSpecUdtDef>) -> Option<String> {
    lib.iter().find_map(|d| match d {
        ScSpecUdtDef::Lib(s) => Some(s.to_utf8_string_lossy()),
        ScSpecUdtDef::Hash(_) => None,
    })
}

/// Enriches spec entries in place with their spec-shaking hashes.
///
/// Stores each type's and event's own hash in its `lib` field, and rewrites
/// every user-defined-type reference to carry the referenced type's hash. See
/// the [module docs](self) for details. Idempotent.
pub fn resolve(entries: &mut [ScSpecEntry]) {
    // Compute each entry's hash, store it, and record referenceable types.
    let mut hashes: HashMap<String, [u8; 8]> = HashMap::new();
    for entry in entries.iter_mut() {
        let hash = hash_for_entry(entry);
        set_hash(entry, hash);
        if let Some(name) = referenceable_type_name(entry) {
            hashes.insert(name, hash);
        }
    }

    // Rewrite references now that every type's hash is known. A `Udt`
    // reference whose name has a known hash becomes a `UdtV2` reference
    // carrying that hash; references to unknown names (e.g. types defined in
    // another contract) are left unchanged.
    for entry in entries.iter_mut() {
        walk_entry_type_defs(entry, &mut |t| {
            if let ScSpecTypeDef::Udt(u) = t {
                if let Some(hash) = hashes.get(&u.name.to_utf8_string_lossy()) {
                    *t = ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 {
                        hash: *hash,
                        name: u.name.clone(),
                    });
                }
            }
        });
    }
}

/// The name of a type that other types may reference. Events and functions are
/// not referenceable as types and return `None`.
fn referenceable_type_name(entry: &ScSpecEntry) -> Option<String> {
    match entry {
        ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::EventV0(_) | ScSpecEntry::FunctionV0(_) => None,
    }
}

/// A mutable reference to an entry's `lib` field, or `None` for functions.
fn lib_mut(entry: &mut ScSpecEntry) -> Option<&mut VecM<ScSpecUdtDef>> {
    match entry {
        ScSpecEntry::UdtStructV0(s) => Some(&mut s.lib),
        ScSpecEntry::UdtUnionV0(u) => Some(&mut u.lib),
        ScSpecEntry::UdtEnumV0(e) => Some(&mut e.lib),
        ScSpecEntry::UdtErrorEnumV0(e) => Some(&mut e.lib),
        ScSpecEntry::EventV0(e) => Some(&mut e.lib),
        ScSpecEntry::FunctionV0(_) => None,
    }
}

/// Stores `hash` in the entry's `lib` field, replacing any hash already there
/// and preserving any library name. No-op for functions.
fn set_hash(entry: &mut ScSpecEntry, hash: [u8; 8]) {
    if let Some(lib) = lib_mut(entry) {
        let mut defs: Vec<ScSpecUdtDef> = lib
            .iter()
            .filter(|d| !matches!(d, ScSpecUdtDef::Hash(_)))
            .cloned()
            .collect();
        defs.push(ScSpecUdtDef::Hash(hash));
        *lib = defs.try_into().expect("lib entry count within bounds");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stellar_xdr::{
        ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0, ScSpecUdtStructFieldV0,
        ScSpecUdtStructV0, StringM,
    };

    fn struct_referencing(name: &str, field_type: ScSpecTypeDef) -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: StringM::default(),
            lib: VecM::default(),
            name: name.try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: StringM::default(),
                name: "f".try_into().unwrap(),
                type_: field_type,
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn enum_named(name: &str) -> ScSpecEntry {
        ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            doc: StringM::default(),
            lib: VecM::default(),
            name: name.try_into().unwrap(),
            cases: vec![ScSpecUdtEnumCaseV0 {
                doc: StringM::default(),
                name: "A".try_into().unwrap(),
                value: 0,
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn udt(name: &str) -> ScSpecTypeDef {
        ScSpecTypeDef::Udt(ScSpecTypeUdt {
            name: name.try_into().unwrap(),
        })
    }

    #[test]
    fn stores_hash_and_rewrites_reference() {
        let referenced = enum_named("B");
        let referenced_hash = hash_for_entry(&referenced);

        let mut entries = vec![struct_referencing("A", udt("B")), referenced];
        resolve(&mut entries);

        // The referenced enum stores its own hash in lib.
        let ScSpecEntry::UdtEnumV0(b) = &entries[1] else {
            panic!("expected enum");
        };
        assert!(b.lib.iter().any(|d| *d == ScSpecUdtDef::Hash(referenced_hash)));

        // The reference now carries the referenced type's hash.
        let ScSpecEntry::UdtStructV0(a) = &entries[0] else {
            panic!("expected struct");
        };
        match &a.fields[0].type_ {
            ScSpecTypeDef::UdtV2(u) => {
                assert_eq!(u.name.to_utf8_string_lossy(), "B");
                assert_eq!(u.hash, referenced_hash);
            }
            other => panic!("expected UdtV2, got {other:?}"),
        }
    }

    #[test]
    fn rewrites_nested_references() {
        let mut entries = vec![
            struct_referencing(
                "A",
                ScSpecTypeDef::Vec(Box::new(stellar_xdr::ScSpecTypeVec {
                    element_type: Box::new(udt("B")),
                })),
            ),
            enum_named("B"),
        ];
        resolve(&mut entries);

        let ScSpecEntry::UdtStructV0(a) = &entries[0] else {
            panic!("expected struct");
        };
        let ScSpecTypeDef::Vec(v) = &a.fields[0].type_ else {
            panic!("expected vec");
        };
        assert!(matches!(*v.element_type, ScSpecTypeDef::UdtV2(_)));
    }

    #[test]
    fn unknown_reference_left_unchanged() {
        let mut entries = vec![struct_referencing("A", udt("Missing"))];
        resolve(&mut entries);
        let ScSpecEntry::UdtStructV0(a) = &entries[0] else {
            panic!("expected struct");
        };
        assert!(matches!(a.fields[0].type_, ScSpecTypeDef::Udt(_)));
    }

    #[test]
    fn is_idempotent() {
        let mut once = vec![struct_referencing("A", udt("B")), enum_named("B")];
        resolve(&mut once);
        let mut twice = once.clone();
        resolve(&mut twice);
        assert_eq!(once, twice);
    }

    #[test]
    fn lib_name_extracts_library() {
        let lib: VecM<ScSpecUdtDef> = vec![
            ScSpecUdtDef::Lib("mylib".try_into().unwrap()),
            ScSpecUdtDef::Hash([0; 8]),
        ]
        .try_into()
        .unwrap();
        assert_eq!(lib_name(&lib), Some("mylib".to_string()));
        assert_eq!(lib_name(&VecM::default()), None);
    }
}
