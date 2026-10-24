use crate::{testutils::Address as _, token::TokenClient, Address, Env};

#[test]
fn test_native_sac_register_idempotent() {
    let env = Env::default();
    let native1 = env.register_stellar_asset_contract_native();
    let native2 = env.register_stellar_asset_contract_native();

    assert_eq!(native1.address(), native2.address());
}

#[test]
fn test_native_sac_set_balance_contract_address() {
    let env = Env::default();
    let native = env.register_stellar_asset_contract_native();
    let token_client = TokenClient::new(&env, &native.address());

    let c_addr = Address::generate(&env);
    assert_eq!(token_client.balance(&c_addr), 0);

    native.set_balance(&c_addr, 100_000_000);
    assert_eq!(token_client.balance(&c_addr), 100_000_000);

    // Update existing balance
    native.set_balance(&c_addr, 250_000_000);
    assert_eq!(token_client.balance(&c_addr), 250_000_000);

    // Set to 0
    native.set_balance(&c_addr, 0);
    assert_eq!(token_client.balance(&c_addr), 0);
}

#[test]
fn test_native_sac_set_balance_account_address() {
    let env = Env::default();
    let native = env.register_stellar_asset_contract_native();
    let token_client = TokenClient::new(&env, &native.address());

    let g_addr = Address::from_str(
        &env,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
    );
    assert!(token_client.try_balance(&g_addr).is_err());

    native.set_balance(&g_addr, 500_000_000);
    assert_eq!(token_client.balance(&g_addr), 500_000_000);

    // Update existing balance
    native.set_balance(&g_addr, 123_456_789);
    assert_eq!(token_client.balance(&g_addr), 123_456_789);
}

#[test]
fn test_native_sac_cap73_transfer_flow() {
    let env = Env::default();
    let native = env.register_stellar_asset_contract_native();

    let funder = Address::generate(&env);
    native.set_balance(&funder, 100_000_000); // 10 XLM

    let recipient = Address::from_str(
        &env,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
    );

    // CAP-73: transfer auto-creates the recipient G-account if it does not exist.
    env.mock_all_auths();
    let token_client = TokenClient::new(&env, &native.address());
    token_client.transfer(&funder, &recipient, &50_000_000);

    assert_eq!(token_client.balance(&funder), 50_000_000);
    assert_eq!(token_client.balance(&recipient), 50_000_000);
}

#[test]
fn test_credit_sac_set_balance_contract_address() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let token_client = TokenClient::new(&env, &sac.address());

    let c_addr = Address::generate(&env);
    assert_eq!(token_client.balance(&c_addr), 0);

    sac.set_balance(&c_addr, 300_000);
    assert_eq!(token_client.balance(&c_addr), 300_000);

    // Update existing balance
    sac.set_balance(&c_addr, 700_000);
    assert_eq!(token_client.balance(&c_addr), 700_000);
}

#[test]
fn test_credit_sac_set_balance_account_address() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let token_client = TokenClient::new(&env, &sac.address());

    let g_addr = Address::from_str(
        &env,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
    );
    assert!(token_client.try_balance(&g_addr).is_err());

    sac.set_balance(&g_addr, 450_000);
    assert_eq!(token_client.balance(&g_addr), 450_000);

    // Update existing balance
    sac.set_balance(&g_addr, 900_000);
    assert_eq!(token_client.balance(&g_addr), 900_000);
}

#[test]
fn test_credit_sac_transfer_between_contract_and_account() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let token_client = TokenClient::new(&env, &sac.address());

    let c_addr = Address::generate(&env);
    let g_addr = Address::from_str(
        &env,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
    );

    sac.set_balance(&c_addr, 1_000_000);
    sac.set_balance(&g_addr, 500_000);

    env.mock_all_auths();
    token_client.transfer(&c_addr, &g_addr, &200_000);

    assert_eq!(token_client.balance(&c_addr), 800_000);
    assert_eq!(token_client.balance(&g_addr), 700_000);
}

#[test]
#[should_panic(expected = "balance must be non-negative")]
fn test_set_balance_negative_panics() {
    let env = Env::default();
    let native = env.register_stellar_asset_contract_native();
    let c_addr = Address::generate(&env);
    native.set_balance(&c_addr, -1);
}

#[test]
#[should_panic(expected = "amount exceeds maximum balance for account (i64::MAX)")]
fn test_set_balance_account_overflow_panics() {
    let env = Env::default();
    let native = env.register_stellar_asset_contract_native();
    let g_addr = Address::from_str(
        &env,
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
    );
    native.set_balance(&g_addr, i64::MAX as i128 + 1);
}
