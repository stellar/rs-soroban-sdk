use crate::{self as soroban_sdk};
use sha2::{Digest, Sha256};
use soroban_sdk::{
    address::Executable, contract, env::EnvTestConfig, testutils::Address as _, Address, Bytes,
    BytesN, Env, String, TryIntoVal,
};

#[contract]
struct TestContract;

#[test]
fn test_account_address_str_conversions() {
    let env = Env::default();

    let strkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";

    let address = Address::from_str(&env, &strkey);
    assert_eq!(
        address.to_string().to_string(),
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn test_account_address_conversions() {
    let env = Env::default();

    let strkey: String = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
        .try_into_val(&env)
        .unwrap();

    let address = Address::from_string(&strkey);
    assert_eq!(
        address.to_string().to_string(),
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn test_account_address_conversions_from_bytes() {
    let env = Env::default();

    let strkey: Bytes = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
        .as_bytes()
        .try_into_val(&env)
        .unwrap();

    let address = Address::from_string_bytes(&strkey);
    assert_eq!(
        address.to_string().to_string(),
        "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ"
    );
}

#[test]
fn test_contract_address_conversions() {
    let env = Env::default();

    let strkey: String = "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA"
        .try_into_val(&env)
        .unwrap();

    let address = Address::from_string(&strkey);
    assert_eq!(address.to_string(), strkey);
}

#[test]
fn test_get_non_existent_address_executable() {
    let env = Env::default();

    let account_strkey = "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ";
    let account_address = Address::from_str(&env, &account_strkey);
    let account_executable = account_address.executable();
    assert!(account_executable.is_none());
    assert!(!account_address.exists());

    let contract_strkey = "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA";
    let contract_address = Address::from_str(&env, &contract_strkey);
    let contract_executable = contract_address.executable();
    assert!(contract_executable.is_none());
    assert!(!contract_address.exists());
}

#[test]
fn test_get_existing_contract_address_executable_wasm() {
    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../target/wasm32v1-none/release/test_udt.wasm");

    let env = Env::new_with_config(EnvTestConfig {
        // Disable test snapshots because the tests in this repo will run across
        // multiple hosts, and this test uses a wasm file that won't build consistently
        // across different hosts.
        capture_snapshot_at_drop: false,
    });

    let contract_address = env.register(EXAMPLE_WASM, ());
    let contract_executable = contract_address.executable();
    let sha256: [u8; 32] = Sha256::digest(EXAMPLE_WASM).into();
    let wasm_hash = BytesN::from_array(&env, &sha256);
    assert_eq!(contract_executable, Some(Executable::Wasm(wasm_hash)));
    assert!(contract_address.exists());
}

#[test]
fn test_get_existing_contract_address_executable_native() {
    let env = Env::default();

    let native_contract_address = env.register(TestContract, ());
    let native_contract_executable = native_contract_address.executable();
    let empty_sha256: [u8; 32] = Sha256::digest([]).into();
    let empty_wasm_hash = BytesN::from_array(&env, &empty_sha256);
    assert_eq!(
        native_contract_executable,
        Some(Executable::Wasm(empty_wasm_hash))
    );
    assert!(native_contract_address.exists());
}

#[test]
fn test_get_existing_contract_address_executable_asset() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let sac = env.register_stellar_asset_contract_v2(admin);
    let sac_address = sac.address();
    assert_eq!(sac_address.executable(), Some(Executable::StellarAsset));
    assert!(sac_address.exists());

    let sac_issuer = sac.issuer().address();
    assert_eq!(sac_issuer.executable(), Some(Executable::Account));
    assert!(sac_issuer.exists());
}
