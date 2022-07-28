use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use super::{
    env::internal::{Env as _, EnvBase, RawValConvertible},
    env::{EnvObj, EnvType},
    xdr::ScObjectType,
    Binary, ConversionError, Env, EnvVal, IntoVal, RawVal, TryFromVal, TryIntoVal,
};

/// BigInt is an arbitrary sized signed integer.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{BigInt, Env};
///
/// # fn main() {
/// let env = Env::default();
/// let b1 = BigInt::from_u64(&env, u64::MAX);
/// let b2 = b1 * 3;
/// let b3 = b2 / 4;
/// assert_eq!(b3, 13_835_058_055_282_163_711u64)
/// # }
/// ```
#[repr(transparent)]
#[derive(Clone)]
pub struct BigInt(EnvObj);

impl Debug for BigInt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BigInt(")?;
        Display::fmt(&self, f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let env = self.env();
        let bi = self.0.to_object();
        let obj: EnvObj = env.bigint_to_radix_be(bi, 10u32.into()).in_env(env);
        if let Ok(bin) = TryInto::<Binary>::try_into(obj) {
            let sign = env.bigint_cmp(bi, env.bigint_from_u64(0));
            if let -1 = unsafe { <i32 as RawValConvertible>::unchecked_from_val(sign) } {
                write!(f, "-")?;
            }
            for x in bin.iter() {
                write!(f, "{:?}", x)?;
            }
        }
        Ok(())
    }
}

impl TryFrom<EnvVal> for BigInt {
    type Error = ConversionError;

    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.clone().try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for BigInt {
    type Error = ConversionError;

    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_object().is_obj_type(ScObjectType::BigInt) {
            Ok(BigInt(obj))
        } else {
            Err(ConversionError {})
        }
    }
}

impl TryIntoVal<Env, BigInt> for RawVal {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<BigInt, Self::Error> {
        EnvType {
            env: env.clone(),
            val: self,
        }
        .try_into()
    }
}

impl From<BigInt> for RawVal {
    fn from(b: BigInt) -> Self {
        b.0.into()
    }
}

impl From<BigInt> for EnvVal {
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

impl From<EnvType<u64>> for BigInt {
    fn from(ev: EnvType<u64>) -> Self {
        BigInt::from_u64(&ev.env, ev.val)
    }
}

impl IntoVal<Env, BigInt> for u64 {
    fn into_val(self, env: &Env) -> BigInt {
        BigInt::from_u64(env, self)
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

impl From<EnvType<i64>> for BigInt {
    fn from(ev: EnvType<i64>) -> Self {
        BigInt::from_i64(&ev.env, ev.val)
    }
}

impl IntoVal<Env, BigInt> for i64 {
    fn into_val(self, env: &Env) -> BigInt {
        BigInt::from_i64(env, self)
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

impl From<EnvType<u32>> for BigInt {
    fn from(ev: EnvType<u32>) -> Self {
        BigInt::from_u32(&ev.env, ev.val)
    }
}

impl IntoVal<Env, BigInt> for u32 {
    fn into_val(self, env: &Env) -> BigInt {
        BigInt::from_u32(env, self)
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

impl From<EnvType<i32>> for BigInt {
    fn from(ev: EnvType<i32>) -> Self {
        BigInt::from_i32(&ev.env, ev.val)
    }
}

impl IntoVal<Env, BigInt> for i32 {
    fn into_val(self, env: &Env) -> BigInt {
        BigInt::from_i32(env, self)
    }
}

#[cfg(not(target_family = "wasm"))]
use super::{env::Object, xdr::ScVal};

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&BigInt> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &BigInt) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<BigInt> for ScVal {
    type Error = ConversionError;
    fn try_from(v: BigInt) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, BigInt> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<BigInt, Self::Error> {
        let o: Object = self.try_into_val(env).map_err(|_| ConversionError)?;
        let env = env.clone();
        EnvObj { val: o, env }.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<EnvType<ScVal>> for BigInt {
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        ScVal::try_into_val(v.val, &v.env)
    }
}

impl Add for BigInt {
    type Output = BigInt;
    fn add(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_add(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Add<u64> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.add(rhs)
    }
}

impl Add<i64> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.add(rhs)
    }
}

impl Add<u32> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.add(rhs)
    }
}

impl Add<i32> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.add(rhs)
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_sub(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Sub<u64> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.sub(rhs)
    }
}

impl Sub<i64> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.sub(rhs)
    }
}

impl Sub<u32> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.sub(rhs)
    }
}

