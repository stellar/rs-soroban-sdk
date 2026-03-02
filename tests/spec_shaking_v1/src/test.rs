extern crate std;

use soroban_sdk::xdr::ScSpecEntry;
use std::collections::HashSet;
use std::vec::Vec;

const WASM: &[u8] =
    include_bytes!("../../../target/wasm32v1-none/release/test_spec_shaking_v1.wasm");

#[test]
fn test_spec_shaking_v1() {
    // Read all spec entries from the WASM.
    let entries = soroban_spec::read::from_wasm(WASM).unwrap();

    // No markers should be embedded without the experimental feature.
    let markers = soroban_spec::shaking::find_all(WASM);
    assert!(
        markers.is_empty(),
        "no markers should be present without experimental_spec_shaking_v2, found {}",
        markers.len()
    );

    let all_names: HashSet<std::string::String> = entries.iter().filter_map(entry_name).collect();

    // All functions should be present in the spec.
    let fn_names: Vec<std::string::String> = entries
        .iter()
        .filter_map(|e| {
            if let ScSpecEntry::FunctionV0(f) = e {
                Some(f.name.to_utf8_string_lossy())
            } else {
                None
            }
        })
        .collect();
    for expected_fn in [
        "with_param",
        "with_return",
        "with_error",
        "with_vec",
        "with_map",
        "publish_simple",
        "publish_topic_type",
        "publish_data_type",
        "publish_nested_topic",
        "publish_nested_data",
        "publish_ref_event",
        "with_imported",
        "with_non_pub",
        "with_non_pub_error",
    ] {
        assert!(
            fn_names.contains(&expected_fn.into()),
            "fn {expected_fn} missing"
        );
    }

    // Public types should have spec entries.
    let pub_types = [
        "UsedParamStruct",
        "UsedReturnEnum",
        "UsedParamIntEnum",
        "UsedErrorEnum",
        "UsedNestedInStruct",
        "UsedVecElement",
        "UsedMapKey",
        "UsedMapVal",
        "UsedEventSimple",
        "UsedEventTopicType",
        "UsedEventWithTopicType",
        "UsedEventDataType",
        "UsedEventWithDataType",
        "UsedEventTopicOuter",
        "UsedEventTopicInner",
        "UsedEventWithNestedTopic",
        "UsedEventDataOuter",
        "UsedEventDataInner",
        "UsedEventWithNestedData",
        "UsedRefTopicType",
        "UsedRefDataType",
        "UsedRefDataInner",
        "UsedEventWithRefs",
        "UnusedStruct",
        "UnusedEnum",
        "UnusedIntEnum",
        "UnusedEvent",
    ];
    for name in pub_types {
        assert!(
            all_names.contains(name),
            "pub type/event {name} should have a spec entry without the feature"
        );
    }

    // Non-pub types should NOT have spec entries without the feature.
    let non_pub_types = [
        "UsedNonPubStruct",
        "UsedNonPubError",
        "UnusedNonPubStruct",
        "UnusedNonPubError",
    ];
    for name in non_pub_types {
        assert!(
            !all_names.contains(name),
            "non-pub type {name} should NOT have a spec entry without the feature"
        );
    }

    // Imported types should NOT have spec entries (export = false without the feature).
    let imported_types = [
        "StructA",
        "StructB",
        "StructTupleA",
        "StructTupleB",
        "EnumA",
        "EnumB",
        "EnumIntA",
        "EnumIntB",
        "ErrorA",
        "ErrorB",
        "ErrorC",
        "EventA",
        "EventB",
    ];
    for name in imported_types {
        assert!(
            !all_names.contains(name),
            "imported type {name} should NOT have a spec entry without the feature"
        );
    }
}

/// Extract the name from a non-function spec entry.
fn entry_name(entry: &ScSpecEntry) -> Option<std::string::String> {
    match entry {
        ScSpecEntry::FunctionV0(_) => None,
        ScSpecEntry::UdtStructV0(s) => Some(s.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtUnionV0(u) => Some(u.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_utf8_string_lossy()),
        ScSpecEntry::EventV0(e) => Some(e.name.to_utf8_string_lossy()),
    }
}
