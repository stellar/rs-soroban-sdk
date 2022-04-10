#![no_std]
use sdk::{OrAbort, Symbol, Val};
use stellar_contract_sdk as sdk;
use stellar_contract_sdk_macros as sdkmacros;

const DATA_POOL_ASSET_CIRCULATING: Val = Val::from_symbol(Symbol::from_str("hello"));

#[sdkmacros::contractfn]
pub fn init(acc_id: Val, pool_asset: Val, asset_a: Val, asset_b: Val) -> Val {
    sdk::ledger::put_contract_data(DATA_POOL_ASSET_CIRCULATING, Val::from_u63(0));
    todo!()
}

#[sdkmacros::contractfn]
pub fn deposit(src_acc_id: Val, amount_a: Val, amount_b: Val) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn withdraw(src_acc_id: Val, pool_amount: Val) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn trade_fixed_in(
    src_acc_id: Val,
    asset_in: Val,
    amount_in: Val,
    asset_out: Val,
    min_amount_out: Val,
) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn trade_fixed_out(
    src_acc_id: Val,
    asset_in: Val,
    max_amount_in: Val,
    asset_out: Val,
    amount_out: Val,
) -> Val {
    todo!()
}
