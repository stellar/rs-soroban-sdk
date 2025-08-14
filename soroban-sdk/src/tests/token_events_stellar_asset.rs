use crate::{self as soroban_sdk, IntoVal};

use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _},
    token::StellarAssetClient,
    token::{SetAdmin, SetAuthorized},
    vec, Address, Env, Symbol,
};

#[contract]
struct Contract;

#[test]
fn test_set_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);

    let event = SetAdmin {
        admin: admin.clone(),
        new_admin: new_admin.clone(),
    };

    // Verify the event publishes the expected topics and data.
    let topics = (symbol_short!("set_admin"), admin.clone());
    let data = new_admin.clone();

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1) = topics;
    let topics = (t0, t1, client.name());

    client.set_admin(&new_admin);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}

#[test]
fn test_set_authorized() {
    let env = Env::default();
    env.mock_all_auths();

    let authorizee = Address::generate(&env);
    let authorize = true;

    let event = SetAuthorized {
        id: authorizee.clone(),
        authorize: true,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (Symbol::new(&env, "set_authorized"), authorizee.clone());
    let data = authorize;

    let id = env.register(Contract, ());
    env.as_contract(&id, || event.publish(&env));
    let token_events = env.events().all();
    assert_eq!(
        token_events,
        vec![
            &env,
            (id.clone(), topics.into_val(&env), data.into_val(&env)),
        ]
    );

    // Verify the event published is consistent with the asset contract.
    let admin = Address::generate(&env);
    let asset = env.register_stellar_asset_contract_v2(admin);
    let client = StellarAssetClient::new(&env, &asset.address());

    let (t0, t1) = topics;
    let topics = (t0, t1, client.name());

    client.set_authorized(&authorizee, &authorize);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (asset.address(), topics.into_val(&env), data.into_val(&env)),
        ]
    );
}
