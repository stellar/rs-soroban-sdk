use core::ops::Bound;

use crate::{Bytes, Env};

#[test]
fn test_slice_full_range() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let sliced = bytes.slice(0..3);
    assert_eq!(sliced.len(), 3);
    assert_eq!(sliced.get_unchecked(0), 1);
    assert_eq!(sliced.get_unchecked(1), 2);
    assert_eq!(sliced.get_unchecked(2), 3);
}

#[test]
fn test_slice_middle() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let sliced = bytes.slice(1..2);
    assert_eq!(sliced.len(), 1);
    assert_eq!(sliced.get_unchecked(0), 2);
}

// Excluded(u32::MAX) start computes u32::MAX + 1 which overflows.
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_slice_excluded_start_u32_max_overflows_panics_guest_side() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let _ = bytes.slice((Bound::Excluded(u32::MAX), Bound::Unbounded));
}

// Included(u32::MAX) end computes u32::MAX + 1 which overflows.
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_slice_included_end_u32_max_overflows_panics_guest_side() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let _ = bytes.slice(0..=u32::MAX);
}

// Both bounds u32::MAX: Excluded start computes u32::MAX + 1 which overflows,
// and Included end computes u32::MAX + 1 which also overflows.
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_slice_both_u32_max_overflows_panics_guest_side() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let _ = bytes.slice(u32::MAX..=u32::MAX);
}

#[test]
#[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
fn test_slice_out_of_bounds_end_host_side() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let _ = bytes.slice(0..4);
}

#[test]
#[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
fn test_slice_out_of_bounds_start_host_side() {
    let env = Env::default();
    let bytes = Bytes::from_slice(&env, &[1, 2, 3]);
    let _ = bytes.slice(4..);
}

#[test]
fn test_slice_empty_full_range() {
    let env = Env::default();
    let bytes = Bytes::new(&env);
    let sliced = bytes.slice(..);
    assert_eq!(sliced.len(), 0);
}

#[test]
fn test_slice_empty_zero_range() {
    let env = Env::default();
    let bytes = Bytes::new(&env);
    let sliced = bytes.slice(0..0);
    assert_eq!(sliced.len(), 0);
}
