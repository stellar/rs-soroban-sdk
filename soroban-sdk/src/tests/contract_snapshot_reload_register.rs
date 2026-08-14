//! Tests what happens when a contract is deployed and used, the Env is
//! snapshotted, the snapshot is reloaded into a new Env, another contract is
//! registered, and then the original contract is called.

use crate::{self as soroban_sdk};
use soroban_sdk::{contract, contractimpl, testutils::Snapshot, xdr, Address, Env, TryFromVal};

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractA {
    pub fn store(env: Env, k: i32, v: i32) {
        env.storage().persistent().set(&k, &v)
    }
    pub fn get(env: Env, k: i32) -> i32 {
        env.storage().persistent().get(&k).unwrap()
    }
}

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn hello(_env: Env) -> i32 {
        42
    }
}

/// Deploys ContractA, calls it so that it has some persistent state, and
/// returns a snapshot of the Env along with the XDR form of ContractA's
/// address.
fn snapshot_with_contract_a() -> (Snapshot, xdr::ScAddress) {
    let e = Env::default();
    let contract_a_id = e.register(ContractA, ());
    let client = ContractAClient::new(&e, &contract_a_id);
    client.store(&2, &4);
    assert_eq!(client.get(&2), 4);
    (
        e.to_snapshot(),
        xdr::ScAddress::try_from(&contract_a_id).unwrap(),
    )
}

/// The address generators are part of the snapshot, so a contract registered
/// after a reload gets a fresh address rather than colliding with the address
/// the original contract already occupies.
#[test]
fn register_after_reload_uses_a_distinct_address() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    let contract_b_id = e.register(ContractB, ());

    assert_ne!(contract_b_id, contract_a_id);
}

/// The original contract's ledger state survives the reload, but the native
/// contract implementation behind it doesn't: it lives only in the Env it was
/// registered in, not in the snapshot. Calling it fails.
///
/// The host error differs before and after the native contract Wasm entry
/// change (rs-soroban-env#1720): before it is Error(Storage, MissingValue) —
/// "trying to get non-existing value for contract code", because the empty Wasm
/// entry native contracts pointed at was never written to the ledger; after it
/// is Error(WasmVm, InvalidInput) — "unsupported non-core wasm module", because
/// the contract's own stub Wasm entry is in the snapshot and the host gets as
/// far as trying to parse it.
#[test]
fn call_original_contract_after_reload_fails() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();

    let res = ContractAClient::new(&e, &contract_a_id).try_get(&2);
    // Err(Ok(..)) is an error that couldn't be converted into ContractA's own
    // error type, i.e. a host error.
    assert_eq!(
        res,
        Err(Ok(soroban_sdk::Error::from_type_and_code(
            xdr::ScErrorType::Context,
            xdr::ScErrorCode::InvalidAction,
        )))
    );
}

/// Registering another contract after the reload doesn't change that: the new
/// contract is callable, the original one still isn't.
///
/// The host error again differs across rs-soroban-env#1720: before it is
/// Error(WasmVm, InvalidAction), because registering the new contract wrote the
/// shared empty Wasm entry that the original contract's instance also points
/// at, so the host runs it and finds no function; after it is
/// Error(WasmVm, InvalidInput), the same error as without registering anything,
/// because each native contract now has its own stub Wasm entry.
#[test]
fn call_original_contract_after_reload_and_register_fails() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    let contract_b_id = e.register(ContractB, ());

    assert_eq!(ContractBClient::new(&e, &contract_b_id).hello(), 42);

    let res = ContractAClient::new(&e, &contract_a_id).try_get(&2);
    assert_eq!(
        res,
        Err(Ok(soroban_sdk::Error::from_type_and_code(
            xdr::ScErrorType::Context,
            xdr::ScErrorCode::InvalidAction,
        )))
    );
}

/// Re-registering the original contract at its own address after the reload is
/// the working path: it becomes callable again, with its persistent state from
/// before the snapshot intact, and a contract registered alongside it works
/// too.
#[test]
fn re_registering_original_contract_after_reload_restores_it() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    e.register_at(&contract_a_id, ContractA, ());
    let contract_b_id = e.register(ContractB, ());

    assert_eq!(ContractAClient::new(&e, &contract_a_id).get(&2), 4);
    assert_eq!(ContractBClient::new(&e, &contract_b_id).hello(), 42);
}
