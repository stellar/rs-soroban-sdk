#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct Contract;

#[contractevent(data = "single-value")]
pub struct Transfer {
    #[topic]
    name: Symbol,
    #[data]
    value: Symbol,
}

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events().publish(
            (symbol_short!("greetings"), symbol_short!("topic2")),
            symbol_short!("hello"),
        );
        env.events().publish_event(&(
            (symbol_short!("hi_hi"), symbol_short!("topic2")),
            symbol_short!("boo"),
        ));
        env.events().publish_event(&Transfer {
            name: symbol_short!("ho_ho_ho"),
            value: symbol_short!("santa"),
        });
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use soroban_sdk::{symbol_short, testutils::Events, vec, Env, IntoVal};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_pub_event() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        assert_eq!(
            env.events().all(),
            vec![
                &env,
                // Expect 3 events.
                (
                    contract_id.clone(),
                    // Expect these event topics.
                    (symbol_short!("greetings"), symbol_short!("topic2")).into_val(&env),
                    // Expect this event body.
                    symbol_short!("hello").into_val(&env)
                ),
                (
                    contract_id.clone(),
                    // Expect these event topics.
                    (symbol_short!("farewells"), symbol_short!("topic2")).into_val(&env),
                    // Expect this event body.
                    symbol_short!("bye").into_val(&env)
                ),
                (
                    contract_id,
                    // Expect these event topics.
                    (symbol_short!("hi_hi"), symbol_short!("topic2")).into_val(&env),
                    // Expect this event body.
                    symbol_short!("boo").into_val(&env)
                ),
            ],
        );
    }
}
