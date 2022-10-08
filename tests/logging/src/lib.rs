#![no_std]
use soroban_sdk::{contractimpl, log, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        log!(&env, "none");
        log!(&env, "none",);
        log!(&env, "one: {}", symbol!("one"));
        log!(&env, "one: {}", symbol!("one"),);
        log!(&env, "one: {}, two: {}", symbol!("one"), symbol!("two"));
        log!(&env, "one: {}, two: {}", symbol!("one"), symbol!("two"),);
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

        env.logger().print();

        if cfg!(debug_assertions) {
            assert_eq!(
                env.logger().all(),
                std::vec![
                    "none".to_string(),
                    "none".to_string(),
                    "one: Symbol(one)".to_string(),
                    "one: Symbol(one)".to_string(),
                    "one: Symbol(one), two: Symbol(two)".to_string(),
                    "one: Symbol(one), two: Symbol(two)".to_string(),
                ],
            );
        } else {
            assert_eq!(env.logger().all(), std::vec![""; 0]);
        }
    }
}
