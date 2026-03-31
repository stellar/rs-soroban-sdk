use crate::{Env, I256, U256};

// ============================================================
// U256 checked arithmetic
// ============================================================

#[test]
fn test_u256_checked_add_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 6);
    let b = U256::from_u32(&env, 3);
    assert_eq!(a.checked_add(&b), Some(U256::from_u32(&env, 9)));
}

#[test]
fn test_u256_checked_add_zero() {
    let env = Env::default();
    let a = U256::from_u32(&env, 42);
    let zero = U256::from_u32(&env, 0);
    assert_eq!(a.checked_add(&zero), Some(U256::from_u32(&env, 42)));
}

#[test]
fn test_u256_checked_add_overflow() {
    let env = Env::default();
    let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
    let one = U256::from_u32(&env, 1);
    assert_eq!(max.checked_add(&one), None);
}

#[test]
fn test_u256_checked_sub_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 10);
    let b = U256::from_u32(&env, 3);
    assert_eq!(a.checked_sub(&b), Some(U256::from_u32(&env, 7)));
}

#[test]
fn test_u256_checked_sub_zero_result() {
    let env = Env::default();
    let a = U256::from_u32(&env, 5);
    assert_eq!(a.checked_sub(&a), Some(U256::from_u32(&env, 0)));
}

#[test]
fn test_u256_checked_sub_underflow() {
    let env = Env::default();
    let a = U256::from_u32(&env, 3);
    let b = U256::from_u32(&env, 10);
    assert_eq!(a.checked_sub(&b), None);
}

#[test]
fn test_u256_checked_mul_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 6);
    let b = U256::from_u32(&env, 7);
    assert_eq!(a.checked_mul(&b), Some(U256::from_u32(&env, 42)));
}

#[test]
fn test_u256_checked_mul_zero() {
    let env = Env::default();
    let a = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
    let zero = U256::from_u32(&env, 0);
    assert_eq!(a.checked_mul(&zero), Some(U256::from_u32(&env, 0)));
}

#[test]
fn test_u256_checked_mul_overflow() {
    let env = Env::default();
    let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
    let two = U256::from_u32(&env, 2);
    assert_eq!(max.checked_mul(&two), None);
}

#[test]
fn test_u256_checked_pow_success() {
    let env = Env::default();
    let base = U256::from_u32(&env, 3);
    assert_eq!(base.checked_pow(4), Some(U256::from_u32(&env, 81)));
}

#[test]
fn test_u256_checked_pow_zero_exponent() {
    let env = Env::default();
    let base = U256::from_u32(&env, 100);
    assert_eq!(base.checked_pow(0), Some(U256::from_u32(&env, 1)));
}

#[test]
fn test_u256_checked_pow_overflow() {
    let env = Env::default();
    let base = U256::from_parts(&env, 0, 0, 0, u64::MAX);
    assert_eq!(base.checked_pow(256), None);
}

#[test]
fn test_u256_checked_div_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 42);
    let b = U256::from_u32(&env, 6);
    assert_eq!(a.checked_div(&b), Some(U256::from_u32(&env, 7)));
}

#[test]
fn test_u256_checked_div_by_one() {
    let env = Env::default();
    let a = U256::from_u32(&env, 42);
    let one = U256::from_u32(&env, 1);
    assert_eq!(a.checked_div(&one), Some(U256::from_u32(&env, 42)));
}

#[test]
fn test_u256_checked_div_zero_dividend() {
    let env = Env::default();
    let zero = U256::from_u32(&env, 0);
    let b = U256::from_u32(&env, 5);
    assert_eq!(zero.checked_div(&b), Some(U256::from_u32(&env, 0)));
}

#[test]
fn test_u256_checked_div_by_zero() {
    let env = Env::default();
    let a = U256::from_u32(&env, 42);
    let zero = U256::from_u32(&env, 0);
    assert_eq!(a.checked_div(&zero), None);
}

#[test]
fn test_u256_checked_rem_euclid_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 10);
    let b = U256::from_u32(&env, 3);
    assert_eq!(a.checked_rem_euclid(&b), Some(U256::from_u32(&env, 1)));
}

