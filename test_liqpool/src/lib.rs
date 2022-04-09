#![no_std]
use sdk::Val;
use stellar_contract_sdk as sdk;

#[no_mangle]
pub fn init(acc_id: Val, pool_asset: Val, asset_a: Val, asset_b: Val) -> Val {
    todo!()
}

#[no_mangle]
pub fn deposit(src_acc_id: Val, amount_a: Val, amount_b: Val) -> Val {
    todo!()
}

#[no_mangle]
pub fn withdraw(src_acc_id: Val, pool_amount: Val) -> Val {
    todo!()
}

#[no_mangle]
pub fn trade_fixed_in(src_acc_id: Val, asset_in: Val, amount_in: Val, asset_out: Val, min_amount_out: Val) -> Val {
    todo!()
}

#[no_mangle]
pub fn trade_fixed_out(src_acc_id: Val, asset_in: Val, max_amount_in: Val, asset_out: Val, amount_out: Val) -> Val {
    todo!()
}
