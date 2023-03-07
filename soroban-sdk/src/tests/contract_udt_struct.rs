use crate as soroban_sdk;
use soroban_sdk::{contractimpl, contracttype, map, ConversionError, Env, Symbol, TryFromVal};
use stellar_xdr::{
    ReadXdr, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeTuple,
    ScSpecTypeUdt,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt {
    pub a: i32,
    pub b: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct UdtWithLongName {
    pub this_is_a_very_long_name_1234567: u64,
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }

    pub fn add_udt_with_long_name(a: UdtWithLongName, b: UdtWithLongName) -> u64 {
        a.this_is_a_very_long_name_1234567 + b.this_is_a_very_long_name_1234567
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let a = Udt { a: 5, b: 7 };
    let b = Udt { a: 10, b: 14 };
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_long_names_functional() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);

    let a = UdtWithLongName {
        this_is_a_very_long_name_1234567: 1_000_000_000_000,
    };
    let b = UdtWithLongName {
        this_is_a_very_long_name_1234567: 5_000_000_000_000,
    };
    assert_eq!(
        ContractClient::new(&env, &contract_id).add_udt_with_long_name(&a, &b),
        6_000_000_000_000
    );
}

// TODO: at present UDT try_from_vals actually trap rather than returning
// catchable errors. This is intentional to minimize code size. Can revisit.
#[test]
#[should_panic]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a map will decode to a Udt if the symbol keys match the
    // fields.
    let map = map![&env, (Symbol::short("a"), 5), (Symbol::short("b"), 7)].to_raw();
    let udt = Udt::try_from_val(&env, &map);
    assert_eq!(udt, Ok(Udt { a: 5, b: 7 }));

    // If a struct has fields a, b, and a map is decoded into it where the map
    // has fields a, b, and c, it is an error. It is an error because decoding
    // and encoding will not round trip the data, and therefore partial decoding
    // is relatively difficult to use safely.
    let map = map![
        &env,
        (Symbol::short("a"), 5),
        (Symbol::short("b"), 7),
        (Symbol::short("c"), 9)
    ]
    .to_raw();
    let udt = Udt::try_from_val(&env, &map);
    assert_eq!(udt, Err(ConversionError));
}

#[test]
fn test_spec() {
    let entries = ScSpecEntry::from_xdr(__SPEC_XDR_FN_ADD).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Udt".try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        }))]
        .try_into()
        .unwrap(),
    });
    assert_eq!(entries, expect);
}

#[test]
fn test_spec_with_long_names() {
    let entries = ScSpecEntry::from_xdr(__SPEC_XDR_FN_ADD_UDT_WITH_LONG_NAME).unwrap();
    let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
        doc: "".try_into().unwrap(),
        name: "add_udt_with_long_name".try_into().unwrap(),
        inputs: vec![
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "UdtWithLongName".try_into().unwrap(),
                }),
            },
            ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "b".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "UdtWithLongName".try_into().unwrap(),
                }),
            },
        ]
        .try_into()
        .unwrap(),
        outputs: vec![ScSpecTypeDef::U64].try_into().unwrap(),
    });
    assert_eq!(entries, expect);
}
