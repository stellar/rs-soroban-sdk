use crate as soroban_sdk;
use soroban_sdk::{contractimpl, symbol, BytesN, Env};

pub struct OuterContract;

#[contractimpl]
impl OuterContract {
    pub fn outer(env: Env, contract_id: BytesN<32>) {
        let check_call_stack = || {
            let stack = env.call_stack();
            assert_eq!(stack.len(), 1);
            let outer = stack.get(0).unwrap().unwrap();
            assert_eq!(outer.0, BytesN::from_array(&env, &[1u8; 32]));
            assert_eq!(outer.1, symbol!("outer"));
        };

        // Check before the inner call
        check_call_stack();

        let client = InnerContractClient::new(&env, &contract_id);
        client.inner();

        // Check after the inner call
        check_call_stack();
    }
}

pub struct InnerContract;

#[contractimpl]
impl InnerContract {
    pub fn inner(env: Env) {
        let stack = env.call_stack();
        assert_eq!(stack.len(), 2);

        let outer = stack.get(0).unwrap().unwrap();
        assert_eq!(outer.0, BytesN::from_array(&env, &[1u8; 32]));
        assert_eq!(outer.1, symbol!("outer"));

        let inner = stack.get(1).unwrap().unwrap();
        assert_eq!(inner.0, BytesN::from_array(&env, &[0u8; 32]));
        assert_eq!(inner.1, symbol!("inner"));
    }
}

#[test]
fn test() {
    let e = Env::default();

    let inner_contract_id = BytesN::from_array(&e, &[0; 32]);
    e.register_contract(&inner_contract_id, InnerContract);

    let contract_id = BytesN::from_array(&e, &[1; 32]);
    e.register_contract(&contract_id, OuterContract);
    let client = OuterContractClient::new(&e, &contract_id);

    client.outer(&inner_contract_id);
}
