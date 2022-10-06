use crate as soroban_sdk;
use soroban_sdk::{
    contractimpl, contracttype, symbol, vec, ConversionError, Env, IntoVal, TryFromVal,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum Udt {
    Aaa,
    Bbb(i32),
}

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
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let a = Udt::Aaa;
    let b = Udt::Bbb(3);
    let c = client.add(&a, &b);
    assert_eq!(c, (a, b));
}

#[test]
fn test_error_on_partial_decode() {
    let env = Env::default();

    // Success case, a vec will decode to a Udt if the first element is the
    // variant name as a Symbol, and following elements are tuple-like values
    // for the variant.
    let vec = vec![&env, symbol!("Aaa").into_val(&env)].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Ok(Udt::Aaa));
    let vec = vec![&env, symbol!("Bbb").into_val(&env), 8.into()].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Ok(Udt::Bbb(8)));

    // If an enum has a tuple like variant with one value, but the vec has
    // multiple values, it is an error. It is an error because decoding and
    // encoding will not round trip the data, and therefore partial decoding is
    // relatively difficult to use safely.
    let vec = vec![&env, symbol!("Aaa").into_val(&env), 8.into()].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Err(ConversionError));
    let vec = vec![&env, symbol!("Bbb").into_val(&env), 8.into(), 9.into()].to_raw();
    let udt = Udt::try_from_val(&env, vec);
    assert_eq!(udt, Err(ConversionError));
}
