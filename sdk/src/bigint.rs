use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use super::{xdr::ScObjectType, EnvObj, RawVal};

#[repr(transparent)]
#[derive(Clone)]
pub struct BigInt(EnvObj);

impl TryFrom<EnvObj> for BigInt {
    type Error = ();

    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::ScoBigint) {
            Ok(BigInt(obj))
        } else {
            Err(())
        }
    }
}

// impl TryFrom<RawVal> for BigNum {
//     type Error = ();

//     fn try_from(val: RawVal) -> Result<Self, Self::Error> {
//         let obj: Object = val.try_into()?;
//         if obj.is_type(ScObjectType::ScoBigint) {
//             Ok(BigNum(obj))
//         } else {
//             Err(())
//         }
//     }
// }

impl From<BigInt> for EnvObj {
    fn from(b: BigInt) -> Self {
        b.0
    }
}

impl From<BigInt> for RawVal {
    fn from(b: BigInt) -> Self {
        b.0.into()
    }
}

impl From<u64> for BigInt {
    fn from(_x: u64) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_u64(x)) }
        todo!()
    }
}

impl From<BigInt> for u64 {
    fn from(_b: BigInt) -> Self {
        // unsafe { host::bignum::to_u64(b.into()) }
        todo!()
    }
}

impl From<i64> for BigInt {
    fn from(_x: i64) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_i64(x)) }
        todo!()
    }
}

impl From<BigInt> for i64 {
    fn from(_b: BigInt) -> Self {
        // unsafe { host::bignum::to_i64(b.into()) }
        todo!()
    }
}

impl From<u32> for BigInt {
    fn from(_x: u32) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_u64(x.into())) }
        todo!()
    }
}

// TODO: impl From<BigNum> for u32

impl From<i32> for BigInt {
    fn from(_x: i32) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_i64(x.into())) }
        todo!()
    }
}

// TODO: impl From<BigNum> for i32

impl Add for BigInt {
    type Output = BigInt;
    fn add(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::add(self.into(), rhs.into())) }
        todo!()
    }
}

impl Sub for BigInt {
    type Output = BigInt;
    fn sub(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::sub(self.into(), rhs.into())) }
        todo!()
    }
}

impl Mul for BigInt {
    type Output = BigInt;
    fn mul(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::mul(self.into(), rhs.into())) }
        todo!()
    }
}

impl Div for BigInt {
    type Output = BigInt;
    fn div(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::div(self.into(), rhs.into())) }
        todo!()
    }
}

impl Rem for BigInt {
    type Output = BigInt;
    fn rem(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::rem(self.into(), rhs.into())) }
        todo!()
    }
}

impl BitAnd for BigInt {
    type Output = BigInt;
    fn bitand(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::and(self.into(), rhs.into())) }
        todo!()
    }
}

impl BitOr for BigInt {
    type Output = BigInt;
    fn bitor(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::or(self.into(), rhs.into())) }
        todo!()
    }
}

impl BitXor for BigInt {
    type Output = BigInt;
    fn bitxor(self, _rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::xor(self.into(), rhs.into())) }
        todo!()
    }
}

impl Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::neg(self.into())) }
        todo!()
    }
}

impl Not for BigInt {
    type Output = BigInt;
    fn not(self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::not(self.into())) }
        todo!()
    }
}

impl Shl<u64> for BigInt {
    type Output = BigInt;
    fn shl(self, _rhs: u64) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::shl(self.into(), rhs)) }
        todo!()
    }
}

impl Shr<u64> for BigInt {
    type Output = BigInt;
    fn shr(self, _rhs: u64) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::shr(self.into(), rhs)) }
        todo!()
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
    fn cmp(&self, _other: &Self) -> Ordering {
        // let i = unsafe {
        //     <i32 as RawValType>::unchecked_from_val(host::bignum::cmp((*self).into(), (*other).into()))
        todo!()
        // };
        // if i < 0 {
        //     Ordering::Less
        // } else if i > 0 {
        //     Ordering::Greater
        // } else {
        //     Ordering::Equal
        // }
    }
}

impl BigInt {
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    pub fn gcd(&self, _other: BigInt) -> BigInt {
        // unsafe { Self::unchecked_new(host::bignum::gcd((*self).into(), other.into())) }
        todo!()
    }

    pub fn lcm(&self, _other: BigInt) -> BigInt {
        // unsafe { Self::unchecked_new(host::bignum::lcm((*self).into(), other.into())) }
        todo!()
    }

    pub fn pow(&self, _k: u64) -> BigInt {
        // unsafe { Self::unchecked_new(host::bignum::pow((*self).into(), k)) }
        todo!()
    }

    pub fn pow_mod(&self, _q: BigInt, _m: BigInt) -> BigInt {
        // unsafe { Self::unchecked_new(host::bignum::pow_mod((*self).into(), q.into(), m.into())) }
        todo!()
    }

    pub fn sqrt(&self) -> BigInt {
        // unsafe { Self::unchecked_new(host::bignum::sqrt((*self).into())) }
        todo!()
    }

    pub fn is_zero(&self) -> bool {
        // unsafe { <bool as RawValType>::unchecked_from_val(host::bignum::is_zero((*self).into())) }
        todo!()
    }

    pub fn bits(&self) -> u64 {
        // unsafe { host::bignum::bits((*self).into()) }
        todo!()
    }
}
