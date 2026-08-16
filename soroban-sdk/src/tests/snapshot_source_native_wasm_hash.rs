use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractimpl,
    testutils::{HostError, SnapshotSource, SnapshotSourceInput},
    xdr, BytesN, Env,
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

fn contract_code_keys(keys: &Rc<RefCell<Vec<xdr::LedgerKey>>>) -> Vec<xdr::LedgerKey> {
    keys.borrow()
        .iter()
        .filter(|k| matches!(k, xdr::LedgerKey::ContractCode(_)))
        .cloned()
        .collect()
}

/// Native contracts have Wasm hashes that exist on no real network, so their
/// ContractCode entries must never be looked up against the snapshot source,
/// during registration or afterwards. The host looks the code entries of native
/// contracts up in the storage map only, so no filtering is needed in the SDK.
///
/// See issue #1635 "Empty WASM hash leaks to SnapshotSource".
#[test]
fn native_contracts_never_request_contract_code() {
    let (env, keys) = recording_env();

    // Registering, at a generated address and at a given address.
    let contract_id = env.register(Contract, ());
    let _ = env.register_at(&contract_id, Contract, ());

    // Uploading, at a generated hash and at a given hash.
    let wasm_hash = env.upload(Contract);
    let _ = env.upload_at([9u8; 32], Contract);

    // Calling.
    ContractClient::new(&env, &contract_id).hello();

    // Deploying an instance from an uploaded native contract, and calling it.
    let deployed = env.as_contract(&contract_id, || {
        env.deployer()
            .with_address(contract_id.clone(), BytesN::from_array(&env, &[0u8; 32]))
            .deploy_v2(wasm_hash, ())
    });
    ContractClient::new(&env, &deployed).hello();

    assert_eq!(
        contract_code_keys(&keys),
        Vec::new(),
        "no ContractCode entry should be requested from the snapshot source, all recorded keys: {:?}",
        keys.borrow()
    );

    // Sanity check: other lookups (such as the contract instance ContractData
    // entries) do still reach the snapshot source, so the assertion above is
    // not passing simply because nothing is recorded.
    assert!(
        keys.borrow()
            .iter()
            .any(|k| matches!(k, xdr::LedgerKey::ContractData(_))),
        "expected contract instance ContractData lookups to reach the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );
}

/// Wasm contracts are unaffected: their ContractCode entry is still looked up
/// against the snapshot source.
#[test]
fn wasm_contracts_still_request_contract_code() {
    const WASM: &[u8] = include_bytes!("../../doctest_fixtures/contract.wasm");

    let (env, keys) = recording_env();

    let _ = env.register(WASM, ());

    assert!(
        !contract_code_keys(&keys).is_empty(),
        "the Wasm contract's ContractCode lookup should reach the snapshot source, recorded keys: {:?}",
        keys.borrow()
    );
}
