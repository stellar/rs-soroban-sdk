use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::Debug,
    iter::FusedIterator,
    ops::{Bound, RangeBounds},
};

use crate::u32usize::u32_to_usize;

use super::{
    env::internal::Env as _, env::EnvType, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal,
    Object, RawVal, RawValConvertible,
};

#[cfg(not(target_family = "wasm"))]
use super::{env::TryIntoEnvVal, xdr::ScVal};

#[derive(Clone)]
#[repr(transparent)]
pub struct Binary(EnvObj);

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

impl From<Binary> for Object {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0.val
    }
}

impl From<&Binary> for Object {
    #[inline(always)]
    fn from(v: &Binary) -> Self {
        v.0.val
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Binary> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Binary) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Binary> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Binary) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<EnvType<ScVal>> for Binary {
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        let ev: EnvObj = v
            .val
            .try_into_env_val(&v.env)
            .map_err(|_| ConversionError)?;
        ev.try_into()
    }
}

impl Binary {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(obj: EnvObj) -> Self {
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

    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [u8; N]) -> Binary {
        let mut bin = Binary::new(env);
        bin.extend_from_array(items);
        bin
    }

    #[inline(always)]
    pub fn from_slice(env: &Env, items: &[u8]) -> Binary {
        let mut vec = Binary::new(env);
        vec.extend_from_slice(items);
        vec
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, v: u8) {
        let i: u32 = i.try_into().unwrap();
        let v32: u32 = v.into();
        self.0 = self
            .env()
            .binary_put(self.0.to_tagged(), i.into(), v32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> Option<u8> {
        if i < self.len() {
            Some(self.get_unchecked(i))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i: usize) -> u8 {
        let i: u32 = i.try_into().unwrap();
        let res32: u32 = self
            .env()
            .binary_get(self.0.to_tagged(), i.into())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.env().binary_len(self.0.to_tagged()).is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        let env = self.env();
        let val = env.binary_len(self.0.to_tagged());
        u32_to_usize(u32::try_from(val).unwrap())
    }

    #[inline(always)]
    pub fn first(&self) -> Option<u8> {
        if !self.is_empty() {
            Some(self.first_unchecked())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn first_unchecked(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_front(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    pub fn last(&self) -> Option<u8> {
        if !self.is_empty() {
            Some(self.last_unchecked())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn last_unchecked(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_back(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    pub fn remove(&mut self, i: usize) -> Option<()> {
        if i < self.len() {
            self.remove_unchecked(i);
            Some(())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn remove_unchecked(&mut self, i: usize) {
        let i: u32 = i.try_into().unwrap();
        let env = self.env();
        let bin = env.binary_del(self.0.to_tagged(), i.into());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn push(&mut self, x: u8) {
        let x32: u32 = x.into();
        let env = self.env();
        let bin = env.binary_push(self.0.to_tagged(), x32.into());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u8> {
        let last = self.last()?;
        let env = self.env();
        let bin = env.binary_pop(self.0.to_tagged());
        self.0 = bin.in_env(env);
        Some(last)
    }

    #[inline(always)]
    pub fn pop_unchecked(&mut self) -> u8 {
        let last = self.last_unchecked();
        let env = self.env();
        self.0 = env.binary_pop(self.0.to_tagged()).in_env(env);
        last
    }

    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: u8) {
        let env = self.env();
        let x32: u32 = x.into();
        let bin = env.binary_insert(self.0.to_tagged(), i.into(), x32.into());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Binary) {
        let env = self.env();
        let bin = env.binary_append(self.0.to_tagged(), other.0.to_tagged());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, items: [u8; N]) {
        for item in items {
            self.push(item);
        }
    }

    #[inline(always)]
    pub fn extend_from_slice(&mut self, items: &[u8]) {
        for item in items {
            self.push(*item);
        }
    }

    #[must_use]
    pub fn slice(&self, r: impl RangeBounds<usize>) -> Self {
        let start_bound: u32 = match r.start_bound() {
            Bound::Included(s) => *s,
            Bound::Excluded(s) => *s + 1,
            Bound::Unbounded => 0,
        }
        .try_into()
        .unwrap();
        let end_bound: u32 = match r.end_bound() {
            Bound::Included(s) => *s + 1,
            Bound::Excluded(s) => *s,
            Bound::Unbounded => self.len(),
        }
        .try_into()
        .unwrap();
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
pub struct BinIter(Binary);

impl BinIter {
    fn into_bin(self) -> Binary {
        self.0
    }
}

impl Iterator for BinIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() == 0 {
            None
        } else {
            let val = self.0.env().binary_front(self.0 .0.to_object());
            self.0 = self.0.slice(1..);
            #[allow(clippy::cast_possible_truncation)]
            let val = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) } as u8;
            Some(val)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len();
        (len, Some(len))
    }
}

impl DoubleEndedIterator for BinIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len == 0 {
            None
        } else {
            let val = self.0.env().binary_back(self.0 .0.to_object());
            self.0 = self.0.slice(..len - 1);
            #[allow(clippy::cast_possible_truncation)]
            let val = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) } as u8;
            Some(val)
        }
    }
}

impl FusedIterator for BinIter {}

impl ExactSizeIterator for BinIter {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct FixedBinary<const N: usize>(Binary);

impl<const N: usize> Debug for FixedBinary<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ArrayBinary{{length = {}, ", N)?;
        write!(f, "{:?}}}", self.0)?;
        Ok(())
    }
}

impl<const N: usize> Eq for FixedBinary<N> {}

impl<const N: usize> PartialEq for FixedBinary<N> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<const N: usize> PartialOrd for FixedBinary<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<const N: usize> Ord for FixedBinary<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<const N: usize> Borrow<Binary> for FixedBinary<N> {
    fn borrow(&self) -> &Binary {
        &self.0
    }
}

impl<const N: usize> Borrow<Binary> for &FixedBinary<N> {
    fn borrow(&self) -> &Binary {
        &self.0
    }
}

impl<const N: usize> Borrow<Binary> for &mut FixedBinary<N> {
    fn borrow(&self) -> &Binary {
        &self.0
    }
}

impl<const N: usize> AsRef<Binary> for FixedBinary<N> {
    fn as_ref(&self) -> &Binary {
        &self.0
    }
}

impl<const N: usize> From<EnvType<[u8; N]>> for FixedBinary<N> {
    fn from(ev: EnvType<[u8; N]>) -> Self {
        let mut bin = Binary::new(&ev.env);
        for b in ev.val {
            bin.push(b);
        }
        FixedBinary::<N>(bin)
    }
}

impl<const N: usize> TryFrom<EnvVal> for FixedBinary<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<const N: usize> TryFrom<EnvObj> for FixedBinary<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        let bin: Binary = obj.try_into()?;
        bin.try_into()
    }
}

impl<const N: usize> TryFrom<Binary> for FixedBinary<N> {
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

impl<const N: usize> From<FixedBinary<N>> for RawVal {
    #[inline(always)]
    fn from(v: FixedBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: usize> From<FixedBinary<N>> for EnvVal {
    #[inline(always)]
    fn from(v: FixedBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: usize> From<FixedBinary<N>> for EnvObj {
    #[inline(always)]
    fn from(v: FixedBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: usize> From<FixedBinary<N>> for Binary {
    #[inline(always)]
    fn from(v: FixedBinary<N>) -> Self {
        v.0
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<&FixedBinary<N>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &FixedBinary<N>) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<FixedBinary<N>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: FixedBinary<N>) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<EnvType<ScVal>> for FixedBinary<N> {
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        v.try_into()
    }
}

impl<const N: usize> FixedBinary<N> {
    #[inline(always)]
    pub fn from_array<const M: usize>(env: &Env, items: [u8; M]) -> FixedBinary<N> {
        Binary::from_array(env, items).try_into().unwrap()
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, v: u8) {
        self.0.set(i, v);
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> Option<u8> {
        self.0.get(i)
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i: usize) -> u8 {
        self.0.get_unchecked(i)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        N
    }

    #[inline(always)]
    pub fn first(&self) -> Option<u8> {
        self.0.first()
    }

    #[inline(always)]
    pub fn first_unchecked(&self) -> u8 {
        self.0.first_unchecked()
    }

    #[inline(always)]
    pub fn last(&self) -> Option<u8> {
        self.0.last()
    }

    #[inline(always)]
    pub fn last_unchecked(&self) -> u8 {
        self.0.last_unchecked()
    }

    pub fn iter(&self) -> BinIter {
        self.clone().into_iter()
    }
}

impl<const N: usize> IntoIterator for FixedBinary<N> {
    type Item = u8;

    type IntoIter = BinIter;

    fn into_iter(self) -> Self::IntoIter {
        BinIter(self.0)
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

        let bad_fixed: Result<FixedBinary<4>, ConversionError> = bin.try_into();
        assert!(bad_fixed.is_err());
        let fixed: FixedBinary<3> = bin_copy.try_into().unwrap();
        println!("{:?}", fixed);
    }

    #[test]
    fn test_bin_iter() {
        let env = Env::default();
        let mut bin = Binary::new(&env);
        bin.push(10);
        bin.push(20);
        bin.push(30);
        let mut iter = bin.iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        let mut iter = bin.iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next_back(), Some(30));
        assert_eq!(iter.next_back(), Some(20));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let fixed: FixedBinary<3> = bin.try_into().unwrap();
        let mut iter = fixed.iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        let mut iter = fixed.iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next_back(), Some(30));
        assert_eq!(iter.next_back(), Some(20));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_array_binary_borrow() {
        fn get_len(b: impl Borrow<Binary>) -> usize {
            let b: &Binary = b.borrow();
            b.len()
        }

        let env = Env::default();
        let mut bin = Binary::new(&env);
        bin.push(10);
        bin.push(20);
        bin.push(30);
        assert_eq!(bin.len(), 3);

        let arr_bin: FixedBinary<3> = bin.clone().try_into().unwrap();
        assert_eq!(arr_bin.len(), 3);

        assert_eq!(get_len(&bin), 3);
        assert_eq!(get_len(bin), 3);
        assert_eq!(get_len(&arr_bin), 3);
        assert_eq!(get_len(arr_bin), 3);
    }
}
