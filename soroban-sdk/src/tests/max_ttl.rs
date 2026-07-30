use crate::{self as soroban_sdk, testutils::Ledger as _};
use soroban_sdk::{contract, Env};

#[contract]
pub struct Contract;

#[test]
fn max() {
    let e = Env::default();
    let contract_id = e.register(Contract, ());

    e.ledger().set_sequence_number(1);
    e.ledger().set_max_entry_ttl(5);

    e.as_contract(&contract_id, || {
        assert_eq!(e.storage().max_ttl(), 5);
    });
}
