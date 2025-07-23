use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contracttype, Env, IntoVal, Val};

#[contract]
pub struct Contract;

#[contracttype]
pub enum Flag {
    A = 0,
    B = 1,
}

#[test]
fn test_owned_to_val() {
    let env = Env::default();

    let f = Flag::A;
    let val: Val = f.into_val(&env);
    let rt: Flag = val.into_val(&env);

    let Flag::A = rt else {
        panic!("failed roundtrip");
    };
}

#[test]
fn test_ref_to_val() {
    let env = Env::default();

    let f = Flag::A;
    let val: Val = (&f).into_val(&env);
    let rt: Flag = val.into_val(&env);

    let Flag::A = rt else {
        panic!("failed roundtrip");
    };
}

#[test]
fn test_double_ref_to_val() {
    let env = Env::default();

    let f = Flag::A;
    let val: Val = (&&f).into_val(&env);
    let rt: Flag = val.into_val(&env);

    let Flag::A = rt else {
        panic!("failed roundtrip");
    };
}
