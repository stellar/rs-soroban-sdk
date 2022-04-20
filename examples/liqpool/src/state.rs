use stellar_contract_sdk as sdk;
use stellar_contract_sdk::OrAbort;

use crate::datakeys::*;

pub struct State {
    pub asset_p_circulating: i64,
}

impl State {
    pub fn load() -> Self {
        Self {
            asset_p_circulating: sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
                .try_into()
                .or_abort(),
        }
    }
    pub fn save(&self) {
        sdk::ledger::put_contract_data(
            DATA_KEY_ASSET_POOL_CIRCULATING,
            self.asset_p_circulating.into(),
        );
    }
}
