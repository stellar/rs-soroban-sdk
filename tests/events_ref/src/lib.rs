#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, MuxedAddress};

#[contract]
pub struct Contract;

#[contractevent]
pub struct Transfer<'a> {
    #[topic]
    from: &'a Address,
    #[topic]
    to: &'a Address,
    amount: &'a i128,
    to_muxed_id: Option<&'a u64>,
}

#[contractimpl]
impl Contract {
    pub fn transfer(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from: &from,
            to: &to.address(),
            amount: &amount,
            to_muxed_id: to.id().as_ref(),
        }
        .publish(&env);
    }

    pub fn failed_transfer(env: Env, from: Address, to: Address, amount: i128) {
        Transfer {
            from: &from,
            to: &to,
            amount: &amount,
            to_muxed_id: None,
        }
        .publish(&env);
        panic!("fail");
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    extern crate std;

    use soroban_sdk::{
        testutils::{Address as _, Events, MuxedAddress as _},
        Address, Env, Event, MuxedAddress,
    };

    use crate::{Contract, ContractClient, Transfer};

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
            env.events().contract_events(),
            std::vec![Transfer {
                from: &from,
                to: &to.address(),
                amount: &amount,
                to_muxed_id: Some(&to.id().unwrap()),
            }
            .to_contract_event(&env, &contract_id),],
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
            env.events().contract_events(),
            std::vec![Transfer {
                from: &from,
                to: &to,
                amount: &amount,
                to_muxed_id: None,
            }
            .to_contract_event(&env, &contract_id),],
        );
    }

    #[test]
    fn test_no_events_recorded_for_failed_call() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);
        let from = Address::generate(&env);
        let to = Address::generate(&env);
        let _ = client.try_failed_transfer(&from, &to, &1);
        assert_eq!(env.events().contract_events(), std::vec![]);
    }
}
