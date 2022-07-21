use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Bound, RangeBounds},
};

use super::{
    env::internal::Env as _, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal, RawVal,RawValConvertible
};

pub trait FixedLengthBinary {
    fn put(&mut self, i: u32, v: u8);

    fn get(&self, i: u32) -> u8;

    fn is_empty(&self) -> bool;

    fn len(&self) -> u32;

    fn front(&self) -> u8;

    fn back(&self) -> u8;
}

pub trait VariableLengthBinary: FixedLengthBinary {
    fn del(&mut self, i: u32);

    fn push(&mut self, x: u8);

    fn pop(&mut self);

    fn insert(&mut self, i: u32, x: u8);

    fn append(&mut self, other: &Binary);
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Binary(EnvObj);

impl Eq for Binary {}

impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Binary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Binary {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl TryFrom<EnvVal> for Binary {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for Binary {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Binary) {
            Ok(unsafe { Binary::unchecked_new(obj) })
        } else {
            Err(ConversionError {})
        }
    }
}

impl From<Binary> for RawVal {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0.into()
    }
}

impl From<Binary> for EnvVal {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0.into()
    }
}

impl From<Binary> for EnvObj {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0
    }
}

impl FixedLengthBinary for Binary {
    #[inline(always)]
    fn put(&mut self, i: u32, v: u8) {
        let v32: u32 = v.into();
        self.0 = self
            .env()
            .binary_put(self.0.to_tagged(), i.into(), v32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn get(&self, i: u32) -> u8 {
        let res32: u32 = self
            .env()
            .binary_get(self.0.to_tagged(), i.into())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.env().binary_len(self.0.to_tagged()).is_u32_zero()
    }

    #[inline(always)]
    fn len(&self) -> u32 {
        self.env()
            .binary_len(self.0.to_tagged())
            .try_into()
            .unwrap()
    }

    #[inline(always)]
    fn front(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_front(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    fn back(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_back(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }
}

impl VariableLengthBinary for Binary {
    #[inline(always)]
    fn del(&mut self, i: u32) {
        self.0 = self
            .env()
            .binary_del(self.0.to_tagged(), i.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn push(&mut self, x: u8) {
        let x32: u32 = x.into();
        self.0 = self
            .env()
            .binary_push(self.0.to_tagged(), x32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn pop(&mut self) {
        self.0 = self.env().binary_pop(self.0.to_tagged()).in_env(self.env());
    }

    #[inline(always)]
    fn insert(&mut self, i: u32, x: u8) {
        let x32: u32 = x.into();
        self.0 = self
            .env()
            .binary_insert(self.0.to_tagged(), i.into(), x32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn append(&mut self, other: &Binary) {
        self.0 = self
            .env()
            .binary_append(self.0.to_tagged(), other.0.to_tagged())
            .in_env(self.env());
    }
}

impl Debug for Binary {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Binary(")?;
        let mut iter = self.iter();
        if let Some(x) = iter.next() {
            write!(f, "{:?}", x)?;
        }
        for x in iter {
            write!(f, ", {:?}", x)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct BinIter(Binary);

impl Iterator for BinIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            None
        } else {
            let val = self.0.env().binary_front(self.0 .0.to_object());
            self.0 = self.0.slice(1..);
            let val = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) } as u8;
            Some(val)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as usize;
        (len, Some(len))
    }
}

impl Binary {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Binary {
        let obj = env.binary_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    #[must_use]
    pub fn slice(&self, r: impl RangeBounds<u32>) -> Self {
        let start_bound = match r.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => *s + 1,
            Bound::Unbounded => 0,
        };
        let end_bound = match r.end_bound() {
            Bound::Included(s) => *s + 1,
            Bound::Excluded(s) => *s,
            Bound::Unbounded => self.len(),
        };
        let env = self.env();
        let bin = env.binary_slice(self.0.to_tagged(), start_bound.into(), end_bound.into());
        unsafe { Self::unchecked_new(bin.in_env(env)) }
    }

    pub fn iter(&self) -> BinIter {
        self.clone().into_iter()
    }
}

impl IntoIterator for Binary {
    type Item = u8;
    type IntoIter = BinIter;

    fn into_iter(self) -> Self::IntoIter {
        BinIter(self)
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct ArrayBinary<const N: u32>(Binary);

impl<const N: u32> Eq for ArrayBinary<N> {}

impl<const N: u32> PartialEq for ArrayBinary<N> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<const N: u32> PartialOrd for ArrayBinary<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<const N: u32> Ord for ArrayBinary<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<const N: u32> FixedLengthBinary for ArrayBinary<N> {
    #[inline(always)]
    fn put(&mut self, i: u32, v: u8) {
        self.0.put(i, v);
    }

    #[inline(always)]
    fn get(&self, i: u32) -> u8 {
        self.0.get(i)
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        false
    }

    #[inline(always)]
    fn len(&self) -> u32 {
        N
    }

    #[inline(always)]
    fn front(&self) -> u8 {
        self.0.front()
    }

    #[inline(always)]
    fn back(&self) -> u8 {
        self.0.back()
    }
}

impl<const N: u32> TryFrom<EnvVal> for ArrayBinary<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<const N: u32> TryFrom<EnvObj> for ArrayBinary<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        let bin: Binary = obj.try_into()?;
        bin.try_into()
    }
}

impl<const N: u32> TryFrom<Binary> for ArrayBinary<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(bin: Binary) -> Result<Self, Self::Error> {
        if bin.len() == N {
            Ok(Self(bin))
        } else {
            Err(ConversionError {})
        }
    }
}

impl<const N: u32> From<ArrayBinary<N>> for RawVal {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<ArrayBinary<N>> for EnvVal {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<ArrayBinary<N>> for EnvObj {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<ArrayBinary<N>> for Binary {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0
    }
}

impl<const N: u32> Debug for ArrayBinary<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayBinary{{length = {}, ", N)?;
        write!(f, "{:?}}}", self.0)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bin() {
        let env = Env::default();

        let mut bin = Binary::new(&env);
        assert_eq!(bin.len(), 0);
        bin.push(10);
        assert_eq!(bin.len(), 1);
        bin.push(20);
        assert_eq!(bin.len(), 2);
        bin.push(30);
        assert_eq!(bin.len(), 3);
        println!("{:?}", bin);

        let bin_ref = &bin;
        assert_eq!(bin_ref.len(), 3);

        let mut bin_copy = bin.clone();
        assert!(bin == bin_copy);
        assert_eq!(bin_copy.len(), 3);
        bin_copy.push(40);
        assert_eq!(bin_copy.len(), 4);
        assert!(bin != bin_copy);

        assert_eq!(bin.len(), 3);
        assert_eq!(bin_ref.len(), 3);

        bin_copy.pop();
        assert!(bin == bin_copy);

        let bad_fixed: Result<ArrayBinary<4>, ConversionError> = bin.try_into();
        assert!(bad_fixed.is_err());
        let fixed: ArrayBinary<3> = bin_copy.try_into().unwrap();
        println!("{:?}", fixed);
    }
}
