// This contract is not tested and probably has various bugs and vulnerabilities.

#![no_std]
use sdk::{Symbol, Val, Vec};
use stellar_contract_sdk as sdk;

const TOTAL_SUPPLY_KEY: Val = Val::from_symbol(Symbol::from_str("SUPPLY"));

#[no_mangle]
pub fn initialize(src: Val, amount: Val) -> Val {
    sdk::ledger::put_contract_data(TOTAL_SUPPLY_KEY, amount);
    sdk::ledger::put_contract_data(src, amount);
    return true.into();
}

#[no_mangle]
pub fn total_supply() -> Val {
    return sdk::ledger::get_contract_data(TOTAL_SUPPLY_KEY);
}

#[no_mangle]
pub fn balance(acc: Val) -> Val {
    return sdk::ledger::get_contract_data(acc);
}

#[no_mangle]
pub fn allowance(sender: Val, initiator: Val) -> Val {
    return sdk::ledger::get_contract_data(approver_key(sender.as_symbol(), initiator.as_symbol()));
}

fn approver_key(sender: Symbol, initiator: Symbol) -> Val {
    Vec::<Symbol>::new().push(sender).push(initiator).into()
}

#[no_mangle]
pub fn approve(sender: Val, initiator: Val, amount: Val) -> Val {
    sdk::ledger::put_contract_data(
        approver_key(sender.as_symbol(), initiator.as_symbol()),
        amount,
    );
    return true.into();
}

fn do_transfer(sender: Val, receiver: Val, amount: u32) -> bool {
    let sender_balance: u32 = sdk::ledger::get_contract_data(sender).as_u32();
    if sender_balance < amount {
        return false; // sender_balance will be negative
    }

    if sender == receiver {
        return true; // no updates required to send to self
    }

    if sdk::ledger::has_contract_data(receiver) {
        let receiver_balance: u32 = sdk::ledger::get_contract_data(receiver).as_u32();
        if u32::MAX - receiver_balance < amount {
            return false; // receiver_balance will exceed u32::MAX
        }
        sdk::ledger::put_contract_data(receiver, (receiver_balance + amount).into());
    } else {
        sdk::ledger::put_contract_data(receiver, amount.into());
    }
    sdk::ledger::put_contract_data(sender, (sender_balance - amount).into());

    return true;
}

#[no_mangle]
pub fn transfer(sender: Val, receiver: Val, _amount: Val) -> Val {
    return do_transfer(sender, receiver, _amount.as_u32()).into();
}

#[no_mangle]
pub fn transfer_from(sender: Val, initiator: Val, receiver: Val, _amount: Val) -> Val {
    let amount: u32 = _amount.as_u32();
    let key = approver_key(sender.as_symbol(), initiator.as_symbol());

    let sender_approved: u32 = sdk::ledger::get_contract_data(key).as_u32();
    if sender_approved < amount {
        return false.into();
    }

    if do_transfer(sender, receiver, amount) {
        sdk::ledger::put_contract_data(key, (sender_approved - amount).into());
        return true.into();
    }
    return false.into();
}

// Entry point for running the contract against stellar-core
// This is _not_ part of the core contract
#[no_mangle]
pub fn run() -> Val {
    let acc_a = Val::from_symbol(Symbol::from_str("A"));
    let acc_b = Val::from_symbol(Symbol::from_str("B"));

    initialize(acc_a, 1000u32.into());
    sdk::log_value(total_supply());
    sdk::log_value(balance(acc_a));

    transfer(acc_a, acc_b, 100u32.into());
    sdk::log_value(balance(acc_a));
    sdk::log_value(balance(acc_b));

    approve(acc_a, acc_b, 500u32.into());
    sdk::log_value(allowance(acc_a, acc_b));

    transfer_from(acc_a, acc_b, acc_b, 300u32.into());
    sdk::log_value(balance(acc_a));
    sdk::log_value(balance(acc_b));
    return true.into();
}
