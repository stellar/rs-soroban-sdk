use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, contracttype, Env, IntoVal, TryFromVal, Val};
use stellar_xdr::ScVal;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Struct {
    pub tuple: (u32, i64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TupleStruct(pub (u32, i64));

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Enum {
    Tuple((u32, i64)),
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn echo(s: Struct, t: TupleStruct, e: Enum) -> (Struct, TupleStruct, Enum) {
        (s, t, e)
    }
}

fn values() -> (Struct, TupleStruct, Enum) {
    (
        Struct { tuple: (1, -2) },
        TupleStruct((3, -4)),
        Enum::Tuple((5, -6)),
    )
}

#[test]
fn test_val_roundtrip() {
    let env = Env::default();
    let (s, t, e) = values();

    let val: Val = s.clone().into_val(&env);
    assert_eq!(Struct::try_from_val(&env, &val), Ok(s));
    let val: Val = t.clone().into_val(&env);
    assert_eq!(TupleStruct::try_from_val(&env, &val), Ok(t));
    let val: Val = e.clone().into_val(&env);
    assert_eq!(Enum::try_from_val(&env, &val), Ok(e));
}

#[test]
fn test_scval_roundtrip() {
    let env = Env::default();
    let (s, t, e) = values();

    let scval: ScVal = (&s).try_into().unwrap();
    assert_eq!(Struct::try_from_val(&env, &scval), Ok(s));
    let scval: ScVal = (&t).try_into().unwrap();
    assert_eq!(TupleStruct::try_from_val(&env, &scval), Ok(t));
    let scval: ScVal = (&e).try_into().unwrap();
    assert_eq!(Enum::try_from_val(&env, &scval), Ok(e));
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let (s, t, e) = values();
    assert_eq!(client.echo(&s, &t, &e), (s, t, e));
}
