use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use super::{
    xdr::ScObjectType, Env, EnvBase, EnvObj, EnvTrait, EnvVal, EnvValConvertible, OrAbort, RawVal,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct BigInt(EnvObj);

impl TryFrom<EnvVal<RawVal>> for BigInt {
    type Error = ();

    fn try_from(ev: EnvVal<RawVal>) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.clone().try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for BigInt {
    type Error = ();

    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Bigint) {
            Ok(BigInt(obj))
        } else {
            Err(())
        }
    }
}

impl From<BigInt> for RawVal {
    fn from(b: BigInt) -> Self {
        b.0.into()
    }
}

impl From<BigInt> for EnvVal<RawVal> {
    fn from(b: BigInt) -> Self {
        b.0.into()
    }
}

impl From<BigInt> for EnvObj {
    fn from(b: BigInt) -> Self {
        b.0
    }
}

impl TryFrom<BigInt> for u64 {
    type Error = ();

    fn try_from(b: BigInt) -> Result<Self, Self::Error> {
        if b.bits() <= u64::BITS {
            Ok(unsafe { b.to_u64() })
        } else {
            Err(())
        }
    }
}

impl TryFrom<BigInt> for i64 {
    type Error = ();

    fn try_from(b: BigInt) -> Result<Self, Self::Error> {
        if b.bits() <= i64::BITS {
            Ok(unsafe { b.to_i64() })
        } else {
            Err(())
        }
    }
}

impl TryFrom<BigInt> for u32 {
    type Error = ();

    fn try_from(b: BigInt) -> Result<Self, Self::Error> {
        if b.bits() <= u32::BITS {
            Ok(unsafe { b.to_u32() })
        } else {
            Err(())
        }
    }
}

impl TryFrom<BigInt> for i32 {
    type Error = ();

    fn try_from(b: BigInt) -> Result<Self, Self::Error> {
        if b.bits() <= i32::BITS {
            Ok(unsafe { b.to_i32() })
        } else {
            Err(())
        }
    }
}

impl Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_add(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_sub(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Mul for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_mul(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Div for BigInt {
    type Output = BigInt;
    fn div(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_div(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Rem for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_rem(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl BitAnd for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_and(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl BitOr for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_or(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl BitXor for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_xor(self.0.to_tagged(), rhs.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        let env = self.env();
        let b = env.bigint_neg(self.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Not for BigInt {
    type Output = BigInt;
    fn not(self) -> Self::Output {
        let env = self.env();
        let b = env.bigint_not(self.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Shl<i32> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: i32) -> Self::Output {
        let env = self.env();
        let b = env.bigint_shl(self.0.to_tagged(), rhs.into());
        Self::try_from_val(env, b).or_abort()
    }
}

impl Shr<i32> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: i32) -> Self::Output {
        let env = self.env();
        let b = env.bigint_shl(self.0.to_tagged(), rhs.into());
        Self::try_from_val(env, b).or_abort()
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Eq for BigInt {}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let env = self.env();
        let v = env.bigint_cmp(self.0.to_tagged(), other.0.to_tagged());
        let i = i32::try_from(v).or_abort();
        if i < 0 {
            Ordering::Less
        } else if i > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl BigInt {
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn from_u64(env: &Env, u: u64) -> BigInt {
        let obj = env.bigint_from_u64(u).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    unsafe fn to_u64(&self) -> u64 {
        let env = self.env();
        env.bigint_to_u64(self.0.to_tagged())
    }

    pub fn from_i64(env: &Env, i: i64) -> BigInt {
        let obj = env.bigint_from_i64(i).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    unsafe fn to_i64(&self) -> i64 {
        let env = self.env();
        env.bigint_to_i64(self.0.to_tagged())
    }

    pub fn from_u32(env: &Env, u: u32) -> BigInt {
        let obj = env.bigint_from_u64(u as u64).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    unsafe fn to_u32(&self) -> u32 {
        let env = self.env();
        let u = env.bigint_to_u64(self.0.to_tagged());
        u.try_into().or_abort()
    }

    pub fn from_i32(env: &Env, i: i32) -> BigInt {
        let obj = env.bigint_from_i64(i as i64).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    unsafe fn to_i32(&self) -> i32 {
        let env = self.env();
        let i = env.bigint_to_i64(self.0.to_tagged());
        i.try_into().or_abort()
    }

    pub fn gcd(&self, other: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_gcd(self.0.to_tagged(), other.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }

    pub fn lcm(&self, other: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_lcm(self.0.to_tagged(), other.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }

    pub fn pow(&self, k: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_pow(self.0.to_tagged(), k.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }

    pub fn pow_mod(&self, q: BigInt, m: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_pow_mod(self.0.to_tagged(), q.0.to_tagged(), m.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }

    pub fn sqrt(&self) -> BigInt {
        let env = self.env();
        let b = env.bigint_sqrt(self.0.to_tagged());
        Self::try_from_val(env, b).or_abort()
    }

    pub fn is_zero(&self) -> bool {
        let env = self.env();
        let is_zero = env.bigint_is_zero(self.0.to_tagged());
        bool::try_from(is_zero).or_abort()
    }

    pub fn bits(&self) -> u32 {
        let env = self.env();
        let bits = env.bigint_bits(self.0.to_tagged());
        u32::try_from(bits).or_abort()
    }
}
