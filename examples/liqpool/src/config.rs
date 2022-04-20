use stellar_contract_sdk as sdk;
use stellar_contract_sdk::{Object, OrAbort};

use crate::datakeys::*;

#[derive(Clone, Copy)]
pub struct Config {
    pub acc: Object,
    pub asset_p: Object,
    pub asset_a: Object,
    pub asset_b: Object,
}

impl Config {
    pub fn load() -> Self {
        Self {
            acc: sdk::ledger::get_contract_data(DATA_KEY_ACCOUNT)
                .try_into()
                .or_abort(),
            asset_p: sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL)
                .try_into()
                .or_abort(),
            asset_a: sdk::ledger::get_contract_data(DATA_KEY_ASSET_A)
                .try_into()
                .or_abort(),
            asset_b: sdk::ledger::get_contract_data(DATA_KEY_ASSET_B)
                .try_into()
                .or_abort(),
        }
    }
    pub fn save(&self) {
        sdk::ledger::put_contract_data(DATA_KEY_ACCOUNT, self.acc.into());
        sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL, self.asset_p.into());
        sdk::ledger::put_contract_data(DATA_KEY_ASSET_A, self.asset_a.into());
        sdk::ledger::put_contract_data(DATA_KEY_ASSET_B, self.asset_b.into());
    }
}