impl Sub<i32> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.sub(rhs)
    }
}

impl Mul for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_mul(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Mul<u64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.mul(rhs)
    }
}

impl Mul<i64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.mul(rhs)
    }
}

impl Mul<u32> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.mul(rhs)
    }
}

impl Mul<i32> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.mul(rhs)
    }
}

impl Div for BigInt {
    type Output = BigInt;
    fn div(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_div(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Div<u64> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.div(rhs)
    }
}

impl Div<i64> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.div(rhs)
    }
}

impl Div<u32> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.div(rhs)
    }
}

impl Div<i32> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.div(rhs)
    }
}

impl Rem for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_rem(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Rem<u64> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.rem(rhs)
    }
}

impl Rem<i64> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.rem(rhs)
    }
}

impl Rem<u32> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.rem(rhs)
    }
}

impl Rem<i32> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.rem(rhs)
    }
}

impl BitAnd for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_and(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl BitAnd<u64> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.bitand(rhs)
    }
}

impl BitAnd<i64> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.bitand(rhs)
    }
}

impl BitAnd<u32> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.bitand(rhs)
    }
}

impl BitAnd<i32> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.bitand(rhs)
    }
}

impl BitOr for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_or(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl BitOr<u64> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.bitor(rhs)
    }
}

impl BitOr<i64> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.bitor(rhs)
    }
}

impl BitOr<u32> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.bitor(rhs)
    }
}

impl BitOr<i32> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.bitor(rhs)
    }
}

impl BitXor for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_xor(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl BitXor<u64> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.bitxor(rhs)
    }
}

impl BitXor<i64> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.bitxor(rhs)
    }
}

impl BitXor<u32> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.bitxor(rhs)
    }
}

impl BitXor<i32> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.bitxor(rhs)
    }
}

impl Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        let env = self.env();
        let b = env.bigint_neg(self.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Not for BigInt {
    type Output = BigInt;
    fn not(self) -> Self::Output {
        let env = self.env();
        let b = env.bigint_not(self.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Shl<BigInt> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: BigInt) -> Self::Output {
        let env = self.env();
        let b = env.bigint_shl(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Shl<u64> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.shl(rhs)
    }
}

impl Shl<i64> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.shl(rhs)
    }
}

impl Shl<u32> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.shl(rhs)
    }
}

impl Shl<i32> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.shl(rhs)
    }
}

impl Shr<BigInt> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: BigInt) -> Self::Output {
        let env = self.env();
        let b = env.bigint_shl(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}

impl Shr<u64> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.shr(rhs)
    }
}

impl Shr<i64> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.shr(rhs)
    }
}

impl Shr<u32> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.shr(rhs)
    }
}

impl Shr<i32> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.shr(rhs)
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialEq<u64> for BigInt {
    fn eq(&self, other: &u64) -> bool {
        self.eq(&BigInt::from_u64(self.env(), *other))
    }
}

impl PartialEq<i64> for BigInt {
    fn eq(&self, other: &i64) -> bool {
        self.eq(&BigInt::from_i64(self.env(), *other))
    }
}

impl PartialEq<u32> for BigInt {
    fn eq(&self, other: &u32) -> bool {
        self.eq(&BigInt::from_u32(self.env(), *other))
    }
}

impl PartialEq<i32> for BigInt {
    fn eq(&self, other: &i32) -> bool {
        self.eq(&BigInt::from_i32(self.env(), *other))
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl PartialOrd<u64> for BigInt {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        self.partial_cmp(&BigInt::from_u64(self.env(), *other))
    }
}

impl PartialOrd<i64> for BigInt {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.partial_cmp(&BigInt::from_i64(self.env(), *other))
    }
}

impl PartialOrd<u32> for BigInt {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.partial_cmp(&BigInt::from_u32(self.env(), *other))
    }
}

