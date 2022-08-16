#![no_std]
use soroban_sdk::{contractimpl, vec, BytesN, Env, IntoVal, Symbol};

const ADD_CONTRACT_ID: [u8; 32] = [0; 32];

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: i32, y: i32) -> i32 {
        env.invoke_contract(
            &BytesN::from_array(&env, ADD_CONTRACT_ID),
            &Symbol::from_str("add"),
            vec![&env, x.into_env_val(&env), y.into_env_val(&env)],
        )
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{add_with, Contract, ADD_CONTRACT_ID};

    const ADD_CONTRACT_WASM: &[u8] =
        include_bytes!("../../../target/wasm32-unknown-unknown/release/example_add_i32.wasm");

    #[test]
    fn test_add() {
        let e = Env::default();

        let add_contract_id = BytesN::from_array(&e, ADD_CONTRACT_ID);
        e.register_contract_wasm(&add_contract_id, ADD_CONTRACT_WASM);

        let contract_id = BytesN::from_array(&e, [1; 32]);
        e.register_contract(&contract_id, Contract);

        let x = 10i32;
        let y = 12i32;
        let z = add_with::invoke(&e, &contract_id, &x, &y);
        assert!(z == 22);
    }
}
