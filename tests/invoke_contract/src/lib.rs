#![no_std]
use soroban_sdk::{contractimpl, symbol, vec, BytesN, Env, IntoVal};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: i32, y: i32, contract_id: BytesN<32>) -> i32 {
        env.invoke_contract(
            &contract_id,
            &symbol!("add"),
            vec![&env, x.into_env_val(&env), y.into_env_val(&env)],
        )
    }
}

pub struct AddContract;

#[contractimpl]
impl AddContract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{BytesN, Env};

    use crate::{add_with, AddContract, Contract};

    #[test]
    fn test_add() {
        let e = Env::default();
        let add_contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&add_contract_id, AddContract);
        let contract_id = BytesN::from_array(&e, &[1; 32]);
        e.register_contract(&contract_id, Contract);

        let x = 10i32;
        let y = 12i32;
        let z = add_with::invoke(&e, &contract_id, &x, &y, &add_contract_id);
        assert!(z == 22);
    }
}
