#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Map, Symbol, TryFromVal, Val};

/// Original data structure with two fields
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataV1 {
    pub a: i64,
    pub b: i64,
}

/// Updated data structure with an extra field
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataV2 {
    pub a: i64,
    pub b: i64,
    pub c: Option<i64>,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Data,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Save data using the V1 struct (fewer fields)
    pub fn save_v1(env: Env, a: i64, b: i64) {
        let data = DataV1 { a, b };
        env.storage().persistent().set(&DataKey::Data, &data);
    }

    /// Save data using the V2 struct (more fields)
    pub fn save_v2(env: Env, a: i64, b: i64, c: i64) {
        let data = DataV2 { a, b, c: Some(c) };
        env.storage().persistent().set(&DataKey::Data, &data);
    }

    /// WORKING APPROACH: Read with fallback using Map inspection.
    ///
    /// This approach works because we first read the data as a raw Map,
    /// inspect its structure (field count), and then read with the
    /// appropriate struct type.
    ///
    /// Steps:
    /// 1. Read the raw storage as Map<Symbol, Val> to inspect structure
    /// 2. Check the number of fields to determine which version was stored
    /// 3. Read again with the appropriate struct type
    pub fn read_with_fallback_map(env: Env) -> DataV2 {
        // Read as raw Map to inspect the structure without conversion failure
        let raw: Map<Symbol, Val> = env.storage().persistent().get(&DataKey::Data).unwrap();

        // Check if we have 3 fields (V2) or 2 fields (V1)
        if raw.len() == 3 {
            // V2 format - all fields present, read directly as V2
            env.storage().persistent().get(&DataKey::Data).unwrap()
        } else {
            // V1 format - only 2 fields, read as V1 and convert to DataV2
            let v1: DataV1 = env.storage().persistent().get(&DataKey::Data).unwrap();
            DataV2 {
                a: v1.a,
                b: v1.b,
                c: None,
            }
        }
    }

    /// TRAPPING APPROACH: Attempt to read with try_from_val fallback.
    ///
    /// WARNING: This approach DOES NOT WORK as intended!
    ///
    /// When the struct field count doesn't match the stored data, the host
    /// environment traps (panics) before try_from_val can return an Err.
    /// The error "differing host map and output slice lengths when unpacking
    /// map to slice" is raised at the host level and cannot be caught.
    ///
    /// This function is included to demonstrate the limitation.
    pub fn read_with_fallback_try(env: Env) -> DataV2 {
        // Read raw Val from storage
        let raw: Val = env.storage().persistent().get(&DataKey::Data).unwrap();

        // Try to unpack as DataV2 first
        // NOTE: This will TRAP (not return Err) if field count doesn't match!
        if let Ok(v2) = DataV2::try_from_val(&env, &raw) {
            v2
        } else {
            // This branch is never reached when reading V1 data because
            // the host traps before try_from_val returns
            let v1 = DataV1::try_from_val(&env, &raw).unwrap();
            DataV2 {
                a: v1.a,
                b: v1.b,
                c: None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::EnvTestConfig, Env};

    // =========================================================================
    // Tests for the WORKING Map-based approach
    // =========================================================================

    #[test]
    fn test_map_approach_read_v1() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Save data using V1 struct (only 2 fields)
        client.save_v1(&10, &20);

        // Read with Map-based fallback - WORKS: detects V1 and uses None for c
        let data = client.read_with_fallback_map();
        assert_eq!(data.a, 10);
        assert_eq!(data.b, 20);
        assert_eq!(data.c, None);
    }

    #[test]
    fn test_map_approach_read_v2() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Save data using V2 struct (all 3 fields)
        client.save_v2(&10, &20, &30);

        // Read with Map-based fallback - WORKS: detects V2 and reads all fields
        let data = client.read_with_fallback_map();
        assert_eq!(data.a, 10);
        assert_eq!(data.b, 20);
        assert_eq!(data.c, Some(30));
    }

    #[test]
    fn test_map_approach_upgrade_scenario() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Scenario: Data was originally saved with V1
        client.save_v1(&100, &200);

        // Read back with V2-aware function (simulates upgraded contract)
        let data = client.read_with_fallback_map();
        assert_eq!(data.a, 100);
        assert_eq!(data.b, 200);
        assert_eq!(data.c, None); // Falls back to None

        // Now save with V2 (upgraded contract saves new format)
        client.save_v2(&100, &200, &300);

        // Read again - should now get full V2 data
        let data = client.read_with_fallback_map();
        assert_eq!(data.a, 100);
        assert_eq!(data.b, 200);
        assert_eq!(data.c, Some(300));
    }

    // =========================================================================
    // Tests for the TRAPPING try_from_val approach
    // =========================================================================

    #[test]
    fn test_try_approach_read_v2_works() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Save data using V2 struct (all 3 fields)
        client.save_v2(&10, &20, &30);

        // Read with try_from_val approach - WORKS when data matches expected struct
        let data = client.read_with_fallback_try();
        assert_eq!(data.a, 10);
        assert_eq!(data.b, 20);
        assert_eq!(data.c, Some(30));
    }

    #[test]
    #[should_panic(expected = "Error(Object, UnexpectedSize)")]
    fn test_try_approach_read_v1_traps() {
        let env = Env::new_with_config(EnvTestConfig {
            capture_snapshot_at_drop: false,
        });
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        // Save data using V1 struct (only 2 fields)
        client.save_v1(&10, &20);

        // Read with try_from_val approach - TRAPS!
        // The host panics with "differing host map and output slice lengths"
        // before try_from_val can return Err, so fallback logic never executes.
        let _data = client.read_with_fallback_try();
    }
}
