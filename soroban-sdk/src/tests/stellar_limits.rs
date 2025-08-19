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
}