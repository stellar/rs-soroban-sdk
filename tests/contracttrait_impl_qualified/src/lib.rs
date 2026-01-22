#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttrait, Address, Bytes, BytesN, Duration, Env, Map, String,
    Symbol, Timepoint, Vec, I256, U256,
};

// Import types but NOT the trait - use fully qualified path for the trait
use test_contracttrait_trait::{MyEnumUnit, MyEnumVariants, MyStruct};

// =============================================================================
// Test 1: Global path (::extern_crate::Trait)
// =============================================================================

#[contract]
pub struct ContractGlobalPath;

// This uses a fully qualified path for the trait instead of importing it.
// The leading :: is required for external crate paths to distinguish from local modules.
#[contractimpl(contracttrait)]
impl ::test_contracttrait_trait::AllTypes for ContractGlobalPath {}

// =============================================================================
// Test 2: crate:: path (crate::module::Trait)
// =============================================================================

// Define a trait within this crate to test crate:: paths
pub mod traits {
    use soroban_sdk::{contracttrait, Env};

    #[contracttrait]
    pub trait CratePathTrait {
        fn crate_path_method(env: &Env) -> u32 {
            let _ = env;
            100
        }
    }
}

#[contract]
pub struct ContractCratePath;

#[contractimpl(contracttrait)]
impl crate::traits::CratePathTrait for ContractCratePath {}

// =============================================================================
// Test 3: self:: path (self::Trait)
// =============================================================================

// Define a trait in this module to test self:: paths
#[contracttrait]
pub trait SelfPathTrait {
    fn self_path_method(env: &Env) -> u32 {
        let _ = env;
        200
    }
}

#[contract]
pub struct ContractSelfPath;

#[contractimpl(contracttrait)]
impl self::SelfPathTrait for ContractSelfPath {}

// =============================================================================
// Test 4: super:: path (super::Trait)
// =============================================================================

// Define a trait to be referenced via super::
#[contracttrait]
pub trait SuperPathTrait {
    fn super_path_method(env: &Env) -> u32 {
        let _ = env;
        300
    }
}

pub mod submodule {
    use soroban_sdk::{contract, contractimpl};

    #[contract]
    pub struct ContractSuperPath;

    #[contractimpl(contracttrait)]
    impl super::SuperPathTrait for ContractSuperPath {}
}

// =============================================================================
// Test 5: Bare trait path (Trait) - trait is imported
// =============================================================================

use test_contracttrait_trait::AllTypes;

#[contract]
pub struct ContractBarePath;

#[contractimpl(contracttrait)]
impl AllTypes for ContractBarePath {}

// =============================================================================
// Test 6: Relative path (module::Trait) - without crate:: or self::
// =============================================================================

#[contract]
pub struct ContractRelativePath;

#[contractimpl(contracttrait)]
impl traits::CratePathTrait for ContractRelativePath {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{map, symbol_short, testutils::Address as _, vec, Env};

    #[test]
    fn test_global_path() {
        let e = Env::default();
        let contract_id = e.register(ContractGlobalPath, ());
        let client = ContractGlobalPathClient::new(&e, &contract_id);

        // Test primitives
        assert_eq!(client.test_u32(&42u32), 42u32);
        assert_eq!(client.test_i32(&-42i32), -42i32);
        assert_eq!(client.test_u64(&42u64), 42u64);
        assert_eq!(client.test_i64(&-42i64), -42i64);
        assert_eq!(client.test_u128(&42u128), 42u128);
        assert_eq!(client.test_i128(&-42i128), -42i128);
        assert_eq!(client.test_bool(&true), true);

        // Test SDK types
        let addr = Address::generate(&e);
        assert_eq!(client.test_address(&addr), addr);

        let bytes = Bytes::from_slice(&e, &[1, 2, 3]);
        assert_eq!(client.test_bytes(&bytes), bytes);

        let bytes_n = BytesN::from_array(&e, &[0u8; 32]);
        assert_eq!(client.test_bytes_n(&bytes_n), bytes_n);

        let string = String::from_str(&e, "hello");
        assert_eq!(client.test_string(&string), string);

        let symbol = symbol_short!("test");
        assert_eq!(client.test_symbol(&symbol), symbol);

        let vec_val = vec![&e, 1u32, 2u32, 3u32];
        assert_eq!(client.test_vec(&vec_val), vec_val);

        let map_val = map![&e, (1u32, 2u32), (3u32, 4u32)];
        assert_eq!(client.test_map(&map_val), map_val);

        // Test numeric types
        let duration_val = Duration::from_seconds(&e, 100);
        assert_eq!(client.test_duration(&duration_val), duration_val);

        let timepoint_val = Timepoint::from_unix(&e, 100);
        assert_eq!(client.test_timepoint(&timepoint_val), timepoint_val);

        let i256_val = I256::from_i128(&e, 42);
        assert_eq!(client.test_i256(&i256_val), i256_val);

        let u256_val = U256::from_u128(&e, 42);
        assert_eq!(client.test_u256(&u256_val), u256_val);

        // Test env param
        assert_eq!(client.test_env_param(), 42);

        // Test contracttype types
        let my_struct = MyStruct { a: 10, b: 20 };
        assert_eq!(client.test_struct(&my_struct), my_struct);

        assert_eq!(client.test_enum_unit(&MyEnumUnit::A), MyEnumUnit::A);

        let my_enum = MyEnumVariants::VarB(MyStruct { a: 1, b: 2 });
        assert_eq!(client.test_enum_variants(&my_enum), my_enum);
    }

    #[test]
    fn test_crate_path() {
        let e = Env::default();
        let contract_id = e.register(ContractCratePath, ());
        let client = ContractCratePathClient::new(&e, &contract_id);

        assert_eq!(client.crate_path_method(), 100);
    }

    #[test]
    fn test_self_path() {
        let e = Env::default();
        let contract_id = e.register(ContractSelfPath, ());
        let client = ContractSelfPathClient::new(&e, &contract_id);

        assert_eq!(client.self_path_method(), 200);
    }

    #[test]
    fn test_super_path() {
        let e = Env::default();
        let contract_id = e.register(submodule::ContractSuperPath, ());
        let client = submodule::ContractSuperPathClient::new(&e, &contract_id);

        assert_eq!(client.super_path_method(), 300);
    }

    #[test]
    fn test_bare_path() {
        let e = Env::default();
        let contract_id = e.register(ContractBarePath, ());
        let client = ContractBarePathClient::new(&e, &contract_id);

        // Test a few methods to ensure the trait is properly implemented
        assert_eq!(client.test_u32(&42u32), 42u32);
        assert_eq!(client.test_bool(&true), true);
        assert_eq!(client.test_env_param(), 42);
    }

    #[test]
    fn test_relative_path() {
        let e = Env::default();
        let contract_id = e.register(ContractRelativePath, ());
        let client = ContractRelativePathClient::new(&e, &contract_id);

        assert_eq!(client.crate_path_method(), 100);
    }
}
