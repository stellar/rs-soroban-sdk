#![no_std]
use soroban_sdk::{
    contract, contractevent, contractimpl, contracttype, Address, Env, MuxedAddress, Vec,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Deposit(Address, MuxedAddress, i128),
    Withdraw(Address, MuxedAddress, i128),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Key {
    pub from: Address,
    pub to: MuxedAddress,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    pub from: Address,
    pub to: MuxedAddress,
    pub amount: i128,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deposit {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
    pub to_muxed_id: Option<u64>,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Withdraw {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
    pub to_muxed_id: Option<u64>,
}

fn key(from: &Address, to: &MuxedAddress) -> Key {
    Key {
        from: from.clone(),
        to: to.clone(),
    }
}

#[contractimpl]
impl Contract {
    pub fn deposit(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        assert!(amount > 0);
        Deposit {
            from: from.clone(),
            to: to.address(),
            amount,
            to_muxed_id: to.id(),
        }
        .publish(&env);
        let k = key(&from, &to);
        let storage = env.storage().instance();
        let current = storage.get::<_, Record>(&k).map_or(0, |r| r.amount);
        let amount = current.checked_add(amount).unwrap();
        storage.set(&k, &Record { from, to, amount });
    }

    pub fn withdraw(env: Env, from: Address, to: MuxedAddress, amount: i128) {
        assert!(amount > 0);
        Withdraw {
            from: from.clone(),
            to: to.address(),
            amount,
            to_muxed_id: to.id(),
        }
        .publish(&env);
        let k = key(&from, &to);
        let storage = env.storage().instance();
        let current = storage.get::<_, Record>(&k).map_or(0, |r| r.amount);
        let amount = current.checked_sub(amount).unwrap();
        assert!(amount >= 0);
        if amount == 0 {
            storage.remove(&k);
        } else {
            storage.set(&k, &Record { from, to, amount });
        }
    }

    pub fn chain(env: Env, actions: Vec<Action>) {
        for action in actions.iter() {
            match action {
                Action::Deposit(from, to, amount) => Self::deposit(env.clone(), from, to, amount),
                Action::Withdraw(from, to, amount) => Self::withdraw(env.clone(), from, to, amount),
            }
        }
    }

    pub fn get(env: Env, from: Address, to: MuxedAddress) -> Option<Record> {
        env.storage().instance().get(&key(&from, &to))
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{
        testutils::{Address as _, Events, MuxedAddress as _},
        vec, Address, Env, Event, MuxedAddress,
    };

    use crate::{Action, Contract, ContractClient, Deposit, Withdraw};

    #[test]
    fn test_chain_to_zero() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let from = Address::generate(&env);
        let to = MuxedAddress::new(Address::generate(&env), 123);

        client.chain(&vec![
            &env,
            Action::Deposit(from.clone(), to.clone(), 100),
            Action::Deposit(from.clone(), to.clone(), 50),
            Action::Withdraw(from.clone(), to.clone(), 120),
            Action::Withdraw(from.clone(), to.clone(), 30),
        ]);

        assert_eq!(
            env.events().all(),
            [
                Deposit {
                    from: from.clone(),
                    to: to.address(),
                    amount: 100,
                    to_muxed_id: Some(123),
                }
                .to_xdr(&env, &contract_id),
                Deposit {
                    from: from.clone(),
                    to: to.address(),
                    amount: 50,
                    to_muxed_id: Some(123),
                }
                .to_xdr(&env, &contract_id),
                Withdraw {
                    from: from.clone(),
                    to: to.address(),
                    amount: 120,
                    to_muxed_id: Some(123),
                }
                .to_xdr(&env, &contract_id),
                Withdraw {
                    from: from.clone(),
                    to: to.address(),
                    amount: 30,
                    to_muxed_id: Some(123),
                }
                .to_xdr(&env, &contract_id),
            ],
        );

        assert_eq!(client.get(&from, &to), None);
    }
}
