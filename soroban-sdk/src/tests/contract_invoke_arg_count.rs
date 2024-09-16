use crate as soroban_sdk;
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env, IntoVal as _};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add_with(env: Env, x: i32, y: Option<i32>, z: Option<i32>, contract_id: Address) -> i32 {
        let mut args = vec![&env, x.into_val(&env)];
        if let Some(y) = y {
            args.push_back(y.into_val(&env));
        }
        if let Some(z) = z {
            args.push_back(z.into_val(&env));
        }
        env.invoke_contract(&contract_id, &symbol_short!("add"), args)
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

#[test]
fn test_correct_arg_count() {
    let e = Env::default();

    let add_contract_id = e.register_contract(None, AddContract);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10i32;
    let y = 12i32;
    let z = client.add_with(&x, &Some(y), &None, &add_contract_id);
    assert!(z == 22);
}

#[test]
#[should_panic(expected = "invalid number of input arguments: 2 expected, got 1")]
fn test_too_few_args() {
    let e = Env::default();

    let add_contract_id = e.register_contract(None, AddContract);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10i32;
    let _ = client.add_with(&x, &None, &None, &add_contract_id);
}

#[test]
#[should_panic(expected = "invalid number of input arguments: 2 expected, got 3")]
fn test_too_many_args() {
    let e = Env::default();

    let add_contract_id = e.register_contract(None, AddContract);

    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let x = 10i32;
    let y = 12i32;
    let _ = client.add_with(&x, &Some(y), &Some(1), &add_contract_id);
}
