use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{
        DurationSmall, DurationVal, Env as _, I256Small, I256Val, TimepointSmall, TimepointVal,
        U256Small, U256Val,
    },
    Bytes, ConversionError, Env, TryFromVal, TryIntoVal, Val,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
use crate::unwrap::{UnwrapInfallible, UnwrapOptimized};

macro_rules! impl_num_wrapping_val_type {
    ($wrapper:ident, $val:ty, $small:ty) => {
        impl Debug for $wrapper {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                // FIXME: properly print it when we have the conversion functions
                write!(f, "{:?}", self.val.as_val())
            }
        }

        impl Eq for $wrapper {}

        impl PartialEq for $wrapper {
            fn eq(&self, other: &Self) -> bool {
                self.partial_cmp(other) == Some(Ordering::Equal)
            }
        }

        impl PartialOrd for $wrapper {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(Ord::cmp(self, other))
            }
        }

        impl Ord for $wrapper {
            fn cmp(&self, other: &Self) -> Ordering {
                let self_raw = self.val.to_val();
                let other_raw = other.val.to_val();

                match (<$small>::try_from(self_raw), <$small>::try_from(other_raw)) {
                    // Compare small numbers.
                    (Ok(self_num), Ok(other_num)) => self_num.cmp(&other_num),
                    // The object-to-small number comparisons are handled by `obj_cmp`,
                    // so it's safe to handle all the other cases using it.
                    _ => {
                        #[cfg(not(target_family = "wasm"))]
                        if !self.env.is_same_env(&other.env) {
                            return ScVal::from(self).cmp(&ScVal::from(other));
                        }
                        let v = self.env.obj_cmp(self_raw, other_raw).unwrap_infallible();
                        v.cmp(&0)
                    }
                }
            }
        }

        impl From<$wrapper> for $val {
            #[inline(always)]
            fn from(v: $wrapper) -> Self {
                v.val
            }
        }

        impl From<&$wrapper> for $val {
            #[inline(always)]
            fn from(v: &$wrapper) -> Self {
                v.val
            }
        }

        impl From<&$wrapper> for $wrapper {
            #[inline(always)]
            fn from(v: &$wrapper) -> Self {
                v.clone()
            }
        }

        impl TryFromVal<Env, $val> for $wrapper {
            type Error = Infallible;

            fn try_from_val(env: &Env, val: &$val) -> Result<Self, Self::Error> {
                Ok(unsafe { $wrapper::unchecked_new(env.clone(), *val) })
            }
        }

        impl TryFromVal<Env, Val> for $wrapper {
            type Error = ConversionError;

            fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
                Ok(<$val>::try_from_val(env, val)?
                    .try_into_val(env)
                    .unwrap_infallible())
            }
        }

        impl TryFromVal<Env, $wrapper> for Val {
            type Error = ConversionError;

            fn try_from_val(_env: &Env, v: &$wrapper) -> Result<Self, Self::Error> {
                Ok(v.to_val())
            }
        }

        impl TryFromVal<Env, &$wrapper> for Val {
            type Error = ConversionError;

            fn try_from_val(_env: &Env, v: &&$wrapper) -> Result<Self, Self::Error> {
                Ok(v.to_val())
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl From<&$wrapper> for ScVal {
            fn from(v: &$wrapper) -> Self {
                // This conversion occurs only in test utilities, and theoretically all
                // values should convert to an ScVal because the Env won't let the host
                // type to exist otherwise, unwrapping. Even if there are edge cases
                // that don't, this is a trade off for a better test developer
                // experience.
                if let Ok(ss) = <$small>::try_from(v.val) {
                    ScVal::try_from(ss).unwrap()
                } else {
                    ScVal::try_from_val(&v.env, &v.to_val()).unwrap()
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl From<$wrapper> for ScVal {
            fn from(v: $wrapper) -> Self {
                (&v).into()
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl TryFromVal<Env, ScVal> for $wrapper {
            type Error = ConversionError;
            fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
                Ok(<$val>::try_from_val(env, &Val::try_from_val(env, val)?)?
                    .try_into_val(env)
                    .unwrap_infallible())
            }
        }

        impl $wrapper {
            #[inline(always)]
            pub(crate) unsafe fn unchecked_new(env: Env, val: $val) -> Self {
                Self { env, val }
            }

            #[inline(always)]
            pub fn env(&self) -> &Env {
                &self.env
            }

            pub fn as_val(&self) -> &Val {
                self.val.as_val()
            }

            pub fn to_val(&self) -> Val {
                self.val.to_val()
            }

            pub fn to_val_type(&self) -> $val {
                self.val
            }
        }
    };
}

/// U256 holds a 256-bit unsigned integer.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{U256, Env};
///
/// let env = Env::default();
/// let u1 = U256::from_u32(&env, 6);
/// let u2 = U256::from_u32(&env, 3);
/// assert_eq!(u1.add(&u2), U256::from_u32(&env, 9));
/// ```
#[derive(Clone)]
pub struct U256 {
    env: Env,
    val: U256Val,
}

impl_num_wrapping_val_type!(U256, U256Val, U256Small);

impl U256 {
    /// Returns zero.
    pub fn zero(env: &Env) -> U256 {
        U256::from_u32(env, 0)
    }

    /// Returns the minimum value for U256 (0).
    pub fn min_value(env: &Env) -> U256 {
        U256::zero(env)
    }

    /// Returns the maximum value for U256 (2^256 - 1).
    pub fn max_value(env: &Env) -> U256 {
        U256::from_parts(env, u64::MAX, u64::MAX, u64::MAX, u64::MAX)
    }

    pub fn from_u32(env: &Env, u: u32) -> Self {
        U256 {
            env: env.clone(),
            val: U256Val::from_u32(u),
        }
    }

    pub fn from_u128(env: &Env, u: u128) -> Self {
        let lo: Bytes = Bytes::from_array(env, &u.to_be_bytes());
        let mut bytes: Bytes = Bytes::from_array(env, &[0u8; 16]);
        bytes.append(&lo);
        Self::from_be_bytes(env, &bytes)
    }

    pub fn from_parts(env: &Env, hi_hi: u64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> Self {
        let obj = env
            .obj_from_u256_pieces(hi_hi, hi_lo, lo_hi, lo_lo)
            .unwrap_infallible();
        U256 {
            env: env.clone(),
            val: obj.into(),
        }
    }

    pub fn from_be_bytes(env: &Env, bytes: &Bytes) -> Self {
        let val = env
            .u256_val_from_be_bytes(bytes.to_object())
            .unwrap_infallible();
        U256 {
            env: env.clone(),
            val,
        }
    }

    pub fn to_u128(&self) -> Option<u128> {
        let be_bytes = self.to_be_bytes();
        let be_bytes_hi: [u8; 16] = be_bytes.slice(0..16).try_into().unwrap();
        let be_bytes_lo: [u8; 16] = be_bytes.slice(16..32).try_into().unwrap();
        if u128::from_be_bytes(be_bytes_hi) == 0 {
            Some(u128::from_be_bytes(be_bytes_lo))
        } else {
            None
        }
    }

    pub fn to_be_bytes(&self) -> Bytes {
        let obj = self
            .env
            .u256_val_to_be_bytes(self.to_val_type())
            .unwrap_infallible();
        unsafe { Bytes::unchecked_new(self.env.clone(), obj) }
    }

    pub fn add(&self, other: &U256) -> U256 {
        let val = self.env.u256_add(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn sub(&self, other: &U256) -> U256 {
        let val = self.env.u256_sub(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn mul(&self, other: &U256) -> U256 {
        let val = self.env.u256_mul(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn div(&self, other: &U256) -> U256 {
        let val = self.env.u256_div(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn rem_euclid(&self, other: &U256) -> U256 {
        let val = self
            .env
            .u256_rem_euclid(self.val, other.val)
            .unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn pow(&self, pow: u32) -> U256 {
        let val = self.env.u256_pow(self.val, pow.into()).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn shl(&self, bits: u32) -> U256 {
        let val = self.env.u256_shl(self.val, bits.into()).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn shr(&self, bits: u32) -> U256 {
        let val = self.env.u256_shr(self.val, bits.into()).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    /// Checked integer addition. Computes `self + other`, returning `None` if
    /// overflow occurred.
    pub fn checked_add(&self, other: &U256) -> Option<U256> {
        // Overflow occurs if max - self < other
        let remaining = U256::max_value(self.env()).sub(self);
        if remaining >= *other {
            Some(self.add(other))
        } else {
            None
        }
    }

    /// Checked integer subtraction. Computes `self - other`, returning `None`
    /// if underflow occurred.
    pub fn checked_sub(&self, other: &U256) -> Option<U256> {
        if self >= other {
            Some(self.sub(other))
        } else {
            None
        }
    }

    /// Checked integer multiplication. Computes `self * other`, returning
    /// `None` if overflow occurred.
    pub fn checked_mul(&self, other: &U256) -> Option<U256> {
        let zero = U256::zero(self.env());
        if *other == zero {
            Some(zero)
        } else {
            // Overflow occurs if self > max / other
            let max_quotient = U256::max_value(self.env()).div(other);
            if *self <= max_quotient {
                Some(self.mul(other))
            } else {
                None
            }
        }
    }

    /// Checked integer division. Computes `self / other`, returning `None` if
    /// `other == 0`.
    pub fn checked_div(&self, other: &U256) -> Option<U256> {
        let zero = U256::zero(self.env());
        if *other == zero {
            None
        } else {
            Some(self.div(other))
        }
    }

    /// Checked Euclidean remainder. Computes `self.rem_euclid(other)`,
    /// returning `None` if `other == 0`.
    pub fn checked_rem_euclid(&self, other: &U256) -> Option<U256> {
        let zero = U256::zero(self.env());
        if *other == zero {
            None
        } else {
            Some(self.rem_euclid(other))
        }
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if
    /// overflow occurred.
    pub fn checked_pow(&self, exp: u32) -> Option<U256> {
        if exp == 0 {
            return Some(U256::from_u32(self.env(), 1));
        }

        let zero = U256::zero(self.env());
        let one = U256::from_u32(self.env(), 1);

        if *self == zero {
            return Some(zero);
        }
        if *self == one {
            return Some(one);
        }

        // Use iterative approach to check for overflow
        let mut base = self.clone();
        let mut exp = exp;
        let mut result = U256::from_u32(self.env(), 1);

        loop {
            if exp % 2 == 1 {
                result = result.checked_mul(&base)?;
            }
            exp /= 2;
            if exp == 0 {
                break;
            }
            base = base.checked_mul(&base)?;
        }

        Some(result)
    }

    /// Checked shift left. Computes `self << bits`, returning `None` if `bits`
    /// is greater than or equal to 256 or if the shift would overflow.
    pub fn checked_shl(&self, bits: u32) -> Option<U256> {
        if bits >= 256 {
            return None;
        }

        // Check for overflow: value must be recoverable by right shift
        let shifted = self.shl(bits);
        if shifted.shr(bits) == *self {
            Some(shifted)
        } else {
            None
        }
    }
}

/// I256 holds a 256-bit signed integer.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{I256, Env};
///
/// let env = Env::default();
///
/// let i1 = I256::from_i32(&env, -6);
/// let i2 = I256::from_i32(&env, 3);
/// assert_eq!(i1.add(&i2), I256::from_i32(&env, -3));
/// ```
#[derive(Clone)]
pub struct I256 {
    env: Env,
    val: I256Val,
}

impl_num_wrapping_val_type!(I256, I256Val, I256Small);

impl I256 {
    /// Returns zero.
    pub fn zero(env: &Env) -> I256 {
        I256::from_i32(env, 0)
    }

    /// Returns the minimum value for I256 (-2^255).
    pub fn min_value(env: &Env) -> I256 {
        I256::from_parts(env, i64::MIN, 0, 0, 0)
    }

    /// Returns the maximum value for I256 (2^255 - 1).
    pub fn max_value(env: &Env) -> I256 {
        I256::from_parts(env, i64::MAX, u64::MAX, u64::MAX, u64::MAX)
    }

    pub fn from_i32(env: &Env, i: i32) -> Self {
        I256 {
            env: env.clone(),
            val: I256Val::from_i32(i),
        }
    }

    pub fn from_i128(env: &Env, i: i128) -> Self {
        let lo: Bytes = Bytes::from_array(env, &i.to_be_bytes());
        if i < 0 {
            let mut i256_bytes: Bytes = Bytes::from_array(env, &[255_u8; 16]);
            i256_bytes.append(&lo);
            Self::from_be_bytes(env, &i256_bytes)
        } else {
            let mut i256_bytes: Bytes = Bytes::from_array(env, &[0_u8; 16]);
            i256_bytes.append(&lo);
            Self::from_be_bytes(env, &i256_bytes)
        }
    }

    pub fn from_parts(env: &Env, hi_hi: i64, hi_lo: u64, lo_hi: u64, lo_lo: u64) -> Self {
        let obj = env
            .obj_from_i256_pieces(hi_hi, hi_lo, lo_hi, lo_lo)
            .unwrap_infallible();
        I256 {
            env: env.clone(),
            val: obj.into(),
        }
    }

    pub fn from_be_bytes(env: &Env, bytes: &Bytes) -> Self {
        let val = env
            .i256_val_from_be_bytes(bytes.to_object())
            .unwrap_infallible();
        I256 {
            env: env.clone(),
            val,
        }
    }

    pub fn to_i128(&self) -> Option<i128> {
        let be_bytes = self.to_be_bytes();
        let be_bytes_hi: [u8; 16] = be_bytes.slice(0..16).try_into().unwrap();
        let be_bytes_lo: [u8; 16] = be_bytes.slice(16..32).try_into().unwrap();
        let i128_hi = i128::from_be_bytes(be_bytes_hi);
        let i128_lo = i128::from_be_bytes(be_bytes_lo);
        if (i128_hi == 0 && i128_lo >= 0) || (i128_hi == -1 && i128_lo < 0) {
            Some(i128_lo)
        } else {
            None
        }
    }

    pub fn to_be_bytes(&self) -> Bytes {
        let obj = self
            .env
            .i256_val_to_be_bytes(self.to_val_type())
            .unwrap_infallible();
        unsafe { Bytes::unchecked_new(self.env.clone(), obj) }
    }

    pub fn add(&self, other: &I256) -> I256 {
        let val = self.env.i256_add(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn sub(&self, other: &I256) -> I256 {
        let val = self.env.i256_sub(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn mul(&self, other: &I256) -> I256 {
        let val = self.env.i256_mul(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn div(&self, other: &I256) -> I256 {
        let val = self.env.i256_div(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn rem_euclid(&self, other: &I256) -> I256 {
        let val = self
            .env
            .i256_rem_euclid(self.val, other.val)
            .unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn pow(&self, pow: u32) -> I256 {
        let val = self.env.i256_pow(self.val, pow.into()).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn shl(&self, bits: u32) -> I256 {
        let val = self.env.i256_shl(self.val, bits.into()).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn shr(&self, bits: u32) -> I256 {
        let val = self.env.i256_shr(self.val, bits.into()).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    /// Checked integer addition. Computes `self + other`, returning `None` if
    /// overflow or underflow occurred.
    pub fn checked_add(&self, other: &I256) -> Option<I256> {
        let zero = I256::zero(self.env());

        if *other >= zero {
            // Adding a non-negative number: check for overflow
            // Overflow if self > max - other
            let max = I256::max_value(self.env());
            let remaining = max.sub(other);
            if *self <= remaining {
                Some(self.add(other))
            } else {
                None
            }
        } else {
            // Adding a negative number: check for underflow
            // Underflow if self < min - other (but other is negative, so min - other > min)
            let min = I256::min_value(self.env());
            let remaining = min.sub(other);
            if *self >= remaining {
                Some(self.add(other))
            } else {
                None
            }
        }
    }

    /// Checked integer subtraction. Computes `self - other`, returning `None`
    /// if overflow or underflow occurred.
    pub fn checked_sub(&self, other: &I256) -> Option<I256> {
        let zero = I256::zero(self.env());

        if *other >= zero {
            // Subtracting a non-negative number: check for underflow
            // Underflow if self < min + other
            let min = I256::min_value(self.env());
            let limit = min.add(other);
            if *self >= limit {
                Some(self.sub(other))
            } else {
                None
            }
        } else {
            // Subtracting a negative number (adding positive): check for overflow
            // Overflow if self > max + other (but other is negative, so max + other < max)
            let max = I256::max_value(self.env());
            let limit = max.add(other);
            if *self <= limit {
                Some(self.sub(other))
            } else {
                None
            }
        }
    }

    /// Checked integer multiplication. Computes `self * other`, returning
    /// `None` if overflow or underflow occurred.
    pub fn checked_mul(&self, other: &I256) -> Option<I256> {
        let zero = I256::zero(self.env());
        let one = I256::from_i32(self.env(), 1);
        let neg_one = I256::from_i32(self.env(), -1);

        if *self == zero || *other == zero {
            return Some(zero);
        }

        if *self == one {
            return Some(other.clone());
        }
        if *other == one {
            return Some(self.clone());
        }

        let min = I256::min_value(self.env());
        let max = I256::max_value(self.env());

        // Check for the special case of MIN * -1 which would overflow
        if (*self == min && *other == neg_one) || (*self == neg_one && *other == min) {
            return None;
        }

        // Handle -1 cases
        if *self == neg_one {
            return Some(zero.sub(other));
        }
        if *other == neg_one {
            return Some(zero.sub(self));
        }

        // Determine signs
        let self_neg = *self < zero;
        let other_neg = *other < zero;
        let result_neg = self_neg != other_neg;

        // Handle cases involving MIN specially since abs(MIN) overflows
        if *self == min || *other == min {
            // MIN * anything with |anything| > 1 will overflow
            // We already handled MIN * 1, MIN * 0, MIN * -1 above
            return None;
        }

        // Get absolute values for comparison (safe now since neither is MIN)
        let self_abs = if self_neg {
            zero.sub(self)
        } else {
            self.clone()
        };
        let other_abs = if other_neg {
            zero.sub(other)
        } else {
            other.clone()
        };

        // Check for overflow using absolute values
        // For negative result, the limit is abs(min) = max + 1, but we can only represent max
        // So we check: self_abs * other_abs <= max (for positive result)
        // Or: self_abs * other_abs <= max + 1 (for negative result), which we approximate as <= max
        // since if self_abs * other_abs == max + 1, result is exactly MIN which is valid

        // Overflow if self_abs > max / other_abs
        let max_self = max.div(&other_abs);
        if self_abs <= max_self {
            Some(self.mul(other))
        } else if result_neg && self_abs == max_self.add(&one) {
            // Special case: result is exactly MIN
            let result = self.mul(other);
            if result == min {
                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Checked integer division. Computes `self / other`, returning `None` if
    /// `other == 0` or if `self == MIN && other == -1` (overflow case).
    pub fn checked_div(&self, other: &I256) -> Option<I256> {
        let zero = I256::zero(self.env());
        let neg_one = I256::from_i32(self.env(), -1);
        let min = I256::min_value(self.env());

        if *other == zero {
            return None;
        }

        // Check for MIN / -1 overflow
        if *self == min && *other == neg_one {
            return None;
        }

        Some(self.div(other))
    }

    /// Checked Euclidean remainder. Computes `self.rem_euclid(other)`,
    /// returning `None` if `other == 0`.
    pub fn checked_rem_euclid(&self, other: &I256) -> Option<I256> {
        let zero = I256::zero(self.env());
        if *other == zero {
            None
        } else {
            Some(self.rem_euclid(other))
        }
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if
    /// overflow or underflow occurred.
    pub fn checked_pow(&self, exp: u32) -> Option<I256> {
        if exp == 0 {
            return Some(I256::from_i32(self.env(), 1));
        }

        let zero = I256::zero(self.env());
        let one = I256::from_i32(self.env(), 1);
        let neg_one = I256::from_i32(self.env(), -1);

        if *self == zero {
            return Some(zero);
        }
        if *self == one {
            return Some(one);
        }
        if *self == neg_one {
            // (-1)^exp = 1 if exp is even, -1 if exp is odd
            return if exp % 2 == 0 {
                Some(one)
            } else {
                Some(neg_one)
            };
        }

        // Use iterative approach to check for overflow
        let mut base = self.clone();
        let mut exp = exp;
        let mut result = I256::from_i32(self.env(), 1);

        loop {
            if exp % 2 == 1 {
                result = result.checked_mul(&base)?;
            }
            exp /= 2;
            if exp == 0 {
                break;
            }
            base = base.checked_mul(&base)?;
        }

        Some(result)
    }

    /// Checked shift left. Computes `self << bits`, returning `None` if `bits`
    /// is greater than or equal to 256 or if the shift would overflow.
    pub fn checked_shl(&self, bits: u32) -> Option<I256> {
        if bits >= 256 {
            return None;
        }

        // Check for overflow: value must be recoverable by right shift
        let shifted = self.shl(bits);
        if shifted.shr(bits) == *self {
            Some(shifted)
        } else {
            None
        }
    }
}

#[doc = "Timepoint holds a 64-bit unsigned integer."]
#[derive(Clone)]
pub struct Timepoint {
    env: Env,
    val: TimepointVal,
}

impl_num_wrapping_val_type!(Timepoint, TimepointVal, TimepointSmall);

impl Timepoint {
    /// Create a Timepoint from a unix time in seconds, the time in seconds
    /// since January 1, 1970 UTC.
    pub fn from_unix(env: &Env, seconds: u64) -> Timepoint {
        let val = TimepointVal::try_from_val(env, &seconds).unwrap_optimized();
        Timepoint {
            env: env.clone(),
            val,
        }
    }

    /// Returns the Timepoint as unix time in seconds, the time in seconds since
    /// January 1, 1970 UTC.
    pub fn to_unix(&self) -> u64 {
        u64::try_from_val(self.env(), &self.to_val_type()).unwrap_optimized()
    }
}

#[doc = "Duration holds a 64-bit unsigned integer."]
#[derive(Clone)]
pub struct Duration {
    env: Env,
    val: DurationVal,
}

impl_num_wrapping_val_type!(Duration, DurationVal, DurationSmall);

impl Duration {
    /// Create a Duration from seconds.
    pub fn from_seconds(env: &Env, seconds: u64) -> Duration {
        let val = DurationVal::try_from_val(env, &seconds).unwrap_optimized();
        Duration {
            env: env.clone(),
            val,
        }
    }

    /// Returns the Duration as seconds.
    pub fn to_seconds(&self) -> u64 {
        u64::try_from_val(self.env(), &self.to_val_type()).unwrap_optimized()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u256_roundtrip() {
        let env = Env::default();

        let u1 = U256::from_u32(&env, 12345);
        let bytes = u1.to_be_bytes();
        let u2 = U256::from_be_bytes(&env, &bytes);
        assert_eq!(u1, u2);
    }

    #[test]
    fn test_u256_u128_conversion() {
        let env = Env::default();

        // positive
        let start = u128::MAX / 7;
        let from = U256::from_u128(&env, start);
        let end = from.to_u128().unwrap();
        assert_eq!(start, end);

        let over_u128 = from.mul(&U256::from_u32(&env, 8));
        let failure = over_u128.to_u128();
        assert_eq!(failure, None);

        // zero
        let start = 0_u128;
        let from = U256::from_u128(&env, start);
        let end = from.to_u128().unwrap();
        assert_eq!(start, end);
    }

    #[test]
    fn test_i256_roundtrip() {
        let env = Env::default();

        let i1 = I256::from_i32(&env, -12345);
        let bytes = i1.to_be_bytes();
        let i2 = I256::from_be_bytes(&env, &bytes);
        assert_eq!(i1, i2);
    }

    #[test]
    fn test_i256_i128_conversion() {
        let env = Env::default();

        // positive
        let start = i128::MAX / 7;
        let from = I256::from_i128(&env, start);
        let end = from.to_i128().unwrap();
        assert_eq!(start, end);

        let over_i128 = from.mul(&I256::from_i32(&env, 8));
        let failure = over_i128.to_i128();
        assert_eq!(failure, None);

        // negative
        let start = i128::MIN / 7;
        let from = I256::from_i128(&env, start);
        let end = from.to_i128().unwrap();
        assert_eq!(start, end);

        let over_i128 = from.mul(&I256::from_i32(&env, 8));
        let failure = over_i128.to_i128();
        assert_eq!(failure, None);

        // zero
        let start = 0_i128;
        let from = I256::from_i128(&env, start);
        let end = from.to_i128().unwrap();
        assert_eq!(start, end);
    }

    #[test]
    fn test_timepoint_roundtrip() {
        let env = Env::default();

        let tp = Timepoint::from_unix(&env, 123);
        let u = tp.to_unix();
        assert_eq!(u, 123);
    }

    #[test]
    fn test_duration_roundtrip() {
        let env = Env::default();

        let tp = Duration::from_seconds(&env, 123);
        let u = tp.to_seconds();
        assert_eq!(u, 123);
    }

    #[test]
    fn test_u256_arith() {
        let env = Env::default();

        let u1 = U256::from_u32(&env, 6);
        let u2 = U256::from_u32(&env, 3);
        assert_eq!(u1.add(&u2), U256::from_u32(&env, 9));
        assert_eq!(u1.sub(&u2), U256::from_u32(&env, 3));
        assert_eq!(u1.mul(&u2), U256::from_u32(&env, 18));
        assert_eq!(u1.div(&u2), U256::from_u32(&env, 2));
        assert_eq!(u1.pow(2), U256::from_u32(&env, 36));
        assert_eq!(u1.shl(2), U256::from_u32(&env, 24));
        assert_eq!(u1.shr(1), U256::from_u32(&env, 3));

        let u3 = U256::from_u32(&env, 7);
        let u4 = U256::from_u32(&env, 4);
        assert_eq!(u3.rem_euclid(&u4), U256::from_u32(&env, 3));
    }

    #[test]
    fn test_i256_arith() {
        let env = Env::default();

        let i1 = I256::from_i32(&env, -6);
        let i2 = I256::from_i32(&env, 3);
        assert_eq!(i1.add(&i2), I256::from_i32(&env, -3));
        assert_eq!(i1.sub(&i2), I256::from_i32(&env, -9));
        assert_eq!(i1.mul(&i2), I256::from_i32(&env, -18));
        assert_eq!(i1.div(&i2), I256::from_i32(&env, -2));
        assert_eq!(i1.pow(2), I256::from_i32(&env, 36));
        assert_eq!(i1.shl(2), I256::from_i32(&env, -24));
        assert_eq!(i1.shr(1), I256::from_i32(&env, -3));

        let u3 = I256::from_i32(&env, -7);
        let u4 = I256::from_i32(&env, 4);
        assert_eq!(u3.rem_euclid(&u4), I256::from_i32(&env, 1));
    }

    #[test]
    fn test_u256_min_max_value() {
        let env = Env::default();

        let zero = U256::zero(&env);
        let min = U256::min_value(&env);
        let max = U256::max_value(&env);

        // zero should be 0
        assert_eq!(zero, U256::from_u32(&env, 0));

        // min should be 0
        assert_eq!(min, U256::from_u32(&env, 0));
        assert_eq!(min, zero);

        // max should be 2^256 - 1 (all bits set)
        assert_eq!(
            max,
            U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX)
        );

        // max > min
        assert!(max > min);
    }

    #[test]
    fn test_i256_min_max_value() {
        let env = Env::default();

        let zero = I256::zero(&env);
        let min = I256::min_value(&env);
        let max = I256::max_value(&env);

        // zero should be 0
        assert_eq!(zero, I256::from_i32(&env, 0));

        // max should be 2^255 - 1
        assert_eq!(
            max,
            I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX)
        );

        // min should be -2^255
        assert_eq!(min, I256::from_parts(&env, i64::MIN, 0, 0, 0));

        // max > 0 and min < 0
        assert!(max > zero);
        assert!(min < zero);

        // max > min
        assert!(max > min);
    }

    #[test]
    fn test_u256_checked_add() {
        let env = Env::default();

        // Normal case
        let u1 = U256::from_u32(&env, 6);
        let u2 = U256::from_u32(&env, 3);
        assert_eq!(u1.checked_add(&u2), Some(U256::from_u32(&env, 9)));

        // Overflow case
        let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let one = U256::from_u32(&env, 1);
        assert_eq!(max.checked_add(&one), None);

        // Adding to max-1 should work
        let max_minus_one = max.sub(&one);
        assert_eq!(max_minus_one.checked_add(&one), Some(max.clone()));
    }

    #[test]
    fn test_u256_checked_sub() {
        let env = Env::default();

        // Normal case
        let u1 = U256::from_u32(&env, 6);
        let u2 = U256::from_u32(&env, 3);
        assert_eq!(u1.checked_sub(&u2), Some(U256::from_u32(&env, 3)));

        // Underflow case
        assert_eq!(u2.checked_sub(&u1), None);

        // Subtracting zero should work
        let zero = U256::from_u32(&env, 0);
        assert_eq!(u1.checked_sub(&zero), Some(u1.clone()));

        // Subtracting from zero should fail for non-zero
        assert_eq!(zero.checked_sub(&u1), None);
    }

    #[test]
    fn test_u256_checked_mul() {
        let env = Env::default();

        // Normal case
        let u1 = U256::from_u32(&env, 6);
        let u2 = U256::from_u32(&env, 3);
        assert_eq!(u1.checked_mul(&u2), Some(U256::from_u32(&env, 18)));

        // Multiply by zero
        let zero = U256::from_u32(&env, 0);
        assert_eq!(u1.checked_mul(&zero), Some(zero.clone()));
        assert_eq!(zero.checked_mul(&u1), Some(zero.clone()));

        // Overflow case
        let max = U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let two = U256::from_u32(&env, 2);
        assert_eq!(max.checked_mul(&two), None);
    }

    #[test]
    fn test_u256_checked_div() {
        let env = Env::default();

        // Normal case
        let u1 = U256::from_u32(&env, 6);
        let u2 = U256::from_u32(&env, 3);
        assert_eq!(u1.checked_div(&u2), Some(U256::from_u32(&env, 2)));

        // Divide by zero
        let zero = U256::from_u32(&env, 0);
        assert_eq!(u1.checked_div(&zero), None);

        // Zero divided by non-zero
        assert_eq!(zero.checked_div(&u1), Some(zero.clone()));
    }

    #[test]
    fn test_u256_checked_rem_euclid() {
        let env = Env::default();

        // Normal case
        let u1 = U256::from_u32(&env, 7);
        let u2 = U256::from_u32(&env, 4);
        assert_eq!(u1.checked_rem_euclid(&u2), Some(U256::from_u32(&env, 3)));

        // Mod by zero
        let zero = U256::from_u32(&env, 0);
        assert_eq!(u1.checked_rem_euclid(&zero), None);
    }

    #[test]
    fn test_u256_checked_pow() {
        let env = Env::default();

        // Normal case
        let u = U256::from_u32(&env, 2);
        assert_eq!(u.checked_pow(10), Some(U256::from_u32(&env, 1024)));

        // Power of zero
        assert_eq!(u.checked_pow(0), Some(U256::from_u32(&env, 1)));

        // Zero to the power of something
        let zero = U256::from_u32(&env, 0);
        assert_eq!(zero.checked_pow(5), Some(zero.clone()));

        // One to any power
        let one = U256::from_u32(&env, 1);
        assert_eq!(one.checked_pow(100), Some(one.clone()));

        // Overflow case: 2^256 would overflow
        assert_eq!(u.checked_pow(256), None);
    }

    #[test]
    fn test_u256_checked_shl() {
        let env = Env::default();

        // Normal case
        let u = U256::from_u32(&env, 1);
        assert_eq!(u.checked_shl(4), Some(U256::from_u32(&env, 16)));

        // Shift by 0
        assert_eq!(u.checked_shl(0), Some(u.clone()));

        // Shift >= 256 returns None
        assert_eq!(u.checked_shl(256), None);
        assert_eq!(u.checked_shl(300), None);

        // Overflow case: shifting a value with high bit set
        let high_bit = U256::from_parts(&env, 1u64 << 63, 0, 0, 0); // Bit 255 set
        assert_eq!(high_bit.checked_shl(1), None); // Would lose the high bit
    }

    #[test]
    fn test_i256_checked_add() {
        let env = Env::default();

        // Normal case (positive + positive)
        let i1 = I256::from_i32(&env, 6);
        let i2 = I256::from_i32(&env, 3);
        assert_eq!(i1.checked_add(&i2), Some(I256::from_i32(&env, 9)));

        // Normal case (negative + negative)
        let i3 = I256::from_i32(&env, -6);
        let i4 = I256::from_i32(&env, -3);
        assert_eq!(i3.checked_add(&i4), Some(I256::from_i32(&env, -9)));

        // Normal case (positive + negative)
        assert_eq!(i1.checked_add(&i4), Some(I256::from_i32(&env, 3)));

        // Overflow case (max + positive)
        let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let one = I256::from_i32(&env, 1);
        assert_eq!(max.checked_add(&one), None);

        // Underflow case (min + negative)
        let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
        let neg_one = I256::from_i32(&env, -1);
        assert_eq!(min.checked_add(&neg_one), None);
    }

    #[test]
    fn test_i256_checked_sub() {
        let env = Env::default();

        // Normal case
        let i1 = I256::from_i32(&env, 6);
        let i2 = I256::from_i32(&env, 3);
        assert_eq!(i1.checked_sub(&i2), Some(I256::from_i32(&env, 3)));

        // Normal case (negative result)
        assert_eq!(i2.checked_sub(&i1), Some(I256::from_i32(&env, -3)));

        // Underflow case (min - positive)
        let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
        let one = I256::from_i32(&env, 1);
        assert_eq!(min.checked_sub(&one), None);

        // Overflow case (max - negative)
        let max = I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX);
        let neg_one = I256::from_i32(&env, -1);
        assert_eq!(max.checked_sub(&neg_one), None);
    }

    #[test]
    fn test_i256_checked_mul() {
        let env = Env::default();

        // Normal case
        let i1 = I256::from_i32(&env, 6);
        let i2 = I256::from_i32(&env, 3);
        assert_eq!(i1.checked_mul(&i2), Some(I256::from_i32(&env, 18)));

        // Normal case (negative * positive)
        let neg_i1 = I256::from_i32(&env, -6);
        assert_eq!(neg_i1.checked_mul(&i2), Some(I256::from_i32(&env, -18)));

        // Normal case (negative * negative)
        let neg_i2 = I256::from_i32(&env, -3);
        assert_eq!(neg_i1.checked_mul(&neg_i2), Some(I256::from_i32(&env, 18)));

        // Multiply by zero
        let zero = I256::from_i32(&env, 0);
        assert_eq!(i1.checked_mul(&zero), Some(zero.clone()));

        // Overflow case (min * -1)
        let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
        let neg_one = I256::from_i32(&env, -1);
        assert_eq!(min.checked_mul(&neg_one), None);
    }

    #[test]
    fn test_i256_checked_div() {
        let env = Env::default();

        // Normal case
        let i1 = I256::from_i32(&env, 6);
        let i2 = I256::from_i32(&env, 3);
        assert_eq!(i1.checked_div(&i2), Some(I256::from_i32(&env, 2)));

        // Normal case (negative / positive)
        let neg_i1 = I256::from_i32(&env, -6);
        assert_eq!(neg_i1.checked_div(&i2), Some(I256::from_i32(&env, -2)));

        // Divide by zero
        let zero = I256::from_i32(&env, 0);
        assert_eq!(i1.checked_div(&zero), None);

        // Overflow case (min / -1)
        let min = I256::from_parts(&env, i64::MIN, 0, 0, 0);
        let neg_one = I256::from_i32(&env, -1);
        assert_eq!(min.checked_div(&neg_one), None);
    }

    #[test]
    fn test_i256_checked_rem_euclid() {
        let env = Env::default();

        // Normal case
        let i1 = I256::from_i32(&env, -7);
        let i2 = I256::from_i32(&env, 4);
        assert_eq!(i1.checked_rem_euclid(&i2), Some(I256::from_i32(&env, 1)));

        // Mod by zero
        let zero = I256::from_i32(&env, 0);
        assert_eq!(i1.checked_rem_euclid(&zero), None);
    }

    #[test]
    fn test_i256_checked_pow() {
        let env = Env::default();

        // Normal case
        let i = I256::from_i32(&env, 2);
        assert_eq!(i.checked_pow(10), Some(I256::from_i32(&env, 1024)));

        // Negative base with even exponent
        let neg_i = I256::from_i32(&env, -2);
        assert_eq!(neg_i.checked_pow(10), Some(I256::from_i32(&env, 1024)));

        // Negative base with odd exponent
        assert_eq!(neg_i.checked_pow(3), Some(I256::from_i32(&env, -8)));

        // Power of zero
        assert_eq!(i.checked_pow(0), Some(I256::from_i32(&env, 1)));

        // -1 to even power
        let neg_one = I256::from_i32(&env, -1);
        assert_eq!(neg_one.checked_pow(100), Some(I256::from_i32(&env, 1)));

        // -1 to odd power
        assert_eq!(neg_one.checked_pow(101), Some(I256::from_i32(&env, -1)));

        // Overflow case
        assert_eq!(i.checked_pow(255), None);
    }

    #[test]
    fn test_i256_checked_shl() {
        let env = Env::default();

        // Normal case (positive)
        let i = I256::from_i32(&env, 1);
        assert_eq!(i.checked_shl(4), Some(I256::from_i32(&env, 16)));

        // Normal case (negative)
        let neg_i = I256::from_i32(&env, -1);
        assert_eq!(neg_i.checked_shl(4), Some(I256::from_i32(&env, -16)));

        // Shift by 0
        assert_eq!(i.checked_shl(0), Some(i.clone()));

        // Shift >= 256 returns None
        assert_eq!(i.checked_shl(256), None);
        assert_eq!(i.checked_shl(300), None);

        // Overflow case for positive: bit 254 set, shifting left would change sign
        let large_pos = I256::from_parts(&env, 1i64 << 62, 0, 0, 0); // Bit 254 set
        assert_eq!(large_pos.checked_shl(1), None); // Would become negative
    }
}
