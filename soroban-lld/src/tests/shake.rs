//! Tests for spec shaking.

use super::{custom, module};
use crate::shake::{read_xdr, shake, Outcome, XDR_DEPTH_LIMIT};
use crate::wasm::{custom_section, META_SECTION_NAME, SPEC_SECTION_NAME};
use stellar_xdr::{Limits, ScMetaEntry, ScSpecEntry, WriteXdr};
use stellar_xdr::{
    ScMetaV0, ScSpecEventDataFormat, ScSpecEventV0, ScSpecFunctionV0, ScSpecTypeDef,
    ScSpecUdtStructFieldV0, ScSpecUdtStructV0, StringM, VecM,
};

fn function(name: &str) -> ScSpecEntry {
    ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: StringM::default(),
        name: name.try_into().unwrap(),
        inputs: VecM::default(),
        outputs: VecM::default(),
    })
}

fn struct_(name: &str) -> ScSpecEntry {
    ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: StringM::default(),
        lib: StringM::default(),
        name: name.try_into().unwrap(),
        fields: vec![ScSpecUdtStructFieldV0 {
            doc: StringM::default(),
            name: "f".try_into().unwrap(),
            type_: ScSpecTypeDef::U32,
        }]
        .try_into()
        .unwrap(),
    })
}

fn event(name: &str) -> ScSpecEntry {
    ScSpecEntry::EventV0(ScSpecEventV0 {
        doc: StringM::default(),
        lib: StringM::default(),
        name: name.try_into().unwrap(),
        prefix_topics: VecM::default(),
        params: VecM::default(),
        data_format: ScSpecEventDataFormat::SingleValue,
    })
}

fn xdr<T: WriteXdr>(entries: &[T]) -> Vec<u8> {
    let mut out = Vec::new();
    for e in entries {
        out.extend(e.to_xdr(Limits::depth(XDR_DEPTH_LIMIT)).unwrap());
    }
    out
}

fn shaking_meta() -> Vec<ScMetaEntry> {
    vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
        key: soroban_spec::shaking::META_KEY.try_into().unwrap(),
        val: soroban_spec::shaking::META_VALUE_V2.try_into().unwrap(),
    })]
}

/// Builds a data section holding the markers for `live`, the way the SDK's
/// marker statics land in a linked contract.
fn data_section(live: &[&ScSpecEntry]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for entry in live {
        bytes.extend(soroban_spec::shaking::generate_marker_for_entry(entry));
    }
    let mut section = vec![0x01]; // one segment
    section.push(0x01); // passive
    section.push(u8::try_from(bytes.len()).unwrap());
    section.extend(bytes);
    section
}

fn contract(spec: &[ScSpecEntry], meta: &[ScMetaEntry], live: &[&ScSpecEntry]) -> Vec<u8> {
    module(&[
        (11, data_section(live)),
        (0, custom(SPEC_SECTION_NAME, &xdr(spec))),
        (0, custom(META_SECTION_NAME, &xdr(meta))),
    ])
}

fn spec_of(wasm: &[u8]) -> Vec<ScSpecEntry> {
    read_xdr(&custom_section(wasm, SPEC_SECTION_NAME).unwrap().unwrap()).unwrap()
}

#[test]
fn keeps_functions_and_used_types_drops_the_rest() {
    let (f, used, unused) = (function("hello"), struct_("Used"), struct_("Unused"));
    let (live_ev, dead_ev) = (event("Live"), event("Dead"));
    let spec = vec![f.clone(), used.clone(), unused, live_ev.clone(), dead_ev];
    let wasm = contract(&spec, &shaking_meta(), &[&used, &live_ev]);

    let (out, outcome) = shake(&wasm).unwrap().unwrap();
    assert_eq!(
        outcome,
        Outcome::Shaken {
            before: 5,
            after: 3
        }
    );
    // The function is kept without a marker, because functions are the API.
    assert_eq!(spec_of(&out), vec![f, used, live_ev]);
}

#[test]
fn collapses_exact_duplicates() {
    // The same type defined by two crates in the build appears twice.
    let (f, used) = (function("hello"), struct_("Used"));
    let spec = vec![f.clone(), used.clone(), used.clone()];
    let wasm = contract(&spec, &shaking_meta(), &[&used]);

    let (out, outcome) = shake(&wasm).unwrap().unwrap();
    assert_eq!(
        outcome,
        Outcome::Shaken {
            before: 3,
            after: 2
        }
    );
    assert_eq!(spec_of(&out), vec![f, used]);
}

#[test]
fn leaves_the_spec_alone_without_shaking_meta() {
    // An SDK that emits no markers. Shaking would drop every type.
    let spec = vec![function("hello"), struct_("Used")];
    let other_meta = vec![ScMetaEntry::ScMetaV0(ScMetaV0 {
        key: "rssdkver".try_into().unwrap(),
        val: "28.0.0".try_into().unwrap(),
    })];
    let wasm = contract(&spec, &other_meta, &[]);

    let (out, outcome) = shake(&wasm).unwrap().unwrap();
    assert_eq!(outcome, Outcome::Skipped);
    assert_eq!(spec_of(&out), spec);
}

#[test]
fn leaves_the_spec_alone_with_no_meta_at_all() {
    let spec = vec![function("hello"), struct_("Used")];
    let wasm = module(&[(0, custom(SPEC_SECTION_NAME, &xdr(&spec)))]);
    assert_eq!(shake(&wasm).unwrap().unwrap().1, Outcome::Skipped);
}

#[test]
fn ignores_wasm_that_is_not_a_contract() {
    // No spec section at all, as for any other wasm the linker produces.
    let wasm = module(&[(1, vec![0x01, 0x60, 0x00, 0x00])]);
    assert!(shake(&wasm).unwrap().is_none());
}

#[test]
fn drops_every_type_when_nothing_is_live() {
    let f = function("hello");
    let spec = vec![f.clone(), struct_("Unused"), event("Dead")];
    let wasm = contract(&spec, &shaking_meta(), &[]);

    let (out, outcome) = shake(&wasm).unwrap().unwrap();
    assert_eq!(
        outcome,
        Outcome::Shaken {
            before: 3,
            after: 1
        }
    );
    assert_eq!(spec_of(&out), vec![f]);
}

#[test]
fn preserves_other_sections() {
    let used = struct_("Used");
    let wasm = module(&[
        (1, vec![0x01, 0x60, 0x00, 0x00]),
        (11, data_section(&[&used])),
        (
            0,
            custom(SPEC_SECTION_NAME, &xdr(std::slice::from_ref(&used))),
        ),
        (0, custom(META_SECTION_NAME, &xdr(&shaking_meta()))),
        (0, custom("contractenvmetav0", b"envmeta")),
    ]);
    let (out, _) = shake(&wasm).unwrap().unwrap();
    assert_eq!(
        custom_section(&out, "contractenvmetav0").unwrap(),
        Some(b"envmeta".to_vec()),
    );
    assert_eq!(
        custom_section(&out, META_SECTION_NAME).unwrap(),
        Some(xdr(&shaking_meta())),
    );
    assert_eq!(spec_of(&out), vec![used]);
}
