use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractimpl,
    testutils::{HostError, SnapshotSource, SnapshotSourceInput},
    xdr, Env,
};
use std::{cell::RefCell, rc::Rc};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(_env: Env) {}
}

/// The SHA256 of empty/zero bytes, used by the host as the synthetic WASM hash
/// for native (non-WASM) test contracts.
const EMPTY_WASM_HASH: [u8; 32] = [
    0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24,
    0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
];

/// A snapshot source that records every key it is asked for and always returns
/// `Ok(None)`.
struct RecordingSnapshotSource {
    keys: Rc<RefCell<Vec<xdr::LedgerKey>>>,
}

impl SnapshotSource for RecordingSnapshotSource {
    fn get(
        &self,
        key: &Rc<xdr::LedgerKey>,
    ) -> Result<Option<(Rc<xdr::LedgerEntry>, Option<u32>)>, HostError> {
        self.keys.borrow_mut().push(key.as_ref().clone());
        Ok(None)
    }
}

#[test]
fn test_empty_wasm_hash_not_requested_from_snapshot_source() {
    let keys = Rc::new(RefCell::new(Vec::new()));
    let input = SnapshotSourceInput {
        source: Rc::new(RecordingSnapshotSource { keys: keys.clone() }),
        ledger_info: None,
        snapshot: None,
    };

    let env = Env::from_ledger_snapshot(input);

    // Registering a native test contract should not cause a lookup for the
    // empty WASM hash ContractCode entry against the snapshot source.
    let _ = env.register(Contract, ());

    let recorded = keys.borrow();

    // The empty WASM hash ContractCode entry must never be requested.
    let requested_empty_wasm = recorded.iter().any(|k| {
        matches!(
            k,
            xdr::LedgerKey::ContractCode(xdr::LedgerKeyContractCode { hash, .. })
                if hash.0 == EMPTY_WASM_HASH
        )
    });
    assert!(
        !requested_empty_wasm,
        "empty WASM hash ContractCode entry should not be requested from the snapshot source, recorded keys: {recorded:?}"
    );

    // Sanity check: other lookups (such as the contract instance ContractData
    // entry) are still allowed to reach the snapshot source.
    let requested_contract_data = recorded
        .iter()
        .any(|k| matches!(k, xdr::LedgerKey::ContractData(_)));
    assert!(
        requested_contract_data,
        "expected the contract instance ContractData lookup to reach the snapshot source, recorded keys: {recorded:?}"
    );
}
