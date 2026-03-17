#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env};

pub trait Chained {
    type Base;
    type Foo;
    type End;

    fn set_val(env: Env, input: Self::End);
    fn get_val(env: Env) -> Self::End;
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Chained for Contract {
    type Base = u64;
    type Foo = Self::Base;
    type End = Self::Foo;

    fn set_val(env: Env, input: Self::End) {
        env.storage()
            .instance()
            .set(&symbol_short!("chained"), &input);
    }

    fn get_val(env: Env) -> Self::End {
        env.storage()
            .instance()
            .get(&symbol_short!("chained"))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Chained, Contract, ContractClient};

    #[test]
    fn test_chained() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.set_val(&42u64);
        assert_eq!(client.get_val(), 42u64);

        client.set_val(&(41 as <Contract as Chained>::End));
        assert_eq!(client.get_val(), (41 as <Contract as Chained>::End));
    }
}
