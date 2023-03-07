#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events().publish(
            (Symbol::short("greetings"), Symbol::short("topic2")),
            Symbol::short("hello"),
        );
        env.events().publish(
            (Symbol::short("farewells"), Symbol::short("topic2")),
            Symbol::short("bye"),
        );
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use soroban_sdk::{testutils::Events, vec, BytesN, Env, IntoVal, Symbol};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_pub_event() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        assert_eq!(
            env.events().all(),
            vec![
                &env,
                // Expect 2 events.
                (
                    contract_id.clone(),
                    // Expect these event topics.
                    (Symbol::short("greetings"), Symbol::short("topic2")).into_val(&env),
                    // Expect this event body.
                    Symbol::short("hello").into_val(&env)
                ),
                (
                    contract_id,
                    // Expect these event topics.
                    (Symbol::short("farewells"), Symbol::short("topic2")).into_val(&env),
                    // Expect this event body.
                    Symbol::short("bye").into_val(&env)
                ),
            ],
        );
    }
}
