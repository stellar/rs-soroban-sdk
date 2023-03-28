use crate::{self as soroban_sdk, Symbol};
use soroban_sdk::{contractimpl, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn store(env: Env, k: i32, v: i32) {
        env.storage().set(&k, &v)
    }

    pub fn store_temp(env: Env, k: Symbol, v: i128) {
        env.temp_storage().set(&k, &v)
    }
}

#[test]
fn test_storage() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    client.store(&2, &4);

    assert_eq!(
        e.as_contract(&contract_id, || e.storage().get::<_, i32>(&2)),
        Some(Ok(4))
    );
}

#[test]
fn test_temp_storage() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    client.store_temp(&Symbol::new(&e, "abcdefg123456"), &(1_i128 << 126));

    assert_eq!(
        e.as_contract(&contract_id, || e
            .temp_storage()
            .get::<_, i128>(&Symbol::new(&e, "abcdefg123456"))),
        Some(Ok(1_i128 << 126))
    );
    e.reset_temp_storage();
    assert_eq!(
        e.as_contract(&contract_id, || e
            .temp_storage()
            .has(&Symbol::new(&e, "abcdefg123456"))),
        false
    );
}
