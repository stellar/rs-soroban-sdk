use crate as soroban_sdk;
use soroban_sdk::contracterror;
use stellar_xdr::{ReadXdr, ScSpecEntry, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0};

/// E has variants A and B.
#[contracterror]
#[derive(Copy, Clone)]
pub enum E {
    /// A is a.
    A = 1,
    /// B is b.
    B = 2,
}

#[test]
fn test_spec() {
    let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_E).unwrap();
    let expect = ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
        doc: "E has variants A and B.".try_into().unwrap(),
        name: "E".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        cases: [
            ScSpecUdtErrorEnumCaseV0 {
                doc: "A is a.".try_into().unwrap(),
                name: "A".try_into().unwrap(),
                value: 1,
            },
            ScSpecUdtErrorEnumCaseV0 {
                doc: "B is b.".try_into().unwrap(),
                name: "B".try_into().unwrap(),
                value: 2,
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entry, expect);
}
