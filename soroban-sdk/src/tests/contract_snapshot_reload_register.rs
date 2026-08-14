//! Tests what happens when a contract is deployed and used, the ledger is
//! snapshotted, the snapshot is reloaded into a new Env, another contract is
//! registered, and then the original contract is called.
//!
//! The behaviour these tests capture is the same before and after the native
//! contract Wasm entry change (rs-soroban-env#1720), except for the host error
//! noted on `call_original_contract_after_reload_without_registering_fails`.

use crate::{self as soroban_sdk};
use soroban_ledger_snapshot::LedgerSnapshot;
use soroban_sdk::{contract, contractimpl, xdr, Address, Env, TryFromVal};

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

/// A contract that has none of ContractA's functions.
#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn hello(_env: Env) -> i32 {
        42
    }
}

/// A contract that reads the same storage keys ContractA writes.
#[contract]
pub struct ContractC;

#[contractimpl]
impl ContractC {
    pub fn get(env: Env, k: i32) -> i32 {
        env.storage().persistent().get(&k).unwrap()
    }
}

/// Deploys ContractA, calls it so that it has some persistent state, and
/// returns a snapshot of the ledger along with the XDR form of ContractA's
/// address.
fn snapshot_with_contract_a() -> (LedgerSnapshot, xdr::ScAddress) {
    let e = Env::default();
    let contract_a_id = e.register(ContractA, ());
    let client = ContractAClient::new(&e, &contract_a_id);
    client.store(&2, &4);
    assert_eq!(client.get(&2), 4);
    (
        e.to_ledger_snapshot(),
        xdr::ScAddress::try_from(&contract_a_id).unwrap(),
    )
}

/// Registering a contract in an Env restored from a snapshot hands out the same
/// address that the snapshot's first contract already occupies, because the
/// address generation seed isn't carried in the snapshot.
#[test]
fn register_after_reload_collides_with_original_contract_address() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_ledger_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    let contract_b_id = e.register(ContractB, ());

    assert_eq!(contract_b_id, contract_a_id);
}

/// Calling the original contract after a reload, without registering anything,
/// fails: the native contract implementation isn't carried in the snapshot, so
/// the contract's executable has no code behind it in the new Env.
///
/// This is the one case where the host error differs before and after the
/// native-contract-Wasm-entry change (rs-soroban-env#1720). Before, native
/// contracts pointed at an empty Wasm entry that was never written to the
/// ledger, so the host failed the code lookup with Error(Storage, MissingValue)
/// — "trying to get non-existing value for contract code". After, every native
/// contract has its own stub Wasm entry which is in the snapshot, so the lookup
/// succeeds and the host fails parsing it with Error(WasmVm, InvalidInput) —
/// "unsupported non-core wasm module". Both surface through the client as
/// Error(Context, InvalidAction).
#[test]
fn call_original_contract_after_reload_without_registering_fails() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_ledger_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();

    let res = ContractAClient::new(&e, &contract_a_id).try_get(&2);
    assert_eq!(
        res,
        Err(Ok(soroban_sdk::Error::from_type_and_code(
            xdr::ScErrorType::Context,
            xdr::ScErrorCode::InvalidAction,
        )))
    );
}

/// Because of the address collision above, calling the original contract after
/// the reload dispatches to the newly registered contract, and fails with a
/// missing-function error instead of running ContractA's function.
#[test]
fn call_original_contract_after_reload_and_register_fails() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_ledger_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    let _contract_b_id = e.register(ContractB, ());

    let res = ContractAClient::new(&e, &contract_a_id).try_get(&2);
    // Err(Ok(..)) is an error that couldn't be converted into ContractA's own
    // error type, i.e. a host error. The host error is
    // Error(Context, MissingValue) — "calling unknown contract function: get" —
    // which the client surfaces as Error(Context, InvalidAction).
    assert_eq!(
        res,
        Err(Ok(soroban_sdk::Error::from_type_and_code(
            xdr::ScErrorType::Context,
            xdr::ScErrorCode::InvalidAction,
        )))
    );
}

/// The collision doesn't only break calls to the original contract: the newly
/// registered contract takes over the original contract's persistent storage,
/// because storage is keyed by contract address.
#[test]
fn contract_registered_after_reload_inherits_original_contract_storage() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_ledger_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    let contract_c_id = e.register(ContractC, ());
    assert_eq!(contract_c_id, contract_a_id);

    // ContractC never stored anything, but it reads back the value that
    // ContractA stored before the snapshot was taken.
    assert_eq!(ContractCClient::new(&e, &contract_c_id).get(&2), 4);
}

/// Registering the new contract at an address that doesn't collide, and
/// re-registering the original contract at its own address, is the working
/// path: both contracts are callable.
#[test]
fn no_collision_when_new_contract_registered_at_distinct_address() {
    let (snapshot, contract_a_id_xdr) = snapshot_with_contract_a();

    let e = Env::from_ledger_snapshot(snapshot);
    let contract_a_id = Address::try_from_val(&e, &contract_a_id_xdr).unwrap();
    e.register_at(&contract_a_id, ContractA, ());
    let contract_b_id = Address::from_str(
        &e,
        "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE",
    );
    e.register_at(&contract_b_id, ContractB, ());

    assert_eq!(ContractAClient::new(&e, &contract_a_id).get(&2), 4);
    assert_eq!(ContractBClient::new(&e, &contract_b_id).hello(), 42);
}
