use crate as soroban_sdk;
use soroban_sdk::contracttype;
use stellar_xdr::{ReadXdr, ScSpecEntry, ScSpecUdtStructFieldV0, ScSpecUdtStructV0};

/// S holds two u64s.
#[contracttype]
pub struct S(
    /// first
    u64,
    /// second
    u64,
);

#[test]
fn test_spec() {
    let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_S).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "S holds two u64s.".try_into().unwrap(),
        name: "S".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        fields: [
            ScSpecUdtStructFieldV0 {
                doc: "first".try_into().unwrap(),
                name: "0".try_into().unwrap(),
                type_: stellar_xdr::ScSpecTypeDef::U64,
            },
            ScSpecUdtStructFieldV0 {
                doc: "second".try_into().unwrap(),
                name: "1".try_into().unwrap(),
                type_: stellar_xdr::ScSpecTypeDef::U64,
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entry, expect);
}
