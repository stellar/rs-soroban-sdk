use crate::{self as soroban_sdk, IntoVal};

use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _},
    token::{SetAdmin, SetAuthorized},
    vec, Address, Env, Symbol,
};

#[contract]
struct Contract;

#[test]
fn test_set_admin() {
    let env = Env::default();
    let id = env.register(Contract, ());
    let event = SetAdmin {
        new_admin: Address::generate(&env),
    };
    env.as_contract(&id, || event.publish(&env));
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id.clone(),
                (symbol_short!("set_admin"),).into_val(&env),
                event.new_admin.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_set_authorized() {
    let env = Env::default();
    let id = env.register(Contract, ());
    let event = SetAuthorized {
        id: Address::generate(&env),
        authorize: true,
    };
    env.as_contract(&id, || event.publish(&env));
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id.clone(),
                (Symbol::new(&env, "set_authorized"), event.id.clone(),).into_val(&env),
                true.into_val(&env),
            ),
        ]
    );
}
