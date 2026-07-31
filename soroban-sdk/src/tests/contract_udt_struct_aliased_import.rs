use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contractimpl, contracttype, Env};
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtStructFieldV0,
    ScSpecUdtStructV0,
};

mod inner {
    use crate::{self as soroban_sdk};
    use soroban_sdk::contracttype;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[contracttype]
    pub struct Inner {
        pub a: i32,
        pub b: i32,
    }
}

use inner::Inner as Renamed;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Outer {
    pub inner: Renamed,
    pub c: i32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Outer, b: Outer) -> (Outer, Outer) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());

    let a = Outer {
        inner: Renamed { a: 5, b: 7 },
        c: 1,
    };
    let b = Outer {
        inner: Renamed { a: 10, b: 14 },
        c: 2,
    };
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_functional_with_original_type() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());

    let a = Outer {
        inner: inner::Inner { a: 5, b: 7 },
        c: 1,
    };
    let b = Outer {
        inner: inner::Inner { a: 10, b: 14 },
        c: 2,
    };
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::from_xdr(Outer::spec_xdr(), Limits::none()).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        name: "Outer".try_into().unwrap(),
        fields: vec![
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "c".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "inner".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    // The field is declared using the aliased import name,
                    // so that is the name recorded in the spec, not the
                    // name of the type at its original definition site.
                    name: "Renamed".try_into().unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);

    // The aliased import, Renamed, is the same type as inner::Inner, so its
    // spec is registered under the type's original name, Inner, not the
    // aliased name used at the field-declaration site above.
    let entries = ScSpecEntry::from_xdr(Renamed::spec_xdr(), Limits::none()).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        name: "Inner".try_into().unwrap(),
        fields: vec![
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
            ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::I32,
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);
}
