use crate as soroban_sdk;
use soroban_sdk::contracttype;
use stellar_xdr::{ReadXdr, ScSpecEntry, ScSpecUdtStructFieldV0, ScSpecUdtStructV0};

/// S holds a and
// TODO: Implement.
#[contracttype]
/// b.
pub struct S {
    /// a is a
    a: u64,
    /// b is b
    b: u64,
}

#[test]
fn test_spec() {
    let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_S).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "S holds a and\nb.".try_into().unwrap(),
        name: "S".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        fields: [
            ScSpecUdtStructFieldV0 {
                doc: "a is a".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: stellar_xdr::ScSpecTypeDef::U64,
            },
            ScSpecUdtStructFieldV0 {
                doc: "b is b".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: stellar_xdr::ScSpecTypeDef::U64,
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entry, expect);
}
