#![no_std]
use soroban_sdk::{contractimpl, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.logger().debug("hello started: {}", symbol!("hello"));
        env.logger().debug("hello finished: {}", symbol!("bye"));
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::string::ToString;

    use soroban_sdk::{testutils::Logger, BytesN, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_debug_event() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        assert_eq!(
            env.logger().all(),
            std::vec![
                "hello started: Symbol(hello)".to_string(),
                "hello finished: Symbol(bye)".to_string(),
            ],
        );
    }
}
