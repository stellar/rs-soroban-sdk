extern crate std;

use std::collections::HashSet;
use std::string::String;
use stellar_xdr::ScSpecEntry;

// Built without LTO by the build-test-wasms make target.
const WASM: &[u8] = include_bytes!("../../../target/wasm32v1-none/release/test_spec_no_lto.wasm");

#[test]
fn test_spec_entries_of_types_in_other_crates_are_linked_without_lto() {
    let entries = soroban_spec::read::from_wasm(WASM).unwrap();
    let defined: HashSet<String> = entries
        .iter()
        .filter_map(|e| match e {
            ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
            ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
            _ => None,
        })
        .collect();

    // Types used directly by the contract's functions.
    assert!(defined.contains("::test_spec_lib_no_lto::IntEnum"));
    assert!(defined.contains("::test_spec_lib_no_lto::Enum"));
    assert!(defined.contains("::test_spec_lib_no_lto::Tuple"));
    assert!(defined.contains("::test_spec_lib_no_lto::Outer"));
    // A type only used as the field of another type in the same crate.
    assert!(defined.contains("::test_spec_lib_no_lto::Inner"));
    // A type only used as the field of a type in the contract crate.
    assert!(defined.contains("::test_spec_lib_no_lto::Wrapped"));
    assert!(defined.contains("::test_spec_no_lto::Wrapper"));
}
