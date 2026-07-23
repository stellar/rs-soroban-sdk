//! Shared traversal of the `ScSpecTypeDef`s reachable from a spec entry.
//!
//! Used by [`crate::resolve`] to rewrite user-defined-type references and by
//! [`crate::shaking`] to normalize them back when hashing. Keeping a single
//! traversal here ensures the two stay in agreement — a divergence would make
//! stored hashes disagree with WASM markers.

use stellar_xdr::{ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0};

/// Applies `f` to every `ScSpecTypeDef` reachable from `entry`, recursing into
/// composite types (option, result, vec, map, tuple). `f` is applied to a node
/// before its children.
pub(crate) fn walk_entry_type_defs<F: FnMut(&mut ScSpecTypeDef)>(
    entry: &mut ScSpecEntry,
    f: &mut F,
) {
    match entry {
        ScSpecEntry::FunctionV0(func) => {
            for input in func.inputs.iter_mut() {
                walk_type_def(&mut input.type_, f);
            }
            for output in func.outputs.iter_mut() {
                walk_type_def(output, f);
            }
        }
        ScSpecEntry::UdtStructV0(s) => {
            for field in s.fields.iter_mut() {
                walk_type_def(&mut field.type_, f);
            }
        }
        ScSpecEntry::UdtUnionV0(u) => {
            for case in u.cases.iter_mut() {
                if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                    for ty in t.type_.iter_mut() {
                        walk_type_def(ty, f);
                    }
                }
            }
        }
        ScSpecEntry::EventV0(e) => {
            for p in e.params.iter_mut() {
                walk_type_def(&mut p.type_, f);
            }
        }
        ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
    }
}

/// Applies `f` to a type and, after `f`, to each of its child types.
pub(crate) fn walk_type_def<F: FnMut(&mut ScSpecTypeDef)>(t: &mut ScSpecTypeDef, f: &mut F) {
    f(t);
    match t {
        ScSpecTypeDef::Option(o) => walk_type_def(&mut o.value_type, f),
        ScSpecTypeDef::Result(r) => {
            walk_type_def(&mut r.ok_type, f);
            walk_type_def(&mut r.error_type, f);
        }
        ScSpecTypeDef::Vec(v) => walk_type_def(&mut v.element_type, f),
        ScSpecTypeDef::Map(m) => {
            walk_type_def(&mut m.key_type, f);
            walk_type_def(&mut m.value_type, f);
        }
        ScSpecTypeDef::Tuple(tu) => {
            for vt in tu.value_types.iter_mut() {
                walk_type_def(vt, f);
            }
        }
        _ => {}
    }
}