#[test]
fn test_u256_checked_rem_euclid_exact() {
    let env = Env::default();
    let a = U256::from_u32(&env, 9);
    let b = U256::from_u32(&env, 3);
    assert_eq!(a.checked_rem_euclid(&b), Some(U256::from_u32(&env, 0)));
}

#[test]
fn test_u256_checked_rem_euclid_by_zero() {
    let env = Env::default();
    let a = U256::from_u32(&env, 10);
    let zero = U256::from_u32(&env, 0);
    assert_eq!(a.checked_rem_euclid(&zero), None);
}

#[test]
fn test_u256_checked_shl_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 1);
    assert_eq!(a.checked_shl(8), Some(U256::from_u32(&env, 256)));
}

#[test]
fn test_u256_checked_shl_zero_shift() {
    let env = Env::default();
    let a = U256::from_u32(&env, 42);
    assert_eq!(a.checked_shl(0), Some(U256::from_u32(&env, 42)));
}

#[test]
fn test_u256_checked_shl_max_valid() {
    let env = Env::default();
    let one = U256::from_u32(&env, 1);
    let result = one.checked_shl(255);
    // 1 << 255 = high bit of hi_hi set
    let expected = U256::from_parts(&env, 1u64 << 63, 0, 0, 0);
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn test_u256_checked_shl_overflow_256() {
    let env = Env::default();
    let a = U256::from_u32(&env, 1);
    assert_eq!(a.checked_shl(256), None);
}

#[test]
fn test_u256_checked_shr_success() {
    let env = Env::default();
    let a = U256::from_u32(&env, 256);
    assert_eq!(a.checked_shr(4), Some(U256::from_u32(&env, 16)));
}

#[test]
fn test_u256_checked_shr_zero_shift() {
    let env = Env::default();
    let a = U256::from_u32(&env, 42);
    assert_eq!(a.checked_shr(0), Some(U256::from_u32(&env, 42)));
}

#[test]
fn test_u256_checked_shr_max_valid() {
    let env = Env::default();
    let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
    assert_eq!(max.checked_shr(255), Some(U256::from_u32(&env, 1)));
}

#[test]
fn test_u256_checked_shr_overflow_256() {
    let env = Env::default();
    let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
    assert_eq!(max.checked_shr(256), None);
}

// ============================================================
// I256 checked arithmetic
// ============================================================

#[test]
fn test_i256_checked_add_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, -6);
    let b = I256::from_i32(&env, 3);
    assert_eq!(a.checked_add(&b), Some(I256::from_i32(&env, -3)));
}

#[test]
fn test_i256_checked_add_zero() {
    let env = Env::default();
    let a = I256::from_i32(&env, -42);
    let zero = I256::from_i32(&env, 0);
    assert_eq!(a.checked_add(&zero), Some(I256::from_i32(&env, -42)));
}

#[test]
fn test_i256_checked_add_overflow_positive() {
    let env = Env::default();
    // I256::MAX = 2^255 - 1
    let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
    let one = I256::from_i32(&env, 1);
    assert_eq!(max.checked_add(&one), None);
}

#[test]
fn test_i256_checked_add_overflow_negative() {
    let env = Env::default();
    // I256::MIN = -2^255
    let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
    let neg_one = I256::from_i32(&env, -1);
    assert_eq!(min.checked_add(&neg_one), None);
}

#[test]
fn test_i256_checked_sub_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, 10);
    let b = I256::from_i32(&env, 3);
    assert_eq!(a.checked_sub(&b), Some(I256::from_i32(&env, 7)));
}

#[test]
fn test_i256_checked_sub_negative_result() {
    let env = Env::default();
    let a = I256::from_i32(&env, 3);
    let b = I256::from_i32(&env, 10);
    assert_eq!(a.checked_sub(&b), Some(I256::from_i32(&env, -7)));
}

#[test]
fn test_i256_checked_sub_overflow() {
    let env = Env::default();
    let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
    let one = I256::from_i32(&env, 1);
    assert_eq!(min.checked_sub(&one), None);
}

