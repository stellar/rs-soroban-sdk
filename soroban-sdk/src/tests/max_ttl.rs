use crate::{self as soroban_sdk, testutils::Ledger as _};
use soroban_sdk::{contract, Env};

#[contract]
pub struct Contract;

#[test]
fn max() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);

    let mut ledger_info = e.ledger().get();
    ledger_info.sequence_number = 1;
    ledger_info.max_entry_ttl = 5;
    e.ledger().set(ledger_info);

    e.as_contract(&contract_id, || {
        assert_eq!(e.storage().max_ttl(), 5);
    });
}
