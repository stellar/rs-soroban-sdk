#![no_std]
use sdk::{OrAbort, Symbol, Val};
use stellar_contract_sdk as sdk;
use stellar_contract_sdk_macros as sdkmacros;

const DATA_KEY_ACC_ID: Val = Val::from_symbol(Symbol::from_str("accid"));
const DATA_KEY_ASSET_POOL: Val = Val::from_symbol(Symbol::from_str("assetpool"));
const DATA_KEY_ASSET_A: Val = Val::from_symbol(Symbol::from_str("asseta"));
const DATA_KEY_ASSET_B: Val = Val::from_symbol(Symbol::from_str("assetb"));

const DATA_KEY_ASSET_POOL_CIRCULATING: Val =
    Val::from_symbol(Symbol::from_str("assetpoolcirculating"));

#[sdkmacros::contractfn]
pub fn init(acc_id: Val, pool_asset: Val, asset_a: Val, asset_b: Val) -> Val {
    sdk::ledger::put_contract_data(DATA_KEY_ACC_ID, acc_id);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL, pool_asset);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_A, asset_a);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_B, asset_b);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING, Val::from_u63(0));
    Val::from_bool(true)
}

#[sdkmacros::contractfn]
pub fn deposit(_src_acc_id: Val, _amount_a: Val, _amount_b: Val) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn withdraw(_src_acc_id: Val, _pool_amount: Val) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn trade_fixed_in(
    _src_acc_id: Val,
    _asset_in: Val,
    _amount_in: Val,
    _asset_out: Val,
    _min_amount_out: Val,
) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn trade_fixed_out(
    _src_acc_id: Val,
    _asset_in: Val,
    _max_amount_in: Val,
    _asset_out: Val,
    _amount_out: Val,
) -> Val {
    todo!()
}
