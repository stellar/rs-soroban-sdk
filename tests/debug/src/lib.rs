#![no_std]
use soroban_sdk::{contractimpl, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.debugger().debug("hello started: {}", symbol!("hello"));
        env.debugger().debug("hello finished: {}", symbol!("bye"));
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use alloc::string::ToString;
    use soroban_sdk::{symbol, testutils::Debugger, vec, BytesN, Env, IntoVal};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_debug_event() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        assert_eq!(
            env.debugger().all(),
            vec![
                &env,
                // Expect 2 debug events.
                (
                    "hello started: {}".to_string(),
                    symbol!("hello").into_val(&env),
                ),
                (
                    "hello finished: {}".to_string(),
                    symbol!("bye").into_val(&env)
                ),
            ],
        );
    }
}
