use stellar_contract_sdk as sdk;
use stellar_contract_sdk::{Object, OrAbort};

use crate::config::Config;

#[derive(Clone, Copy)]
pub struct Reserves {
    c: Config,
    pub a: i64,
    pub b: i64,
}

impl Reserves {
    pub fn load(c: &Config) -> Self {
        Self {
            c: *c,
            a: sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(c.acc, c.asset_a))
                .try_into()
                .or_abort(),
            b: sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(c.acc, c.asset_b))
                .try_into()
                .or_abort(),
        }
    }
    pub fn for_asset(&self, asset: Object) -> i64 {
        if asset == self.c.asset_a {
            self.a
        } else if asset == self.c.asset_b {
            self.b
        } else {
            panic!("unrecognized asset")
        }
    }
}
