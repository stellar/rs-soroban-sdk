use crate::{Val,val::ValType,host};

#[inline(always)]
pub fn get_current_ledger_num() -> u32 {
    unsafe { host::ledger::get_current_ledger_num().as_u32_unchecked() }
}

#[inline(always)]
pub fn pay(src: Val, dst: Val, asset: Val, amount: Val) -> Val {
    unsafe { host::ledger::pay(src, dst, asset, amount) }
}

#[inline(always)]
pub fn put_contract_data(k: Val, v: Val) {
    unsafe { host::ledger::put_contract_data(k, v) };
}

#[inline(always)]
pub fn has_contract_data(k: Val) -> bool {
    unsafe { <bool as ValType>::unchecked_from_val(host::ledger::has_contract_data(k)) }
}

#[inline(always)]
pub fn get_contract_data(k: Val) -> Val {
    unsafe { host::ledger::get_contract_data(k) }
}

#[inline(always)]
pub fn del_contract_data(k: Val) {
    unsafe { host::ledger::del_contract_data(k) };
}
