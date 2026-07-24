//! Computes the `id` field of [`ScSpecTypeUdtv2`] references.
//!
//! A `ScSpecTypeUdtv2` reference names another user-defined type and carries an
//! 8-byte `id` identifying that referenced type. The id is the truncated (first
//! 8 bytes) SHA256 of the referenced type's own [`ScSpecEntry`], computed over a
//! canonical form in which all `UdtV2` ids the entry itself contains are zeroed.
//! Zeroing the ids in the preimage keeps the identity independent of other
//! types' ids, so it is well-defined even for mutually- or self-recursive types.
//!
//! So for `struct A { b: B }`, the field `b` in `A`'s spec entry is a
//! `ScSpecTypeUdtv2 { name: "B", id: canonical_id(B's entry) }`.

use sha2::{Digest, Sha256};
use stellar_xdr::{
    Limits, ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdtv2, ScSpecUdtUnionCaseV0, WriteXdr,
};

/// Location of a single `ScSpecTypeUdtv2` id within an entry's XDR encoding.
pub struct UdtRefSite {
    /// Byte offset of the 8-byte id within the entry's XDR.
    pub id_offset: usize,
    /// Name of the referenced user-defined type.
    pub name: String,
}

/// Calls `f` on every `ScSpecTypeUdtv2` reachable from `t`, recursing through
/// the parameterized type-defs that can contain nested references.
fn for_each_ref_ty(t: &mut ScSpecTypeDef, f: &mut impl FnMut(&mut ScSpecTypeUdtv2)) {
    match t {
        ScSpecTypeDef::UdtV2(u) => f(u),
        ScSpecTypeDef::Option(o) => for_each_ref_ty(&mut o.value_type, f),
        ScSpecTypeDef::Result(r) => {
            for_each_ref_ty(&mut r.ok_type, f);
            for_each_ref_ty(&mut r.error_type, f);
        }
        ScSpecTypeDef::Vec(v) => for_each_ref_ty(&mut v.element_type, f),
        ScSpecTypeDef::Map(m) => {
            for_each_ref_ty(&mut m.key_type, f);
            for_each_ref_ty(&mut m.value_type, f);
        }
        ScSpecTypeDef::Tuple(tu) => {
            for vt in tu.value_types.iter_mut() {
                for_each_ref_ty(vt, f);
            }
        }
        _ => {}
    }
}

/// Calls `f` on every `ScSpecTypeUdtv2` reference contained in `entry`, in a
/// stable order.
fn for_each_ref(entry: &mut ScSpecEntry, mut f: impl FnMut(&mut ScSpecTypeUdtv2)) {
    match entry {
        ScSpecEntry::FunctionV0(fun) => {
            for i in fun.inputs.iter_mut() {
                for_each_ref_ty(&mut i.type_, &mut f);
            }
            for o in fun.outputs.iter_mut() {
                for_each_ref_ty(o, &mut f);
            }
        }
        ScSpecEntry::UdtStructV0(s) => {
            for field in s.fields.iter_mut() {
                for_each_ref_ty(&mut field.type_, &mut f);
            }
        }
        ScSpecEntry::UdtUnionV0(u) => {
            for case in u.cases.iter_mut() {
                if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                    for ty in t.type_.iter_mut() {
                        for_each_ref_ty(ty, &mut f);
                    }
                }
            }
        }
        ScSpecEntry::EventV0(e) => {
            for p in e.params.iter_mut() {
                for_each_ref_ty(&mut p.type_, &mut f);
            }
        }
        ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
    }
}

/// The name of the user-defined type an entry defines, or `None` for entries
/// that cannot be referenced by a `ScSpecTypeUdtv2` (functions, events).
fn entry_name(entry: &ScSpecEntry) -> Option<String> {
    match entry {
        ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::FunctionV0(_) | ScSpecEntry::EventV0(_) => None,
    }
}

/// Returns `entry` with every `ScSpecTypeUdtv2` id set to zero (the canonical
/// form used as the preimage for [`canonical_id`]).
fn canonical(entry: &ScSpecEntry) -> ScSpecEntry {
    let mut e = entry.clone();
    for_each_ref(&mut e, |u| u.id = [0u8; 8]);
    e
}

/// The XDR of `entry` in canonical form: every `ScSpecTypeUdtv2` id zeroed.
pub fn canonical_xdr(entry: &ScSpecEntry) -> Vec<u8> {
    canonical(entry).to_xdr(Limits::none()).unwrap()
}