#[test]
fn test_i256_checked_mul_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, -6);
    let b = I256::from_i32(&env, 7);
    assert_eq!(a.checked_mul(&b), Some(I256::from_i32(&env, -42)));
}

#[test]
fn test_i256_checked_mul_zero() {
    let env = Env::default();
    let a = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
    let zero = I256::from_i32(&env, 0);
    assert_eq!(a.checked_mul(&zero), Some(I256::from_i32(&env, 0)));
}

#[test]
fn test_i256_checked_mul_overflow_positive() {
    let env = Env::default();
    let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
    let two = I256::from_i32(&env, 2);
    assert_eq!(max.checked_mul(&two), None);
}

#[test]
fn test_i256_checked_mul_overflow_negative() {
    let env = Env::default();
    // I256::MIN * -1 overflows because |I256::MIN| > I256::MAX
    let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
    let neg_one = I256::from_i32(&env, -1);
    assert_eq!(min.checked_mul(&neg_one), None);
}

#[test]
fn test_i256_checked_pow_success() {
    let env = Env::default();
    let base = I256::from_i32(&env, -3);
    assert_eq!(base.checked_pow(3), Some(I256::from_i32(&env, -27)));
}

#[test]
fn test_i256_checked_pow_even_exponent() {
    let env = Env::default();
    let base = I256::from_i32(&env, -2);
    assert_eq!(base.checked_pow(4), Some(I256::from_i32(&env, 16)));
}

#[test]
fn test_i256_checked_pow_zero_exponent() {
    let env = Env::default();
    let base = I256::from_i32(&env, -100);
    assert_eq!(base.checked_pow(0), Some(I256::from_i32(&env, 1)));
}

#[test]
fn test_i256_checked_pow_overflow() {
    let env = Env::default();
    let base = I256::from_parts(&env, 0, 0, 0, i64::MAX as u64);
    assert_eq!(base.checked_pow(256), None);
}

#[test]
fn test_i256_checked_div_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, -42);
    let b = I256::from_i32(&env, 6);
    assert_eq!(a.checked_div(&b), Some(I256::from_i32(&env, -7)));
}

#[test]
fn test_i256_checked_div_by_one() {
    let env = Env::default();
    let a = I256::from_i32(&env, -42);
    let one = I256::from_i32(&env, 1);
    assert_eq!(a.checked_div(&one), Some(I256::from_i32(&env, -42)));
}

#[test]
fn test_i256_checked_div_zero_dividend() {
    let env = Env::default();
    let zero = I256::from_i32(&env, 0);
    let b = I256::from_i32(&env, 5);
    assert_eq!(zero.checked_div(&b), Some(I256::from_i32(&env, 0)));
}

#[test]
fn test_i256_checked_div_by_neg_one() {
    let env = Env::default();
    let a = I256::from_i32(&env, 42);
    let neg_one = I256::from_i32(&env, -1);
    assert_eq!(a.checked_div(&neg_one), Some(I256::from_i32(&env, -42)));
}

#[test]
fn test_i256_checked_div_by_zero() {
    let env = Env::default();
    let a = I256::from_i32(&env, 42);
    let zero = I256::from_i32(&env, 0);
    assert_eq!(a.checked_div(&zero), None);
}

#[test]
fn test_i256_checked_div_min_by_neg_one() {
    let env = Env::default();
    let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
    let neg_one = I256::from_i32(&env, -1);
    assert_eq!(min.checked_div(&neg_one), None);
}

#[test]
fn test_i256_checked_rem_euclid_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, -10);
    let b = I256::from_i32(&env, 3);
    // Euclidean remainder is always non-negative for positive divisor
    assert_eq!(a.checked_rem_euclid(&b), Some(I256::from_i32(&env, 2)));
}

#[test]
fn test_i256_checked_rem_euclid_exact() {
    let env = Env::default();
    let a = I256::from_i32(&env, 9);
    let b = I256::from_i32(&env, 3);
    assert_eq!(a.checked_rem_euclid(&b), Some(I256::from_i32(&env, 0)));
}

