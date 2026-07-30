//! Computes the `id` field of [`ScSpecTypeUdtv2`] references.
//!
//! A `ScSpecTypeUdtv2` reference identifies another user-defined type solely by
//! an 8-byte `id`. The id is the truncated (first 8 bytes) SHA256 of the
//! referenced type's own [`ScSpecEntry`], computed over a canonical form in
//! which all `UdtV2` ids the entry itself contains are zeroed. Zeroing the ids
//! in the preimage keeps the identity independent of other types' ids, so it is
//! well-defined even for mutually- or self-recursive types.
//!
//! So for `struct A { b: B }`, the field `b` in `A`'s spec entry is a
//! `ScSpecTypeUdtv2 { id: canonical_id(B's entry) }`.

use sha2::{Digest, Sha256};
use stellar_xdr::{
    Limits, ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdtv2, ScSpecUdtUnionCaseV0, WriteXdr,
};

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

/// Calls `f` on every `ScSpecTypeUdtv2` reference contained in `entry`.
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

/// The XDR of `entry` in canonical form: every `ScSpecTypeUdtv2` id zeroed.
pub fn canonical_xdr(entry: &ScSpecEntry) -> Vec<u8> {
    let mut e = entry.clone();
    for_each_ref(&mut e, |u| u.id = [0u8; 8]);
    e.to_xdr(Limits::none()).unwrap()
}

/// The identity of the type `entry` defines: the truncated 8-byte SHA256 of its
/// canonical (all-ids-zeroed) XDR. This is the value carried by every
/// `ScSpecTypeUdtv2` that references this type.
pub fn canonical_id(entry: &ScSpecEntry) -> [u8; 8] {
    Sha256::digest(canonical_xdr(entry))[..8]
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::canonical_id;
    use stellar_xdr::{
        ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdtv2, ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
    };

    fn struct_a(ref_id: [u8; 8]) -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "A".try_into().unwrap(),
            fields: vec![ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 { id: ref_id }),
            }]
            .try_into()
            .unwrap(),
        })
    }

    #[test]
    fn id_independent_of_ref_ids() {
        assert_eq!(
            canonical_id(&struct_a([9u8; 8])),
            canonical_id(&struct_a([0u8; 8]))
        );
    }
}
