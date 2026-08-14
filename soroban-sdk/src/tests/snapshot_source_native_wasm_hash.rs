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

fn recording_env() -> (Env, Rc<RefCell<Vec<xdr::LedgerKey>>>) {
    let keys = Rc::new(RefCell::new(Vec::new()));
    let input = SnapshotSourceInput {
        source: Rc::new(RecordingSnapshotSource { keys: keys.clone() }),
        ledger_info: None,
        snapshot: None,
    };
    (Env::from_ledger_snapshot(input), keys)
}

fn contract_code_requested(keys: &Rc<RefCell<Vec<xdr::LedgerKey>>>) -> bool {
    keys.borrow()
        .iter()
        .any(|k| matches!(k, xdr::LedgerKey::ContractCode(_)))
}

/// Native contracts each have their own Wasm hash, and none of those hashes
/// exist on any real network, so none of them should ever be looked up against
/// the snapshot source.
///
/// This also guards the SDK's copy of the host's Wasm hash derivation: if the
/// host changes how it derives the hash for a contract registered without an
/// explicit hash, the SDK stops recognising it and this test fails rather than
/// silently losing the filtering.
#[test]
fn registering_does_not_request_the_native_wasm_hash() {
    let (env, keys) = recording_env();

    let _ = env.register(Contract, ());

    assert!(
        !contract_code_requested(&keys),
        "no ContractCode entry should be requested from the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );

    // Sanity check: other lookups (such as the contract instance ContractData
    // entry) are still allowed to reach the snapshot source.
    let requested_contract_data = keys
        .borrow()
        .iter()
        .any(|k| matches!(k, xdr::LedgerKey::ContractData(_)));
    assert!(
        requested_contract_data,
        "expected the contract instance ContractData lookup to reach the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );
}

#[test]
fn uploading_does_not_request_the_native_wasm_hash() {
    let (env, keys) = recording_env();

    let _ = env.upload(Contract);

    assert!(
        !contract_code_requested(&keys),
        "no ContractCode entry should be requested from the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );
}

#[test]
fn uploading_at_a_hash_does_not_request_the_native_wasm_hash() {
    let (env, keys) = recording_env();

    let _ = env.upload_at([9u8; 32], Contract);

    assert!(
        !contract_code_requested(&keys),
        "no ContractCode entry should be requested from the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );
}

/// The filter is limited to native contracts: a Wasm hash the SDK has not
/// registered natively still reaches the snapshot source.
#[test]
fn other_wasm_hashes_still_reach_the_snapshot_source() {
    const WASM: &[u8] = include_bytes!("../../doctest_fixtures/contract.wasm");

    let (env, keys) = recording_env();

    let _ = env.register(WASM, ());

    assert!(
        contract_code_requested(&keys),
        "the Wasm contract's ContractCode lookup should reach the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );
}
