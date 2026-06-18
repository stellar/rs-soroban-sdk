use crate::{self as soroban_sdk};
use soroban_sdk::{contracttype, symbol_short, vec, Env, IntoVal, Symbol, ToVal, Val, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Udt {
    pub a: i32,
    pub b: i32,
}

#[test]
fn test_to_val_primitive() {
    let env = Env::default();
    let v = 1u32;
    let to_val: Val = v.to_val(&env);
    let into_val: Val = v.into_val(&env);
    assert!(to_val.shallow_eq(&into_val));
}

#[test]
fn test_to_val_host_type() {
    let env = Env::default();
    let s: Symbol = symbol_short!("hello");
    let to_val: Val = ToVal::to_val(&s, &env);
    let into_val: Val = s.into_val(&env);
    assert!(to_val.shallow_eq(&into_val));

    let v: Vec<u32> = vec![&env, 1, 2, 3];
    let to_val: Val = ToVal::to_val(&v, &env);
    let into_val: Val = v.into_val(&env);
    assert!(to_val.shallow_eq(&into_val));
}

#[test]
fn test_to_val_udt() {
    let env = Env::default();
    let udt = Udt { a: 1, b: 2 };
    let to_val: Val = udt.to_val(&env);
    let into_val: Val = udt.into_val(&env);
    // Round-trip back to the UDT to confirm equality of the produced Val.
    let from_to: Udt = to_val.into_val(&env);
    let from_into: Udt = into_val.into_val(&env);
    assert_eq!(from_to, udt);
    assert_eq!(from_into, udt);
}
