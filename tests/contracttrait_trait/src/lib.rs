#![no_std]
use soroban_sdk::{
    contracttrait, contracttype, Address, Bytes, BytesN, Duration, Env, Map, String, Symbol,
    Timepoint, Vec, I256, U256,
};

// Custom contracttype types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MyStruct {
    pub a: i64,
    pub b: i64,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MyEnumUnit {
    A = 1,
    B = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MyEnumVariants {
    VarA,
    VarB(MyStruct),
    VarC(MyEnumUnit),
}

#[contracttrait]
pub trait AllTypes {
    // Primitives
    fn test_u32(v: u32) -> u32 {
        v
    }
    fn test_i32(v: i32) -> i32 {
        v
    }
    fn test_u64(v: u64) -> u64 {
        v
    }
    fn test_i64(v: i64) -> i64 {
        v
    }
    fn test_u128(v: u128) -> u128 {
        v
    }
    fn test_i128(v: i128) -> i128 {
        v
    }
    fn test_bool(v: bool) -> bool {
        v
    }

    // SDK types
    fn test_address(v: Address) -> Address {
        v
    }
    fn test_bytes(v: Bytes) -> Bytes {
        v
    }
    fn test_bytes_n(v: BytesN<32>) -> BytesN<32> {
        v
    }
    fn test_string(v: String) -> String {
        v
    }
    fn test_symbol(v: Symbol) -> Symbol {
        v
    }
    fn test_vec(v: Vec<u32>) -> Vec<u32> {
        v
    }
    fn test_map(v: Map<u32, u32>) -> Map<u32, u32> {
        v
    }

    // Numeric types
    fn test_duration(v: Duration) -> Duration {
        v
    }
    fn test_timepoint(v: Timepoint) -> Timepoint {
        v
    }
    fn test_i256(v: I256) -> I256 {
        v
    }
    fn test_u256(v: U256) -> U256 {
        v
    }

    // Env parameter
    fn test_env_param(env: &Env) -> u32 {
        let _ = env;
        42
    }

    // Contracttype types
    fn test_struct(v: MyStruct) -> MyStruct {
        v
    }
    fn test_enum_unit(v: MyEnumUnit) -> MyEnumUnit {
        v
    }
    fn test_enum_variants(v: MyEnumVariants) -> MyEnumVariants {
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        contract, contractimpl, map, symbol_short, testutils::Address as _, vec, Env,
    };

    #[contract]
    pub struct Contract;

    #[contractimpl(contracttrait)]
    impl AllTypes for Contract {}

    #[test]
    fn test_types() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

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
}
