use crate::{SetAdmin, SetAuthorized};
use soroban_sdk::{
    contract, symbol_short,
    testutils::{Address as _, Events as _},
    token::StellarAssetClient,
    vec, Address, Env, IntoVal, Symbol,
};

#[contract]
struct Contract;

#[test]
fn test_set_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);

    // Register the asset to get the SEP-0011 asset string it uses in events.
    let asset = env.register_stellar_asset_contract_v2(admin.clone());
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = SetAdmin {
        admin: admin.clone(),
        sep0011_asset: sep0011_asset.clone(),
        new_admin: new_admin.clone(),
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        symbol_short!("set_admin"),
        admin.clone(),
        sep0011_asset.clone(),
    );
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
    client.set_admin(&new_admin);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (symbol_short!("set_admin"), admin, sep0011_asset).into_val(&env),
                new_admin.into_val(&env),
            ),
        ]
    );
}

#[test]
fn test_set_authorized() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let authorizee = Address::generate(&env);
    let authorize = true;

    // Register the asset to get the SEP-0011 asset string it uses in events.
    let asset = env.register_stellar_asset_contract_v2(admin.clone());
    let client = StellarAssetClient::new(&env, &asset.address());
    let sep0011_asset = client.name();

    let event = SetAuthorized {
        id: authorizee.clone(),
        sep0011_asset: sep0011_asset.clone(),
        authorize,
    };

    // Verify the event publishes the expected topics and data.
    let topics = (
        Symbol::new(&env, "set_authorized"),
        authorizee.clone(),
        sep0011_asset.clone(),
    );
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
    client.set_authorized(&authorizee, &authorize);
    let asset_events = env.events().all();
    assert_eq!(
        asset_events,
        vec![
            &env,
            (
                asset.address(),
                (
                    Symbol::new(&env, "set_authorized"),
                    authorizee,
                    sep0011_asset
                )
                    .into_val(&env),
                authorize.into_val(&env),
            ),
        ]
    );
}
