#![no_std]
use sdk::{Object, Val};
use stellar_contract_sdk as sdk;

#[no_mangle]
pub fn pay(src: Object, dst: Object, asset: Object, amount: Val) -> Val {
    sdk::ledger::pay(src, dst, asset, amount)
}
