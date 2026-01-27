use core::ops::Bound;

use crate::{self as soroban_sdk, testutils::EnvTestConfig, Env};
use soroban_sdk::contract;

#[contract]
pub struct TestPrngRangeContract;

#[test]
fn test_gen_range_u64_full_range() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let _: u64 = env.prng().gen_range(..);
    });
}

#[test]
fn test_gen_range_u64_inclusive() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let val: u64 = env.prng().gen_range(5..=10);
        assert!(val >= 5 && val <= 10);
    });
}

#[test]
fn test_gen_range_u64_exclusive_end() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let val: u64 = env.prng().gen_range(5..10);
        assert!(val >= 5 && val < 10);
    });
}

#[test]
fn test_gen_range_u64_single_value() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let val: u64 = env.prng().gen_range(7..=7);
        assert_eq!(val, 7);
    });
}

// Excluded(u64::MAX) start computes u64::MAX + 1 which overflows.
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_gen_range_u64_excluded_start_u64_max_overflows() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let _: u64 = env
            .prng()
            .gen_range((Bound::Excluded(u64::MAX), Bound::Unbounded));
    });
}

#[test]
fn test_gen_range_u64_included_end_u64_max() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let _: u64 = env.prng().gen_range(0u64..=u64::MAX);
    });
}

// Excluded(0) end computes 0 - 1 which underflows.
#[should_panic(expected = "attempt to subtract with overflow")]
#[test]
fn test_gen_range_u64_excluded_end_zero_underflows() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let _: u64 = env
            .prng()
            .gen_range((Bound::<u64>::Unbounded, Bound::Excluded(0)));
    });
}

#[test]
fn test_gen_range_u64_both_u64_max() {
    let env = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    let id = env.register(TestPrngRangeContract, ());
    env.as_contract(&id, || {
        let val: u64 = env.prng().gen_range(u64::MAX..=u64::MAX);
        assert_eq!(val, u64::MAX);
    });
}
