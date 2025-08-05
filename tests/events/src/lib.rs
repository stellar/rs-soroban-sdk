#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, MuxedAddress};

#[contract]
pub struct Contract;

#[contractevent]
pub struct Transfer {
    #[topic]
    from: Address,
    #[topic]
    to: Address,
    amount: i128,
    to_muxed_id: Option<u64>,
}

#[contractimpl]
impl Contract {
    pub fn transfer(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from: from.clone(),
            to: to.address(),
            amount,
            to_muxed_id: to.id(),
        }
        .publish(&env);
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use soroban_sdk::{
        map, symbol_short,
        testutils::{Address as _, Events, MuxedAddress as _},
        vec, Address, Env, IntoVal, MuxedAddress, Symbol, Val,
    };

    use crate::{Contract, ContractClient};

    #[test]
    fn test_event() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let from = Address::generate(&env);
        let to = MuxedAddress::generate(&env);
        let amount = 1i128;

        client.transfer(&from, &to, &amount);

        assert_eq!(
            env.events().all(),
            vec![
                &env,
                (
                    contract_id.clone(),
                    // Expect these event topics.
                    (Symbol::new(&env, "transfer"), &from, to.address()).into_val(&env),
                    // Expect this event body.
                    map![
                        &env,
                        (
                            symbol_short!("amount"),
                            <_ as IntoVal<Env, Val>>::into_val(&1i128, &env)
                        ),
                        (
                            Symbol::new(&env, "to_muxed_id"),
                            <_ as IntoVal<Env, Val>>::into_val(&to.id().unwrap(), &env)
                        ),
                    ]
                    .to_val()
                ),
            ],
        );
    }

    #[test]
    fn test_event_with_option_none() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let amount = 1i128;

        client.transfer(&from, &to, &amount);

        assert_eq!(
            env.events().all(),
            vec![
                &env,
                (
                    contract_id.clone(),
                    // Expect these event topics.
                    (Symbol::new(&env, "transfer"), &from, &to).into_val(&env),
                    // Expect this event body.
                    map![
                        &env,
                        (
                            symbol_short!("amount"),
                            <_ as IntoVal<Env, Val>>::into_val(&1i128, &env)
                        ),
                        (Symbol::new(&env, "to_muxed_id"), ().into_val(&env),),
                    ]
                    .to_val()
                ),
            ],
        );
    }
}
