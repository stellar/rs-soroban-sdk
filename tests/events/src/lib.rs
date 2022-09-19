#![no_std]
use soroban_sdk::{contractimpl, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events()
            .publish((symbol!("greetings"), symbol!("topic2")), symbol!("hello"));
        env.events()
            .publish((symbol!("farewells"), symbol!("topic2")), symbol!("bye"));
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use soroban_sdk::{symbol, testutils::Events, xdr::ScVal, BytesN, Env, IntoVal};

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
            std::vec![
                // Expect 2 events.
                (
                    &[0; 32],
                    // Expect these event topics.
                    std::vec![
                        ScVal::Symbol("greetings".try_into().unwrap()),
                        ScVal::Symbol("topic2".try_into().unwrap()),
                    ],
                    // Expect this event body.
                    ScVal::Symbol("hello".try_into().unwrap()),
                ),
                (
                    &[0; 32],
                    // Expect these event topics.
                    std::vec![
                        ScVal::Symbol("farewells".try_into().unwrap()),
                        ScVal::Symbol("topic2".try_into().unwrap()),
                    ],
                    // Expect this event body.
                    ScVal::Symbol("byte".try_into().unwrap()),
                ),
            ],
        );
    }
}
