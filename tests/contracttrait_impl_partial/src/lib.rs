#![no_std]
use soroban_sdk::{
    contract, contractimpl, Address, Bytes, BytesN, Duration, Env, Map, String, Symbol, Timepoint,
    Vec, I256, U256,
};
use test_contracttrait_trait::{AllTypes, MyEnumUnit, MyEnumVariants, MyStruct};

#[contract]
pub struct Contract;

#[contractimpl(contracttrait = true)]
impl AllTypes for Contract {
    // Override a few functions with custom implementations

    fn test_u32(v: u32) -> u32 {
        v + 1
    }

    fn test_string(v: String) -> String {
        v
    }

    fn test_env_param(_env: &Env) -> u32 {
        100
    }

    fn test_struct(v: MyStruct) -> MyStruct {
        MyStruct {
            a: v.a * 2,
            b: v.b * 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{map, symbol_short, testutils::Address as _, vec, Env};

    #[test]
    fn test_partial_override() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        // Test overridden functions return custom values
        assert_eq!(client.test_u32(&42u32), 43u32); // 42 + 1
        assert_eq!(client.test_env_param(), 100); // custom value
        let my_struct = MyStruct { a: 10, b: 20 };
        assert_eq!(
            client.test_struct(&my_struct),
            MyStruct { a: 20, b: 40 } // doubled
        );

        // Test non-overridden functions still use defaults
        assert_eq!(client.test_i32(&-42i32), -42i32);
        assert_eq!(client.test_u64(&42u64), 42u64);
        assert_eq!(client.test_i64(&-42i64), -42i64);
        assert_eq!(client.test_u128(&42u128), 42u128);
        assert_eq!(client.test_i128(&-42i128), -42i128);
        assert_eq!(client.test_bool(&true), true);

        // Test SDK types (non-overridden)
        let addr = Address::generate(&e);
        assert_eq!(client.test_address(&addr), addr);

        let bytes = Bytes::from_slice(&e, &[1, 2, 3]);
        assert_eq!(client.test_bytes(&bytes), bytes);

        let bytes_n = BytesN::from_array(&e, &[0u8; 32]);
        assert_eq!(client.test_bytes_n(&bytes_n), bytes_n);

        // test_string is overridden but does the same thing
        let string = String::from_str(&e, "hello");
        assert_eq!(client.test_string(&string), string);

        let symbol = symbol_short!("test");
        assert_eq!(client.test_symbol(&symbol), symbol);

        let vec_val = vec![&e, 1u32, 2u32, 3u32];
        assert_eq!(client.test_vec(&vec_val), vec_val);

        let map_val = map![&e, (1u32, 2u32), (3u32, 4u32)];
        assert_eq!(client.test_map(&map_val), map_val);

        // Test numeric types (non-overridden)
        let duration_val = Duration::from_seconds(&e, 100);
        assert_eq!(client.test_duration(&duration_val), duration_val);

        let timepoint_val = Timepoint::from_unix(&e, 100);
        assert_eq!(client.test_timepoint(&timepoint_val), timepoint_val);

        let i256_val = I256::from_i128(&e, 42);
        assert_eq!(client.test_i256(&i256_val), i256_val);

        let u256_val = U256::from_u128(&e, 42);
        assert_eq!(client.test_u256(&u256_val), u256_val);

        // Test contracttype types (non-overridden)
        assert_eq!(client.test_enum_unit(&MyEnumUnit::A), MyEnumUnit::A);

        let my_enum = MyEnumVariants::VarB(MyStruct { a: 1, b: 2 });
        assert_eq!(client.test_enum_variants(&my_enum), my_enum);
    }
}
