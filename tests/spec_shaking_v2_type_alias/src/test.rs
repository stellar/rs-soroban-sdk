extern crate std;

use soroban_sdk::xdr::{ScSpecEntry, ScSpecTypeDef};
use std::collections::HashSet;
use std::string::String;
use std::vec::Vec;

const WASM: &[u8] =
    include_bytes!("../../../target/wasm32v1-none/release/test_spec_shaking_v2_type_alias.wasm");

/// Read the spec, shake it, and return (kept entries, names of kept UDT entries).
fn shaken() -> (Vec<ScSpecEntry>, HashSet<String>) {
    let entries = soroban_spec::read::from_wasm(WASM).unwrap();
    let markers = soroban_spec::shaking::find_all(WASM);
    let kept: Vec<_> = soroban_spec::shaking::filter(entries.iter().cloned(), &markers).collect();
    let names = kept
        .iter()
        .filter_map(|e| match e {
            ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
            _ => None,
        })
        .collect();
    (kept, names)
}

/// Names of all UDTs a type references, walking through containers.
fn collect_udts(t: &ScSpecTypeDef, out: &mut HashSet<String>) {
    match t {
        ScSpecTypeDef::Udt(u) => {
            out.insert(u.name.to_utf8_string_lossy());
        }
        ScSpecTypeDef::Option(o) => collect_udts(&o.value_type, out),
        ScSpecTypeDef::Result(r) => {
            collect_udts(&r.ok_type, out);
            collect_udts(&r.error_type, out);
        }
        ScSpecTypeDef::Vec(v) => collect_udts(&v.element_type, out),
        ScSpecTypeDef::Map(m) => {
            collect_udts(&m.key_type, out);
            collect_udts(&m.value_type, out);
        }
        ScSpecTypeDef::Tuple(t) => t.value_types.iter().for_each(|e| collect_udts(e, out)),
        _ => {}
    }
}

/// Assert every UDT referenced by the named function resolves to a kept entry.
fn assert_fn_refs_resolve(fn_name: &str) {
    let (kept, present) = shaken();
    let f = kept
        .iter()
        .find_map(|e| match e {
            ScSpecEntry::FunctionV0(f) if f.name.to_utf8_string_lossy() == fn_name => Some(f),
            _ => None,
        })
        .unwrap_or_else(|| panic!("function `{fn_name}` not found in spec"));
    let mut refs = HashSet::new();
    f.inputs.iter().for_each(|i| collect_udts(&i.type_, &mut refs));
    f.outputs.iter().for_each(|o| collect_udts(o, &mut refs));
    for r in refs {
        assert!(
            present.contains(&r),
            "function `{fn_name}` references UDT `{r}`, but no such entry survives shaking"
        );
    }
}

// A Rust type alias for a contract type used as a function parameter resolves,
// via the marker, to the real type `Item`, so the surviving spec entry is named
// `Item`. But the function spec is built from the syntactic token `ItemAlias`,
// leaving a dangling reference to a UDT that has no entry.
#[test]
fn alias_to_udt_param() {
    assert_fn_refs_resolve("use_udt_alias");
}

// A type alias to a primitive is also mapped by its syntactic name, so the
// function spec references a UDT named `Amount` that has no entry at all (the
// real type, `i128`, is a primitive with no UDT entry).
#[test]
fn alias_to_primitive_param() {
    assert_fn_refs_resolve("use_primitive_alias");
}

// A type alias to a container is mapped by its syntactic name too, so the
// function spec references a UDT named `Items` that has no entry (the real type
// is `Vec<Item>`, a container). The element `Item` is kept, but `Items` dangles.
#[test]
fn alias_to_container_param() {
    assert_fn_refs_resolve("use_container_alias");
}
