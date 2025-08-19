use crate::{self as soroban_sdk, Env};

/// Test contract that will trigger various resource usage patterns
#[crate::contract]
pub struct TestLimitsContract;

#[crate::contractimpl]
impl TestLimitsContract {
    pub fn write_entries(env: Env, count: u32) {
        for i in 0..count {
            env.storage().persistent().set(&i, &i);
        }
    }
    
    pub fn read_entries(env: Env, count: u32) -> u32 {
        let mut sum = 0;
        for i in 0..count {
            if let Some(val) = env.storage().persistent().get::<u32, u32>(&i) {
                sum += val;
            }
        }
        sum
    }
    
    pub fn write_large_data(env: Env, size_kb: u32) {
        let data_vec: std::vec::Vec<u8> = (0..size_kb * 1024).map(|i| (i % 256) as u8).collect();
        let large_data = crate::Bytes::from_slice(&env, &data_vec);
        env.storage().persistent().set(&1u32, &large_data);
    }
    
    pub fn emit_events(env: Env, count: u32) {
        for i in 0..count {
            // Use basic logging to trigger events
            env.logs().add("event", &[i.into()]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutils::budget::StellarCoreLimits;

    #[test]
    fn test_current_resource_tracking() {
        let env = Env::default();
        let contract_id = env.register(TestLimitsContract, ());
        let client = TestLimitsContractClient::new(&env, &contract_id);
        
        // Test write entries
        client.write_entries(&5);
        let resources = env.cost_estimate().resources();
        
        println!("Write entries resources: {:#?}", resources);
        assert!(resources.write_entries > 0);
        
        // Test reading entries
        client.read_entries(&5);
        let resources2 = env.cost_estimate().resources();
        
        println!("Read entries resources: {:#?}", resources2);
        // Note: read_entries should be tracked but may not be enforced yet
        
        // Test large data write
        client.write_large_data(&10); // 10KB
        let resources3 = env.cost_estimate().resources();
        
        println!("Large data write resources: {:#?}", resources3);
        assert!(resources3.write_bytes > 0);
    }

    #[test]
    fn test_stellar_limits_manual_check() {
        let env = Env::default();
        let contract_id = env.register(TestLimitsContract, ());
        let client = TestLimitsContractClient::new(&env, &contract_id);
        
        // Set some strict limits
        let limits = StellarCoreLimits {
            write_entries: Some(3),  // Allow only 3 write entries
            write_bytes: Some(1000), // Allow only 1000 bytes to be written
            ..Default::default()
        };
        env.cost_estimate().set_stellar_limits(limits);
        
        // This should succeed (5 entries exceeds limit but we're not enforcing automatically yet)
        client.write_entries(&5);
        let resources = env.cost_estimate().resources();
        println!("Resources after 5 write entries: {:#?}", resources);
        
        // Manual check should fail
        match env.cost_estimate().check_stellar_limits() {
            Ok(()) => panic!("Expected limit check to fail"),
            Err(msg) => {
                println!("Limit check failed as expected: {}", msg);
                assert!(msg.contains("Write entries limit exceeded"));
            }
        }
        
        // Check the limits can be retrieved
        let retrieved_limits = env.cost_estimate().get_stellar_limits();
        assert_eq!(retrieved_limits.write_entries, Some(3));
        assert_eq!(retrieved_limits.write_bytes, Some(1000));
    }

    #[test]
    #[should_panic(expected = "Budget")]
    fn test_stellar_limits_enforcement() {
        let env = Env::default();
        let contract_id = env.register(TestLimitsContract, ());
        let client = TestLimitsContractClient::new(&env, &contract_id);
        
        // Set very strict limits
        let limits = StellarCoreLimits {
            write_entries: Some(2),  // Allow only 2 write entries
            ..Default::default()
        };
        env.set_stellar_limits(limits);
        
        // Execute the operation that exceeds limits
        client.write_entries(&5);
        
        // This should panic due to enforcement
        env.cost_estimate().enforce_stellar_limits();
    }

    #[test]  
    fn test_stellar_limits_enforcement_under_limit() {
        let env = Env::default();
        let contract_id = env.register(TestLimitsContract, ());
        let client = TestLimitsContractClient::new(&env, &contract_id);
        
        // Set limits that should allow the operation
        let limits = StellarCoreLimits {
            write_entries: Some(10),  // Allow up to 10 write entries
            write_bytes: Some(5000),  // Allow up to 5000 bytes
            ..Default::default()
        };
        env.set_stellar_limits(limits);
        
        // This should succeed (5 entries, well under the limit of 10)
        client.write_entries(&5);
        
        // Verify the resources were consumed but under limits
        let resources = env.cost_estimate().resources();
        assert_eq!(resources.write_entries, 5);
        assert!(resources.write_bytes < 5000);
        
        // Enforcement check should pass (no panic)
        env.cost_estimate().enforce_stellar_limits();
    }

    #[test]
    fn test_all_stellar_limits_types() {
        let env = Env::default();
        let contract_id = env.register(TestLimitsContract, ());
        let client = TestLimitsContractClient::new(&env, &contract_id);
        
        // Test write_bytes limit enforcement
        let limits = StellarCoreLimits {
            write_bytes: Some(100), // Very small limit
            ..Default::default()
        };
        env.set_stellar_limits(limits);
        
        // This should trigger write_bytes limit
        client.write_large_data(&10); // 10KB should exceed 100 bytes
        
        match env.cost_estimate().check_stellar_limits() {
            Ok(()) => panic!("Expected write_bytes limit to be exceeded"),
            Err(msg) => assert!(msg.contains("Write bytes limit exceeded")),
        }
    }

    #[test]
    fn test_stellar_limits_example_usage() {
        // This example shows how developers can use stellar-core limits
        // to test their contracts under realistic resource constraints
        
        let env = Env::default();
        let contract_id = env.register(TestLimitsContract, ());
        let client = TestLimitsContractClient::new(&env, &contract_id);
        
        // Set realistic stellar-core limits similar to mainnet
        let mainnet_like_limits = StellarCoreLimits {
            read_entries: Some(40),      // Mainnet-like read entries limit
            write_entries: Some(25),     // Mainnet-like write entries limit  
            read_bytes: Some(130_000),   // Mainnet-like read bytes limit
            write_bytes: Some(65_000),   // Mainnet-like write bytes limit
            contract_events_size_bytes: Some(8192), // Mainnet-like events limit
        };
        
        // Apply the limits 
        env.set_stellar_limits(mainnet_like_limits);
        
        // Test operations under these constraints
        client.write_entries(&5); // Should be fine (5 < 25)
        
        // Verify we're under limits
        env.cost_estimate().enforce_stellar_limits(); // Won't panic
        
        // Show that we can inspect the current usage
        let resources = env.cost_estimate().resources();
        let limits = env.get_stellar_limits();
        
        println!("Used write entries: {}/{:?}", resources.write_entries, limits.write_entries);
        println!("Used write bytes: {}/{:?}", resources.write_bytes, limits.write_bytes);
        
        // This demonstrates how contract developers can catch limit issues during testing
        // instead of discovering them after deploying to testnet
        assert!(resources.write_entries <= limits.write_entries.unwrap());
        assert!(resources.write_bytes as u32 <= limits.write_bytes.unwrap());
    }
}