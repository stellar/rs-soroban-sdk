#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, IntoVal, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // TODO: Prevent arg overlap with generated args.
    pub fn add_with(env: Env, x: i32, y: i32, contract_id: Address) -> i32 {
        env.invoke_contract(
            &contract_id,
            &Symbol::short("add"),
            vec![&env, x.into_val(&env), y.into_val(&env)],
        )
    }
}

#[contract]
pub struct AddContract;

#[contractimpl]
impl AddContract {
    pub fn add(_: Env, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{AddContract, Contract, ContractClient};

    #[test]
    fn test_add() {
        let e = Env::default();

        let add_contract_id = e.register_contract(None, AddContract);

        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let x = 10i32;
        let y = 12i32;
        let z = client.add_with(&x, &y, &add_contract_id);
        assert!(z == 22);
    }
}
