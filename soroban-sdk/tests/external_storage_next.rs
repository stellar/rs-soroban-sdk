#![cfg(feature = "next")]

use soroban_sdk::{Address, Env, Symbol};

#[allow(dead_code)]
fn external_storage_methods_type_check(
    env: Env,
    contract: Address,
    key: Symbol,
) -> (bool, Option<u64>, bool, Option<u64>, bool, Option<u64>) {
    let storage = env.storage();
    (
        storage.persistent().has_external(&contract, &key),
        storage.persistent().get_external(&contract, &key),
        storage.temporary().has_external(&contract, &key),
        storage.temporary().get_external(&contract, &key),
        storage.instance().has_external(&contract, &key),
        storage.instance().get_external(&contract, &key),
    )
}
