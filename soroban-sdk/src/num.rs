use core::{cmp::Ordering, convert::Infallible, fmt::Debug};
use std::u64;

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

            /// Converts a `Val` known to be of this type into `Self` without
            /// env-based conversion. The caller must guarantee the `Val` is of
            /// the correct type; only a cheap tag check is performed.
            #[inline(always)]
            pub(crate) unsafe fn unchecked_from_val(env: Env, val: Val) -> Self {
                Self {
                    env,
                    val: <$val>::try_from(val).unwrap_optimized(),
                }
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
    pub const BITS: u32 = 256;

    /// Returns the smallest value that can be represented by this type (0).
    pub fn min_value(env: &Env) -> Self {
        Self::from_parts(env, 0, 0, 0, 0)
    }

    /// Returns the largest value that can be represented by this type (2^256 - 1).
    pub fn max_value(env: &Env) -> Self {
        Self::from_parts(env, u64::MAX, u64::MAX, u64::MAX, u64::MAX)
    }

    fn is_zero(&self) -> bool {
        const ZERO: U256Val = U256Val::from_u32(0);
        self.val.as_val().get_payload() == ZERO.as_val().get_payload()
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

    /// Performs checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(&self, other: &U256) -> Option<U256> {
        let val = self
            .env
            .u256_checked_add(self.val, other.val)
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { U256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(&self, other: &U256) -> Option<U256> {
        let val = self
            .env
            .u256_checked_sub(self.val, other.val)
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { U256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(&self, other: &U256) -> Option<U256> {
        let val = self
            .env
            .u256_checked_mul(self.val, other.val)
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { U256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked exponentiation. Returns `None` if overflow occurred.
    pub fn checked_pow(&self, pow: u32) -> Option<U256> {
        let val = self
            .env
            .u256_checked_pow(self.val, pow.into())
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { U256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked division. Returns `None` if `other` is zero.
    pub fn checked_div(&self, other: &U256) -> Option<U256> {
        if other.is_zero() {
            return None;
        }
        Some(self.div(other))
    }

    /// Performs checked Euclidean remainder. Returns `None` if `other` is zero.
    pub fn checked_rem_euclid(&self, other: &U256) -> Option<U256> {
        if other.is_zero() {
            return None;
        }
        Some(self.rem_euclid(other))
    }

    /// Performs checked left shift. Returns `None` if `bits >= 256`.
    pub fn checked_shl(&self, bits: u32) -> Option<U256> {
        if bits >= Self::BITS {
            return None;
        }
        Some(self.shl(bits))
    }

    /// Performs checked right shift. Returns `None` if `bits >= 256`.
    pub fn checked_shr(&self, bits: u32) -> Option<U256> {
        if bits >= Self::BITS {
            return None;
        }
        Some(self.shr(bits))
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
    pub const BITS: u32 = 256;

    /// Returns the smallest value that can be represented by this type (−2^255).
    pub fn min_value(env: &Env) -> Self {
        Self::from_parts(env, i64::MIN, 0, 0, 0)
    }

    /// Returns the largest value that can be represented by this type (2^255 - 1).
    pub fn max_value(env: &Env) -> Self {
        Self::from_parts(env, i64::MAX, u64::MAX, u64::MAX, u64::MAX)
    }

    fn is_zero(&self) -> bool {
        const ZERO: I256Val = I256Val::from_i32(0);
        self.val.as_val().get_payload() == ZERO.as_val().get_payload()
    }

    fn is_neg_one(&self) -> bool {
        const NEG_ONE: I256Val = I256Val::from_i32(-1);
        self.val.as_val().get_payload() == NEG_ONE.as_val().get_payload()
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

    /// Performs checked addition. Returns `None` if overflow occurred.
    pub fn checked_add(&self, other: &I256) -> Option<I256> {
        let val = self
            .env
            .i256_checked_add(self.val, other.val)
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { I256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked subtraction. Returns `None` if overflow occurred.
    pub fn checked_sub(&self, other: &I256) -> Option<I256> {
        let val = self
            .env
            .i256_checked_sub(self.val, other.val)
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { I256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked multiplication. Returns `None` if overflow occurred.
    pub fn checked_mul(&self, other: &I256) -> Option<I256> {
        let val = self
            .env
            .i256_checked_mul(self.val, other.val)
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { I256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Performs checked exponentiation. Returns `None` if overflow occurred.
    pub fn checked_pow(&self, pow: u32) -> Option<I256> {
        let val = self
            .env
            .i256_checked_pow(self.val, pow.into())
            .unwrap_infallible();
        if val.is_void() {
            None
        } else {
            Some(unsafe { I256::unchecked_from_val(self.env.clone(), val) })
        }
    }

    /// Returns `true` if dividing `self` by `other` would overflow or divide by zero.
    /// This covers: `other == 0`, or `self == I256::MIN && other == -1`.
    fn is_div_overflow(&self, other: &I256) -> bool {
        if other.is_zero() {
            return true;
        }
        if other.is_neg_one() {
            let min = I256::min_value(&self.env);
            if *self == min {
                return true;
            }
        }
        false
    }

    /// Performs checked division. Returns `None` if `other` is zero, or if
    /// `self` is `I256::MIN` and `other` is `-1` (overflow).
    pub fn checked_div(&self, other: &I256) -> Option<I256> {
        if self.is_div_overflow(other) {
            return None;
        }
        Some(self.div(other))
    }

    /// Performs checked Euclidean remainder. Returns `None` if `other` is zero,
    /// or if `self` is `I256::MIN` and `other` is `-1` (overflow in intermediate
    /// division).
    pub fn checked_rem_euclid(&self, other: &I256) -> Option<I256> {
        if self.is_div_overflow(other) {
            return None;
        }
        Some(self.rem_euclid(other))
    }

    /// Performs checked left shift. Returns `None` if `bits >= 256`.
    pub fn checked_shl(&self, bits: u32) -> Option<I256> {
        if bits >= Self::BITS {
            return None;
        }
        Some(self.shl(bits))
    }

    /// Performs checked right shift. Returns `None` if `bits >= 256`.
    pub fn checked_shr(&self, bits: u32) -> Option<I256> {
        if bits >= Self::BITS {
            return None;
        }
        Some(self.shr(bits))
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
    fn test_u256_min() {
        let env = Env::default();

        let min = U256::min_value(&env);
        assert_eq!(min, U256::from_u32(&env, 0));

        let one = U256::from_u32(&env, 1);
        assert_eq!(min.checked_sub(&one), None);
        assert!(min.checked_add(&one).is_some());
    }

    #[test]
    fn test_u256_max() {
        let env = Env::default();

        let max = U256::max_value(&env);
        assert_eq!(
            max,
            U256::from_parts(&env, u64::MAX, u64::MAX, u64::MAX, u64::MAX)
        );

        let u128_max = U256::from_u128(&env, u128::MAX);
        assert!(max > u128_max);

        let one = U256::from_u32(&env, 1);
        assert_eq!(max.checked_add(&one), None);
        assert!(max.checked_sub(&one).is_some());
    }

    #[test]
    fn test_i256_min() {
        let env = Env::default();

        let min = I256::min_value(&env);
        assert_eq!(min, I256::from_parts(&env, i64::MIN, 0, 0, 0));

        let i128_min = I256::from_i128(&env, i128::MIN);
        assert!(min < i128_min);

        let one = I256::from_i32(&env, 1);
        assert_eq!(min.checked_sub(&one), None);
        assert!(min.checked_add(&one).is_some());
    }

    #[test]
    fn test_i256_max() {
        let env = Env::default();

        let max = I256::max_value(&env);
        assert_eq!(
            max,
            I256::from_parts(&env, i64::MAX, u64::MAX, u64::MAX, u64::MAX)
        );

        let i128_max = I256::from_i128(&env, i128::MAX);
        assert!(max > i128_max);

        let one = I256::from_i32(&env, 1);
        assert_eq!(max.checked_add(&one), None);
        assert!(max.checked_sub(&one).is_some());
    }
}
