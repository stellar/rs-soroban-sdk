#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, MuxedAddress};
use soroban_token_sdk::events::Transfer;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn transfer(env: &Env, from: Address, to: MuxedAddress, amount: i128) {
        Transfer {
            from,
            to: to.address(),
            to_muxed_id: to.id(),
            amount,
        }
        .publish(env);
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{
        symbol_short,
        testutils::{Address as _, Events, MuxedAddress as _},
        vec, Env, IntoVal, Symbol, Val,
    };

    use crate::{Address, Contract, ContractClient, MuxedAddress};

    #[test]
    fn test_transfer() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let from = Address::generate(&e);
        let to = MuxedAddress::generate(&e);
        let amount: i128 = 123;

        client.transfer(&from, &to, &amount);

        assert_eq!(
            e.events().all(),
            vec![
                &e,
                (
                    contract_id.clone(),
                    (Symbol::new(&e, "transfer"), &from, to.address()).into_val(&e),
                    soroban_sdk::map![
                        &e,
                        (symbol_short!("amount"), amount.into_val(&e)),
                        (
                            Symbol::new(&e, "to_muxed_id"),
                            <_ as IntoVal<Env, Val>>::into_val(&to.id(), &e)
                        ),
                    ]
                    .to_val()
                ),
            ],
        );
    }
}
