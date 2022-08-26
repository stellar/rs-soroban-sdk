use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use super::{
    env::internal::{Env as _, EnvBase, RawValConvertible},
    env::{EnvObj, EnvType},
    xdr::ScObjectType,
    Bytes, ConversionError, Env, EnvVal, IntoVal, Object, RawVal, TryFromVal, TryIntoVal,
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
        let obj: Object = env.bigint_to_radix_be(bi, 10u32.into());
        if let Ok(bin) = TryIntoVal::<_, Bytes>::try_into_val(obj, &env) {
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

impl TryFromVal<Env, Object> for BigInt {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: Object) -> Result<Self, Self::Error> {
        if val.is_obj_type(ScObjectType::BigInt) {
            Ok(BigInt(val.in_env(env)))
        } else {
            Err(ConversionError {})
        }
    }
}

impl TryFromVal<Env, RawVal> for BigInt {
    type Error = <BigInt as TryFromVal<Env, Object>>::Error;

    fn try_from_val(env: &Env, val: RawVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, val.try_into()?)
    }
}

impl TryIntoVal<Env, BigInt> for Object {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<BigInt, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl TryIntoVal<Env, BigInt> for RawVal {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<BigInt, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl IntoVal<Env, RawVal> for BigInt {
    fn into_val(self, _env: &Env) -> RawVal {
        self.into()
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
            Ok(b.to_u64())
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
            Ok(b.to_i64())
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
            Ok(b.to_u32())
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
            Ok(b.to_i32())
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
use super::xdr::ScVal;

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&BigInt> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &BigInt) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.0.env, v.0.val.to_raw())
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
impl TryFromVal<Env, ScVal> for BigInt {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(
            env,
            val.try_into_val(env).map_err(|_| ConversionError)?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, BigInt> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<BigInt, Self::Error> {
        BigInt::try_from_val(env, self)
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
crate::operators::impl_ref_op!(BigInt, Add<BigInt>::add);

impl Add<u64> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.add(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Add<u64>::add);

impl Add<i64> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.add(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Add<i64>::add);

impl Add<u32> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.add(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Add<u32>::add);

impl Add<i32> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.add(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Add<i32>::add);

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_sub(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Sub<BigInt>::sub);

impl Sub<u64> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.sub(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Sub<u64>::sub);

impl Sub<i64> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.sub(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Sub<i64>::sub);

impl Sub<u32> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.sub(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Sub<u32>::sub);

impl Sub<i32> for BigInt {
    type Output = BigInt;
    fn sub(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.sub(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Sub<i32>::sub);

impl Mul for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_mul(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Mul<BigInt>::mul);

impl Mul<u64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.mul(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Mul<u64>::mul);

impl Mul<i64> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.mul(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Mul<i64>::mul);

impl Mul<u32> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.mul(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Mul<u32>::mul);

impl Mul<i32> for BigInt {
    type Output = BigInt;
    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.mul(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Mul<i32>::mul);

impl Div for BigInt {
    type Output = BigInt;
    fn div(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_div(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Div<BigInt>::div);

impl Div<u64> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.div(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Div<u64>::div);

impl Div<i64> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.div(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Div<i64>::div);

impl Div<u32> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.div(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Div<u32>::div);

impl Div<i32> for BigInt {
    type Output = BigInt;
    fn div(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.div(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Div<i32>::div);

impl Rem for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_rem(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Rem<BigInt>::rem);

impl Rem<u64> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.rem(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Rem<u64>::rem);

impl Rem<i64> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.rem(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Rem<i64>::rem);

impl Rem<u32> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.rem(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Rem<u32>::rem);

impl Rem<i32> for BigInt {
    type Output = BigInt;
    fn rem(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.rem(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Rem<i32>::rem);

impl BitAnd for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_and(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, BitAnd<BigInt>::bitand);

impl BitAnd<u64> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.bitand(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitAnd<u64>::bitand);

impl BitAnd<i64> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.bitand(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitAnd<i64>::bitand);

impl BitAnd<u32> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.bitand(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitAnd<u32>::bitand);

impl BitAnd<i32> for BigInt {
    type Output = BigInt;
    fn bitand(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.bitand(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitAnd<i32>::bitand);

impl BitOr for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_or(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, BitOr<BigInt>::bitor);

impl BitOr<u64> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.bitor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitOr<u64>::bitor);

impl BitOr<i64> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.bitor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitOr<i64>::bitor);

impl BitOr<u32> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.bitor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitOr<u32>::bitor);

impl BitOr<i32> for BigInt {
    type Output = BigInt;
    fn bitor(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.bitor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitOr<i32>::bitor);

impl BitXor for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let env = self.env();
        env.check_same_env(rhs.env());
        let b = env.bigint_xor(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, BitXor<BigInt>::bitxor);

impl BitXor<u64> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.bitxor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitXor<u64>::bitxor);

impl BitXor<i64> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.bitxor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitXor<i64>::bitxor);

impl BitXor<u32> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.bitxor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitXor<u32>::bitxor);

impl BitXor<i32> for BigInt {
    type Output = BigInt;
    fn bitxor(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.bitxor(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, BitXor<i32>::bitxor);

impl Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        let env = self.env();
        let b = env.bigint_neg(self.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Neg::neg);

impl Not for BigInt {
    type Output = BigInt;
    fn not(self) -> Self::Output {
        let env = self.env();
        let b = env.bigint_not(self.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Not::not);

impl Shl<BigInt> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: BigInt) -> Self::Output {
        let env = self.env();
        let b = env.bigint_shl(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Shl<BigInt>::shl);

impl Shl<u64> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.shl(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shl<u64>::shl);

impl Shl<i64> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.shl(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shl<i64>::shl);

impl Shl<u32> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.shl(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shl<u32>::shl);

impl Shl<i32> for BigInt {
    type Output = BigInt;
    fn shl(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.shl(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shl<i32>::shl);

impl Shr<BigInt> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: BigInt) -> Self::Output {
        let env = self.env();
        let b = env.bigint_shl(self.0.to_object(), rhs.0.to_object());
        Self::try_from_val(env, b).unwrap()
    }
}
crate::operators::impl_ref_op!(BigInt, Shr<BigInt>::shr);

impl Shr<u64> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: u64) -> Self::Output {
        let rhs = BigInt::from_u64(self.env(), rhs);
        self.shr(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shr<u64>::shr);

impl Shr<i64> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: i64) -> Self::Output {
        let rhs = BigInt::from_i64(self.env(), rhs);
        self.shr(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shr<i64>::shr);

impl Shr<u32> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: u32) -> Self::Output {
        let rhs = BigInt::from_u32(self.env(), rhs);
        self.shr(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shr<u32>::shr);

impl Shr<i32> for BigInt {
    type Output = BigInt;
    fn shr(self, rhs: i32) -> Self::Output {
        let rhs = BigInt::from_i32(self.env(), rhs);
        self.shr(rhs)
    }
}
crate::operators::impl_ref_op!(BigInt, Shr<i32>::shr);

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

    /// Creates a [BigInt] with the value zero.
    pub fn zero(env: &Env) -> BigInt {
        BigInt::from_u32(env, 0)
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
    pub fn to_u64(&self) -> u64 {
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
    pub fn to_i64(&self) -> i64 {
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
    pub fn to_u32(&self) -> u32 {
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
    pub fn to_i32(&self) -> i32 {
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
