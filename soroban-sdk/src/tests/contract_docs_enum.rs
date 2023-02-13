use crate as soroban_sdk;
use soroban_sdk::contracttype;
use stellar_xdr::{
    ReadXdr, ScSpecEntry, ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0,
    ScSpecUdtUnionCaseVoidV0, ScSpecUdtUnionV0,
};

/// E has variants A and B.
#[contracttype]
#[derive(Copy, Clone)]
pub enum E {
    /// A is a.
    A,
    /// B is b.
    B(u64, u64),
}

#[test]
fn test_spec() {
    let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_E).unwrap();
    let expect = ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
        doc: "E has variants A and B.".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        name: "E".try_into().unwrap(),
        cases: [
            ScSpecUdtUnionCaseV0::VoidV0(ScSpecUdtUnionCaseVoidV0 {
                doc: "A is a.".try_into().unwrap(),
                name: "A".try_into().unwrap(),
            }),
            ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                doc: "B is b.".try_into().unwrap(),
                name: "B".try_into().unwrap(),
                type_: [
                    // TODO: Add docs for tuple values in union cases.
                    stellar_xdr::ScSpecTypeDef::U64,
                    stellar_xdr::ScSpecTypeDef::U64,
                ]
                .try_into()
                .unwrap(),
            }),
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entry, expect);
}