#[test]
fn test_i256_checked_rem_euclid_by_zero() {
    let env = Env::default();
    let a = I256::from_i32(&env, 10);
    let zero = I256::from_i32(&env, 0);
    assert_eq!(a.checked_rem_euclid(&zero), None);
}

#[test]
fn test_i256_checked_rem_euclid_min_by_neg_one() {
    let env = Env::default();
    // I256::MIN % -1 overflows because rem_euclid internally performs a division
    let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
    let neg_one = I256::from_i32(&env, -1);
    assert_eq!(min.checked_rem_euclid(&neg_one), None);
}

#[test]
fn test_i256_checked_shl_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, 1);
    assert_eq!(a.checked_shl(8), Some(I256::from_i32(&env, 256)));
}

#[test]
fn test_i256_checked_shl_zero_shift() {
    let env = Env::default();
    let a = I256::from_i32(&env, -42);
    assert_eq!(a.checked_shl(0), Some(I256::from_i32(&env, -42)));
}

#[test]
fn test_i256_checked_shl_overflow_256() {
    let env = Env::default();
    let a = I256::from_i32(&env, 1);
    assert_eq!(a.checked_shl(256), None);
}

#[test]
fn test_i256_checked_shr_success() {
    let env = Env::default();
    let a = I256::from_i32(&env, 256);
    assert_eq!(a.checked_shr(4), Some(I256::from_i32(&env, 16)));
}

#[test]
fn test_i256_checked_shr_zero_shift() {
    let env = Env::default();
    let a = I256::from_i32(&env, -42);
    assert_eq!(a.checked_shr(0), Some(I256::from_i32(&env, -42)));
}

#[test]
fn test_i256_checked_shr_negative() {
    let env = Env::default();
    let a = I256::from_i32(&env, -256);
    // Arithmetic right shift preserves sign
    assert_eq!(a.checked_shr(4), Some(I256::from_i32(&env, -16)));
}

#[test]
fn test_i256_checked_shr_max_valid() {
    let env = Env::default();
    // I256::MAX >> 255 == 0 (sign bit is 0, arithmetic shift fills with 0s)
    let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
    assert_eq!(max.checked_shr(255), Some(I256::from_i32(&env, 0)));
}

#[test]
fn test_i256_checked_shr_overflow_256() {
    let env = Env::default();
    let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
    assert_eq!(max.checked_shr(256), None);
}

// ============================================================
// Consistency: checked vs unchecked produce same result on success
// ============================================================

#[test]
fn test_u256_checked_matches_unchecked() {
    let env = Env::default();
    let a = U256::from_u32(&env, 100);
    let b = U256::from_u32(&env, 37);

    assert_eq!(a.checked_add(&b).unwrap(), a.add(&b));
    assert_eq!(a.checked_sub(&b).unwrap(), a.sub(&b));
    assert_eq!(a.checked_mul(&b).unwrap(), a.mul(&b));
    assert_eq!(a.checked_pow(3).unwrap(), a.pow(3));
    assert_eq!(a.checked_div(&b).unwrap(), a.div(&b));
    assert_eq!(a.checked_rem_euclid(&b).unwrap(), a.rem_euclid(&b));
    assert_eq!(a.checked_shl(3).unwrap(), a.shl(3));
    assert_eq!(a.checked_shr(3).unwrap(), a.shr(3));
}

#[test]
fn test_i256_checked_matches_unchecked() {
    let env = Env::default();
    let a = I256::from_i32(&env, -50);
    let b = I256::from_i32(&env, 13);

    assert_eq!(a.checked_add(&b).unwrap(), a.add(&b));
    assert_eq!(a.checked_sub(&b).unwrap(), a.sub(&b));
    assert_eq!(a.checked_mul(&b).unwrap(), a.mul(&b));
    assert_eq!(a.checked_pow(2).unwrap(), a.pow(2));
    assert_eq!(a.checked_div(&b).unwrap(), a.div(&b));
    assert_eq!(a.checked_rem_euclid(&b).unwrap(), a.rem_euclid(&b));
    assert_eq!(a.checked_shl(3).unwrap(), a.shl(3));
    assert_eq!(a.checked_shr(3).unwrap(), a.shr(3));
}
