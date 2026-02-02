use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl, contracttype, Env, IntoVal, Map, Symbol, TryIntoVal, Val,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct StructWithOptions {
    pub required: i32,
    pub optional_a: Option<i32>,
    pub optional_b: Option<i32>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn roundtrip(s: StructWithOptions) -> StructWithOptions {
        s
    }
}

#[test]
fn test_option_none_omitted_from_map() {
    let env = Env::default();

    // Test struct with None values
    let s = StructWithOptions {
        required: 42,
        optional_a: None,
        optional_b: Some(100),
    };

    // Convert to Val
    let val: Val = s.clone().into_val(&env);

    // Convert to Map to inspect contents
    let map: Map<Symbol, Val> = val.try_into_val(&env).unwrap();

    // Should have 2 keys: required and optional_b (not optional_a since it's None)
    assert_eq!(map.len(), 2);
    assert!(map.contains_key(Symbol::new(&env, "required")));
    assert!(!map.contains_key(Symbol::new(&env, "optional_a")));
    assert!(map.contains_key(Symbol::new(&env, "optional_b")));
}

#[test]
fn test_all_options_none_omitted() {
    let env = Env::default();

    // Test struct with all Options as None
    let s = StructWithOptions {
        required: 42,
        optional_a: None,
        optional_b: None,
    };

    // Convert to Val
    let val: Val = s.clone().into_val(&env);

    // Convert to Map to inspect contents
    let map: Map<Symbol, Val> = val.try_into_val(&env).unwrap();

    // Should only have 1 key: required
    assert_eq!(map.len(), 1);
    assert!(map.contains_key(Symbol::new(&env, "required")));
    assert!(!map.contains_key(Symbol::new(&env, "optional_a")));
    assert!(!map.contains_key(Symbol::new(&env, "optional_b")));
}

#[test]
fn test_all_options_some_included() {
    let env = Env::default();

    // Test struct with all Options as Some
    let s = StructWithOptions {
        required: 42,
        optional_a: Some(10),
        optional_b: Some(20),
    };

    // Convert to Val
    let val: Val = s.clone().into_val(&env);

    // Convert to Map to inspect contents
    let map: Map<Symbol, Val> = val.try_into_val(&env).unwrap();

    // Should have 3 keys: all fields present
    assert_eq!(map.len(), 3);
    assert!(map.contains_key(Symbol::new(&env, "required")));
    assert!(map.contains_key(Symbol::new(&env, "optional_a")));
    assert!(map.contains_key(Symbol::new(&env, "optional_b")));
}

#[test]
fn test_missing_key_becomes_none() {
    let env = Env::default();

    // Create a map with only the required field
    let mut map: Map<Symbol, Val> = Map::new(&env);
    map.set(Symbol::new(&env, "required"), 42i32.into_val(&env));

    // Convert to struct - missing optional fields should become None
    let val: Val = map.into();
    let s: StructWithOptions = val.try_into_val(&env).unwrap();

    assert_eq!(s.required, 42);
    assert_eq!(s.optional_a, None);
    assert_eq!(s.optional_b, None);
}

#[test]
fn test_backwards_compat_void_becomes_none() {
    let env = Env::default();

    // Create a map with VOID for optional field (simulating old serialization format)
    let mut map: Map<Symbol, Val> = Map::new(&env);
    map.set(Symbol::new(&env, "required"), 42i32.into_val(&env));
    map.set(Symbol::new(&env, "optional_a"), Val::VOID.into());
    // optional_b is missing

    // Convert to struct - VOID should become None, missing key should also become None
    let val: Val = map.into();
    let s: StructWithOptions = val.try_into_val(&env).unwrap();

    assert_eq!(s.required, 42);
    assert_eq!(s.optional_a, None);
    assert_eq!(s.optional_b, None);
}

#[test]
fn test_partial_options_present() {
    let env = Env::default();

    // Create a map with one optional field present
    let mut map: Map<Symbol, Val> = Map::new(&env);
    map.set(Symbol::new(&env, "required"), 42i32.into_val(&env));
    map.set(Symbol::new(&env, "optional_b"), 100i32.into_val(&env));
    // optional_a is missing

    // Convert to struct
    let val: Val = map.into();
    let s: StructWithOptions = val.try_into_val(&env).unwrap();

    assert_eq!(s.required, 42);
    assert_eq!(s.optional_a, None);
    assert_eq!(s.optional_b, Some(100));
}

#[test]
fn test_roundtrip_with_none() {
    let env = Env::default();

    let original = StructWithOptions {
        required: 42,
        optional_a: None,
        optional_b: Some(100),
    };

    // Convert to Val and back
    let val: Val = original.clone().into_val(&env);
    let recovered: StructWithOptions = val.try_into_val(&env).unwrap();

    assert_eq!(original, recovered);
}

#[test]
fn test_roundtrip_all_none() {
    let env = Env::default();

    let original = StructWithOptions {
        required: 42,
        optional_a: None,
        optional_b: None,
    };

    // Convert to Val and back
    let val: Val = original.clone().into_val(&env);
    let recovered: StructWithOptions = val.try_into_val(&env).unwrap();

    assert_eq!(original, recovered);
}

#[test]
fn test_roundtrip_all_some() {
    let env = Env::default();

    let original = StructWithOptions {
        required: 42,
        optional_a: Some(10),
        optional_b: Some(20),
    };

    // Convert to Val and back
    let val: Val = original.clone().into_val(&env);
    let recovered: StructWithOptions = val.try_into_val(&env).unwrap();

    assert_eq!(original, recovered);
}

#[test]
fn test_contract_roundtrip() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Test with Some values
    let input = StructWithOptions {
        required: 42,
        optional_a: Some(10),
        optional_b: None,
    };

    let output = client.roundtrip(&input);
    assert_eq!(output, input);
}

#[test]
fn test_contract_roundtrip_all_none() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    // Test with all None values
    let input = StructWithOptions {
        required: 42,
        optional_a: None,
        optional_b: None,
    };

    let output = client.roundtrip(&input);
    assert_eq!(output, input);
}
