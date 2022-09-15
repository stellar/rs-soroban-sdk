#![no_std]
use soroban_sdk::{contractimpl, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.logger().log("hello started: {}", (symbol!("hello"),));
        env.logger()
            .log("hello finished: {}, count: {}", (symbol!("bye"), 1u32));
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::string::ToString;

    use soroban_sdk::{testutils::Logger, BytesN, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_logging() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        if cfg!(debug_assertions) {
            assert_eq!(
                env.logger().all(),
                std::vec![
                    "hello started: Symbol(hello)".to_string(),
                    "hello finished: Symbol(bye), count: U32(1)".to_string(),
                ],
            );
        } else {
            assert_eq!(env.logger().all(), std::vec![""; 0]);
        }
    }
}
