#![cfg(test)]
use soroban_sdk::{testutils::EnvTestConfig, vec, Env};

use crate::{Contract, ContractClient};

mod imported {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/test_alloc.wasm"
    );
}

macro_rules! tests {
    ($context:ident, $contract:expr) => {
        mod $context {
            use super::*;

            #[test]
            fn test() {
                let e = Env::new_with_config(EnvTestConfig {
                    // Disable test snapshots because the tests in this repo will run across
                    // multiple hosts, and this test uses a wasm file that won't build consistently
                    // across different hosts.
                    capture_snapshot_at_drop: false,
                });
                let contract_id = e.register($contract, ());
                let client = ContractClient::new(&e, &contract_id);

                let list = client.num_list(&50);
                assert_eq!(
                    list,
                    vec![
                        &e, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
                        39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
                    ]
                )
            }
        }
    };
}

tests!(native, Contract);
tests!(wasm, imported::WASM);
