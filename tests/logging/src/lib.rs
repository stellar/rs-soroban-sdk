#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        log!(&env, "none");
        log!(&env, "none",);
        log!(&env, "one:", Symbol::short("one"));
        log!(&env, "one:", Symbol::short("one"),);
        log!(
            &env,
            "one and two:",
            Symbol::short("one"),
            Symbol::short("two")
        );
        log!(
            &env,
            "one and two:",
            Symbol::short("one"),
            Symbol::short("two"),
        );
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use soroban_sdk::{testutils::Logs, Env};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_logging() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        env.logs().print();

        if cfg!(debug_assertions) {
            let pats = std::vec![
                "\"none\"",
                "\"none\"",
                "[\"one:\", one]",
                "[\"one:\", one]",
                "[\"one and two:\", one, two]",
                "[\"one and two:\", one, two]"
            ];
            for (msg, pat) in env.logs().all().iter().zip(pats.iter()) {
                assert!(msg.contains(pat));
            }
        } else {
            assert_eq!(env.logs().all(), std::vec![""; 0]);
        }
    }
}
