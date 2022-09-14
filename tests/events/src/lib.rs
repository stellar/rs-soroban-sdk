#![no_std]
use soroban_sdk::{contractimpl, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events()
            .publish((symbol!("topic1"), symbol!("topic2")), symbol!("hello"));
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use soroban_sdk::{symbol, vec, BytesN, Env, IntoVal};

    use crate::{Contract, ContractClient};

    #[test]
    fn test_pub_event() {
        let env = Env::default();
        let contract_id = BytesN::from_array(&env, &[0; 32]);
        env.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&env, &contract_id);

        client.hello();

        assert_eq!(
            env.events().get(),
            vec![
                &env,
                // Expect 1 event.
                (
                    contract_id,
                    // Expect these event topics.
                    vec![
                        &env,
                        symbol!("topic1").into_val(&env),
                        symbol!("topic2").into_val(&env)
                    ],
                    // Expect this event body.
                    symbol!("hello").into_val(&env)
                )
            ],
        );
    }
}
