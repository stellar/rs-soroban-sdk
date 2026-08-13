use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl, contracttype, map, symbol_short, Env, Map, Symbol, TryFromVal, Val,
};
use stellar_xdr::{ScMap, ScMapEntry, ScSymbol, ScVal};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt {
    pub a: i32,
    pub b: Option<i32>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: Udt, b: Udt) -> (Udt, Udt) {
        (a, b)
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());

    let a = Udt { a: 5, b: None };
    let b = Udt { a: 10, b: Some(1) };
    let c = ContractClient::new(&env, &contract_id).add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_missing_option_field_decodes_as_none() {
    let env = Env::default();

    // A field missing from the map decodes as void, and void decodes into an
    // Option as None. This allows a contract to decode data that was stored by
    // an older version of the contract that did not have the field.
    let map = map![&env, (symbol_short!("a"), 5)].to_val();
    let udt = Udt::try_from_val(&env, &map);
    assert_eq!(udt, Ok(Udt { a: 5, b: None }));
}

#[test]
fn test_missing_option_field_decodes_as_none_scval() {
    let env = Env::default();

    // Conversion from an ScVal behaves the same as conversion from a Val.
    let scval = ScVal::Map(Some(
        ScMap::sorted_from(vec![ScMapEntry {
            key: ScVal::Symbol(ScSymbol("a".try_into().unwrap())),
            val: ScVal::I32(5),
        }])
        .unwrap(),
    ));
    let udt = Udt::try_from_val(&env, &scval);
    assert_eq!(udt, Ok(Udt { a: 5, b: None }));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct UdtAllOptional {
    pub a: Option<i32>,
}

#[test]
fn test_all_option_fields_decode_from_empty_map() {
    let env = Env::default();

    // A struct where every field is an Option decodes from an empty map, since
    // every field is absent and so decodes as None.
    let map = Map::<Symbol, Val>::new(&env).to_val();
    let udt = UdtAllOptional::try_from_val(&env, &map);
    assert_eq!(udt, Ok(UdtAllOptional { a: None }));
}
