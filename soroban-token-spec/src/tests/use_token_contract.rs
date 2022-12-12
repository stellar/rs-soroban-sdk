use soroban_sdk::{contractimpl, contracttype, Account, Address, BytesN, Env, IntoVal};

mod token_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_token_spec.wasm"
    );
    pub type TokenClient = Client;
}

use token_contract::{TokenClient, TokenMetadata};

#[contracttype]
pub enum DataKey {
    Token,
}

fn get_token(e: &Env) -> BytesN<32> {
    e.data().get_unchecked(DataKey::Token).unwrap()
}

pub struct TestContract;

#[contractimpl]
impl TestContract {
    pub fn init(e: Env, salt: BytesN<32>) {
        let id = e.deployer().with_current_contract(salt).deploy_token();
        let metadata = TokenMetadata {
            name: "name".into_val(&e),
            symbol: "symbol".into_val(&e),
            decimals: 7u32,
        };
        TokenClient::new(&e, &id).init(&e.current_contract_account().address(), &metadata);

        e.data().set(DataKey::Token, id);
    }

    pub fn get_token(e: Env) -> BytesN<32> {
        get_token(&e)
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        TokenClient::new(&e, get_token(&e)).mint(&e.current_contract_account(), &to, &amount);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        TokenClient::new(&e, get_token(&e)).set_admin(&e.current_contract_account(), &new_admin);
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = BytesN::from_array(&env, &[0; 32]);
    env.register_contract(&contract_id, TestContract);
    let client = TestContractClient::new(&env, &contract_id);

    let salt = BytesN::from_array(&env, &[1; 32]);
    client.init(&salt);

    let token_client = TokenClient::new(&env, &client.get_token());
    assert_eq!(token_client.name(), "name".into_val(&env));

    let acc = Account::random(&env);

    client.mint(&acc.address(), &10);
    assert_eq!(token_client.balance(&acc.address()), 10);

    // transfer admin
    client.set_admin(&acc.address());

    token_client.mint(&acc, &acc.address(), &20);
    // Smoke test check that authorization with wrong args didn't happen.
    assert!(!env.verify_account_authorization(
        &acc,
        &[(&token_client.contract_id, "mint")],
        (acc.address(), 19_i128).into_val(&env),
    ));
    assert!(env.verify_account_authorization(
        &acc,
        &[(&token_client.contract_id, "mint")],
        (acc.address(), 20_i128).into_val(&env),
    ));
    // Smoke test check that double authorization didn't happen.
    assert!(!env.verify_account_authorization(
        &acc,
        &[(&token_client.contract_id, "mint")],
        (acc.address(), 20_i128).into_val(&env),
    ));

    assert_eq!(token_client.balance(&acc.address()), 30);
}
