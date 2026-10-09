//! Demonstrates how `#[contracttype]` works on a struct with a field whose type
//! is a struct imported into scope under an alias, i.e. `use path::Type as
//! Renamed;`.
//!
//! - `test_functional` and `test_functional_with_original_type` confirm that,
//!   used directly within this crate (not via a client generated from the
//!   contract spec), the generated type round-trips through a contract call
//!   correctly, whether values are constructed using the aliased name
//!   (`Renamed`) or the type's original name (`inner::Inner`) — they are the
//!   same type, so Rust's own type checking is what makes this work.
//! - `test_spec` confirms that the spec for the field refers to the type by
//!   the fully qualified name of its definition (`inner::Inner`), not by the
//!   alias written at the field-declaration site (`Renamed`). The macros only
//!   see the alias, so the field takes its spec type from the type the alias
//!   resolves to, and that type's own spec entry has the same name. The field
//!   reference therefore matches up with the type's entry, and anything that
//!   regenerates a client from the spec (e.g. `contractimport!` in another
//!   crate) sees one type.

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
        name: "::soroban_sdk::tests::contract_udt_struct_aliased_import::Outer"
            .try_into()
            .unwrap(),
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
                    // Named after the type's definition, not the alias, so
                    // it matches the type's own entry (below).
                    name: "::soroban_sdk::tests::contract_udt_struct_aliased_import::inner::Inner"
                        .try_into()
                        .unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);

    // Renamed's own spec entry is generated under its original definition
    // name, the same name the field above refers to.
    let entries = ScSpecEntry::from_xdr(Renamed::spec_xdr(), Limits::none()).unwrap();
    let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
        doc: "".try_into().unwrap(),
        lib: "".try_into().unwrap(),
        name: "::soroban_sdk::tests::contract_udt_struct_aliased_import::inner::Inner"
            .try_into()
            .unwrap(),
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