impl PartialOrd<i32> for BigInt {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.partial_cmp(&BigInt::from_i32(self.env(), *other))
    }
}

impl Eq for BigInt {}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let env = self.env();
        let v = env.bigint_cmp(self.0.to_object(), other.0.to_object());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl BigInt {
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    fn env(&self) -> &Env {
        self.0.env()
    }

    /// Creates a [BigInt] with the value of the [u64].
    pub fn from_u64(env: &Env, u: u64) -> BigInt {
        let obj = env.bigint_from_u64(u).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    /// Returns the [BigInt] as a [u64].
    ///
    /// ### Panics
    ///
    /// When the [BigInt] is greater than [u64::MAX].
    unsafe fn to_u64(&self) -> u64 {
        let env = self.env();
        env.bigint_to_u64(self.0.to_object())
    }

    /// Creates a [BigInt] with the value of the [i64].
    pub fn from_i64(env: &Env, i: i64) -> BigInt {
        let obj = env.bigint_from_i64(i).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    /// Returns the [BigInt] as a [i64].
    ///
    /// ### Panics
    ///
    /// When the [BigInt] is greater than [i64::MAX].
    unsafe fn to_i64(&self) -> i64 {
        let env = self.env();
        env.bigint_to_i64(self.0.to_object())
    }

    /// Creates a [BigInt] with the value of the [u32].
    pub fn from_u32(env: &Env, u: u32) -> BigInt {
        let obj = env.bigint_from_u64(u as u64).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    /// Returns the [BigInt] as a [u32].
    ///
    /// ### Panics
    ///
    /// When the [BigInt] is greater than [u32::MAX].
    unsafe fn to_u32(&self) -> u32 {
        let env = self.env();
        let u = env.bigint_to_u64(self.0.to_object());
        u.try_into().unwrap()
    }

    /// Creates a [BigInt] with the value of the [i32].
    pub fn from_i32(env: &Env, i: i32) -> BigInt {
        let obj = env.bigint_from_i64(i as i64).in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    /// Returns the [BigInt] as a [i32].
    ///
    /// ### Panics
    ///
    /// When the [BigInt] is greater than [i32::MAX].
    unsafe fn to_i32(&self) -> i32 {
        let env = self.env();
        let i = env.bigint_to_i64(self.0.to_object());
        i.try_into().unwrap()
    }

    /// Returns the greatest common divisor of the [BigInt] and other.
    pub fn gcd(&self, other: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_gcd(self.0.to_object(), other.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }

    /// Returns the lowest common multiple of the [BigInt] and other.
    pub fn lcm(&self, other: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_lcm(self.0.to_object(), other.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }

    /// Returns the [BigInt] raised to the power specified.
    pub fn pow(&self, power: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_pow(self.0.to_object(), power.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }

    /// Returns `p.pow(q) mod m`.
    ///
    /// ### Panics
    ///
    /// When `q` is negative or `m` is zero.
    pub fn pow_mod(&self, q: BigInt, m: BigInt) -> BigInt {
        let env = self.env();
        let b = env.bigint_pow_mod(self.0.to_object(), q.0.to_object(), m.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }

    /// Returns the square root of the [BigInt].
    pub fn sqrt(&self) -> BigInt {
        let env = self.env();
        let b = env.bigint_sqrt(self.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }

    /// Returns true if the [BigInt] is zero.
    pub fn is_zero(&self) -> bool {
        let env = self.env();
        let is_zero = env.bigint_is_zero(self.0.to_object());
        bool::try_from(is_zero).unwrap()
    }

    /// Returns the minimum number of bits required to store the [BigInt].
    pub fn bits(&self) -> u32 {
        let env = self.env();
        let bits = env.bigint_bits(self.0.to_object());
        u32::try_from(bits).unwrap()
    }
}

#[test]
fn test_bigint() {
    let env = Env::default();
    let bi0 = BigInt::from_u64(&env, 237834);
    println!("{:?}; {}", bi0, bi0);
    let bi1 = BigInt::from_i64(&env, -3748709);
    println!("{:?}; {}", bi1, bi1);
    let bi2 = BigInt::from_i64(&env, 0);
    println!("{:?}; {}", bi2, bi2);
}