/// The identity of the type `entry` defines: the truncated 8-byte SHA256 of its
/// canonical (all-ids-zeroed) XDR. This is the value carried by every
/// `ScSpecTypeUdtv2` that references this type.
pub fn canonical_id(entry: &ScSpecEntry) -> [u8; 8] {
    Sha256::digest(canonical_xdr(entry))[..8]
        .try_into()
        .unwrap()
}

/// Locates every `ScSpecTypeUdtv2` id within `entry`'s canonical XDR encoding,
/// pairing each byte offset with the name of the referenced type. Offsets are
/// found by encoding the entry with one ref's id set to a marker at a time and
/// diffing against the all-zeroed baseline, so they are exact regardless of the
/// surrounding encoding.
pub fn udt_ref_sites(entry: &ScSpecEntry) -> Vec<UdtRefSite> {
    let mut names = Vec::new();
    let mut base = entry.clone();
    for_each_ref(&mut base, |u| {
        names.push(u.name.to_utf8_string_lossy());
        u.id = [0u8; 8];
    });
    let baseline = base.to_xdr(Limits::none()).unwrap();

    let mut sites = Vec::with_capacity(names.len());
    for (i, name) in names.iter().enumerate() {
        let mut e = base.clone();
        let mut idx = 0usize;
        for_each_ref(&mut e, |u| {
            if idx == i {
                u.id = [0xFFu8; 8];
            }
            idx += 1;
        });
        let bytes = e.to_xdr(Limits::none()).unwrap();
        let id_offset = baseline
            .iter()
            .zip(bytes.iter())
            .position(|(a, b)| a != b)
            .expect("marker id must differ from the zeroed baseline");
        sites.push(UdtRefSite {
            id_offset,
            name: name.clone(),
        });
    }
    sites
}

/// Fills the `id` of every `ScSpecTypeUdtv2` reference in `entries` with the
/// [`canonical_id`] of the referenced type, resolved by name against the entries
/// themselves. References to names not defined in `entries` are left unchanged.
///
/// Used when a full spec is available (e.g. during code generation); the
/// soroban-sdk derive macros instead patch the same ids in at compile time.
pub fn resolve_ids(entries: &mut [ScSpecEntry]) {
    let ids: std::collections::BTreeMap<String, [u8; 8]> = entries
        .iter()
        .filter_map(|e| entry_name(e).map(|n| (n, canonical_id(e))))
        .collect();
    for e in entries.iter_mut() {
        for_each_ref(e, |u| {
            if let Some(id) = ids.get(&u.name.to_utf8_string_lossy()) {
                u.id = *id;
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::{canonical_id, resolve_ids, udt_ref_sites};
    use stellar_xdr::{
        Limits, ReadXdr, ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdtv2, ScSpecUdtStructFieldV0,
        ScSpecUdtStructV0, WriteXdr,
    };

    fn struct_a() -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "A".try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 {
                    id: [0u8; 8],
                    name: "B".try_into().unwrap(),
                }),
            }]
            .try_into()
            .unwrap(),
        })
    }

    fn struct_b() -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "B".try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "x".try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            }]
            .try_into()
            .unwrap(),
        })
    }

    #[test]
    fn resolve_sets_ref_to_referenced_type_id() {
        let mut entries = vec![struct_a(), struct_b()];
        let expected_b_id = canonical_id(&struct_b());
        resolve_ids(&mut entries);
        let ScSpecEntry::UdtStructV0(a) = &entries[0] else {
            panic!()
        };
        let ScSpecTypeDef::UdtV2(u) = &a.fields[0].type_ else {
            panic!()
        };
        assert_eq!(u.name.to_utf8_string_lossy(), "B");
        assert_eq!(u.id, expected_b_id);
    }

    #[test]
    fn ref_sites_offsets_locate_the_id() {
        let a = struct_a();
        let sites = udt_ref_sites(&a);
        assert_eq!(sites.len(), 1);
        assert_eq!(sites[0].name, "B");
        // Patching the located offset with a known value and decoding yields
        // that value in the id field.
        let mut bytes = a.to_xdr(Limits::none()).unwrap();
        let off = sites[0].id_offset;
        bytes[off..off + 8].copy_from_slice(&[9u8; 8]);
        let decoded = ScSpecEntry::from_xdr(&bytes, Limits::none()).unwrap();
        let ScSpecEntry::UdtStructV0(s) = decoded else {
            panic!()
        };
        let ScSpecTypeDef::UdtV2(u) = &s.fields[0].type_ else {
            panic!()
        };
        assert_eq!(u.id, [9u8; 8]);
    }
}
