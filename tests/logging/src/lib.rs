#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        log!(&env, "none");
        log!(&env, "none",);
        log!(&env, "one: {}", Symbol::short("one"));
        log!(&env, "one: {}", Symbol::short("one"),);
        log!(
            &env,
            "one: {}, two: {}",
            Symbol::short("one"),
            Symbol::short("two")
        );
        log!(
            &env,
            "one: {}, two: {}",
            Symbol::short("one"),
            Symbol::short("two"),
        );
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::string::ToString;

    use soroban_sdk::{testutils::Logger, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_logging() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
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
