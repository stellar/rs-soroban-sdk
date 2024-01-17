use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{
        DurationSmall, DurationVal, Env as _, EnvBase as _, I256Small, I256Val, TimepointSmall,
        TimepointVal, U256Small, U256Val,
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
                        self.env.check_same_env(&other.env).unwrap_infallible();
                        let v = self.env.obj_cmp(self_raw, other_raw).unwrap_infallible();
                        v.cmp(&0)
                    }
                }
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
        impl TryFrom<&$wrapper> for ScVal {
            type Error = ConversionError;
            fn try_from(v: &$wrapper) -> Result<Self, ConversionError> {
                if let Ok(ss) = <$small>::try_from(v.val) {
                    ScVal::try_from(ss)
                } else {
                    ScVal::try_from_val(&v.env, &v.to_val())
                }
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl TryFrom<$wrapper> for ScVal {
            type Error = ConversionError;
            fn try_from(v: $wrapper) -> Result<Self, ConversionError> {
                (&v).try_into()
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
        env.check_same_env(bytes.env()).unwrap_infallible();
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
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.u256_add(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn sub(&self, other: &U256) -> U256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.u256_sub(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn mul(&self, other: &U256) -> U256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.u256_mul(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn div(&self, other: &U256) -> U256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.u256_div(self.val, other.val).unwrap_infallible();
        U256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn rem_euclid(&self, other: &U256) -> U256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
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
        env.check_same_env(bytes.env()).unwrap_infallible();
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
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.i256_add(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn sub(&self, other: &I256) -> I256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.i256_sub(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn mul(&self, other: &I256) -> I256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.i256_mul(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn div(&self, other: &I256) -> I256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let val = self.env.i256_div(self.val, other.val).unwrap_infallible();
        I256 {
            env: self.env.clone(),
            val,
        }
    }

    pub fn rem_euclid(&self, other: &I256) -> I256 {
        self.env.check_same_env(&other.env).unwrap_infallible();
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
}
