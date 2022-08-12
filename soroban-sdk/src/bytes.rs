use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::Debug,
    iter::FusedIterator,
    ops::{Bound, RangeBounds},
};

use super::{
    env::internal::{Env as _, RawValConvertible},
    env::{EnvObj, EnvType, IntoVal},
    xdr::ScObjectType,
    ConversionError, Env, EnvVal, Object, RawVal, TryIntoVal,
};

#[cfg(doc)]
use crate::{ContractData, Map, Vec};

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

#[deprecated(note = "use soroban_sdk::Bytes")]
pub type Binary = Bytes;

#[deprecated(note = "use soroban_sdk::BytesN")]
pub type FixedBinary<const N: usize> = BytesN<N>;

#[macro_export]
macro_rules! bytes {
    ($env:expr) => {
        $crate::Bytes::new($env)
    };
    ($env:expr, $($x:expr),+ $(,)?) => {
        $crate::Bytes::from_array($env, [$($x),+])
    };
}

/// Bytes is a contiguous growable array type containing `u8`s.
///
/// The array is stored in the Host and available to the Guest through the
/// functions defined on Bytes.
///
/// Bytes values can be stored as [ContractData], or in other
/// types like [Vec], [Map], etc.
///
/// ### Examples
///
/// Bytes values can be created from slices:
/// ```
/// use soroban_sdk::{Bytes, Env};
///
/// let env = Env::default();
/// let bin = Bytes::from_slice(&env, &[0; 32]);
/// assert_eq!(bin.len(), 32);
/// ```
#[derive(Clone)]
#[repr(transparent)]
pub struct Bytes(EnvObj);

impl Debug for Bytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bytes(")?;
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

impl Eq for Bytes {}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Bytes {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl TryFrom<EnvVal> for Bytes {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for Bytes {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_object().is_obj_type(ScObjectType::Bytes) {
            Ok(unsafe { Bytes::unchecked_new(obj) })
        } else {
            Err(ConversionError {})
        }
    }
}

impl TryIntoVal<Env, Bytes> for RawVal {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Bytes, Self::Error> {
        EnvType {
            env: env.clone(),
            val: self,
        }
        .try_into()
    }
}

impl From<Bytes> for RawVal {
    #[inline(always)]
    fn from(v: Bytes) -> Self {
        v.0.into()
    }
}

impl From<Bytes> for EnvVal {
    #[inline(always)]
    fn from(v: Bytes) -> Self {
        v.0.into()
    }
}

impl From<Bytes> for EnvObj {
    #[inline(always)]
    fn from(v: Bytes) -> Self {
        v.0
    }
}

impl From<Bytes> for Object {
    #[inline(always)]
    fn from(v: Bytes) -> Self {
        v.0.val
    }
}

impl From<&Bytes> for Object {
    #[inline(always)]
    fn from(v: &Bytes) -> Self {
        v.0.val
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Bytes> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Bytes) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Bytes> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Bytes) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, Bytes> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Bytes, Self::Error> {
        let o: Object = self.try_into_val(env).map_err(|_| ConversionError)?;
        let env = env.clone();
        EnvObj { val: o, env }.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<EnvType<ScVal>> for Bytes {
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        ScVal::try_into_val(v.val, &v.env)
    }
}

impl IntoVal<Env, Bytes> for &str {
    fn into_val(self, env: &Env) -> Bytes {
        Bytes::from_slice(env, self.as_bytes())
    }
}

impl From<EnvType<&str>> for Bytes {
    fn from(ev: EnvType<&str>) -> Self {
        Bytes::from_slice(&ev.env, ev.val.as_bytes())
    }
}

impl Bytes {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub(crate) fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub(crate) fn as_object(&self) -> &Object {
        self.0.as_object()
    }

    pub(crate) fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub(crate) fn to_object(&self) -> Object {
        self.0.to_object()
    }

    /// Create an empty Bytes.
    #[inline(always)]
    pub fn new(env: &Env) -> Bytes {
        let obj = env.binary_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    /// Create a Bytes from the given `[u8]`.
    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [u8; N]) -> Bytes {
        BytesN::from_array(env, items).0
    }

    #[inline(always)]
    pub fn from_slice(env: &Env, items: &[u8]) -> Bytes {
        let mut vec = Bytes::new(env);
        vec.extend_from_slice(items);
        vec
    }

    #[inline(always)]
    pub fn set(&mut self, i: u32, v: u8) {
        let v32: u32 = v.into();
        self.0 = self
            .env()
            .binary_put(self.0.to_object(), i.into(), v32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> Option<u8> {
        if i < self.len() {
            Some(self.get_unchecked(i))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i: u32) -> u8 {
        let res32: u32 = self
            .env()
            .binary_get(self.0.to_object(), i.into())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.env().binary_len(self.0.to_object()).is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env()
            .binary_len(self.0.to_object())
            .try_into()
            .unwrap()
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
            .binary_front(self.0.to_object())
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
            .binary_back(self.0.to_object())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    pub fn remove(&mut self, i: u32) -> Option<()> {
        if i < self.len() {
            self.remove_unchecked(i);
            Some(())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn remove_unchecked(&mut self, i: u32) {
        let env = self.env();
        let bin = env.binary_del(self.0.to_object(), i.into());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn push(&mut self, x: u8) {
        let x32: u32 = x.into();
        let env = self.env();
        let bin = env.binary_push(self.0.to_object(), x32.into());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u8> {
        let last = self.last()?;
        let env = self.env();
        let bin = env.binary_pop(self.0.to_object());
        self.0 = bin.in_env(env);
        Some(last)
    }

    #[inline(always)]
    pub fn pop_unchecked(&mut self) -> u8 {
        let last = self.last_unchecked();
        let env = self.env();
        self.0 = env.binary_pop(self.0.to_object()).in_env(env);
        last
    }

    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: u8) {
        let env = self.env();
        let x32: u32 = x.into();
        let bin = env.binary_insert(self.0.to_object(), i.into(), x32.into());
        self.0 = bin.in_env(env);
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Bytes) {
        let env = self.env();
        let bin = env.binary_append(self.0.to_object(), other.0.to_object());
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
        let bin = env.binary_slice(self.0.to_object(), start_bound.into(), end_bound.into());
        unsafe { Self::unchecked_new(bin.in_env(env)) }
    }

    pub fn iter(&self) -> BinIter {
        self.clone().into_iter()
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = BinIter;

    fn into_iter(self) -> Self::IntoIter {
        BinIter(self)
    }
}

#[derive(Clone)]
pub struct BinIter(Bytes);

impl BinIter {
    fn into_bin(self) -> Bytes {
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
            let val = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) } as u8;
            Some(val)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as usize;
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
            let val = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) } as u8;
            Some(val)
        }
    }
}

impl FusedIterator for BinIter {}

impl ExactSizeIterator for BinIter {
    fn len(&self) -> usize {
        self.0.len() as usize
    }
}

/// BytesN is a contiguous fixed-size array type containing `u8`s.
///
/// The array is stored in the Host and available to the Guest through the
/// functions defined on Bytes.
///
/// Bytes values can be stored as [ContractData], or in other
/// types like [Vec], [Map], etc.
///
/// ### Examples
///
/// BytesN values can be created from arrays:
/// ```
/// use soroban_sdk::{Bytes, BytesN, Env};
///
/// let env = Env::default();
/// let bin = BytesN::from_array(&env, [0; 32]);
/// assert_eq!(bin.len(), 32);
/// ```
///
/// BytesN and Bytes values are convertible:
/// ```
/// use soroban_sdk::{Bytes, BytesN, Env};
///
/// let env = Env::default();
/// let bin = Bytes::from_slice(&env, &[0; 32]);
/// let bin: BytesN<32> = bin.try_into().expect("bin to have length 32");
/// assert_eq!(bin.len(), 32);
/// ```
#[derive(Clone)]
#[repr(transparent)]
pub struct BytesN<const N: usize>(Bytes);

impl<const N: usize> Debug for BytesN<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BytesN<{}>(", N)?;
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

impl<const N: usize> Eq for BytesN<N> {}

impl<const N: usize> PartialEq for BytesN<N> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<const N: usize> PartialOrd for BytesN<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<const N: usize> Ord for BytesN<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<const N: usize> Borrow<Bytes> for BytesN<N> {
    fn borrow(&self) -> &Bytes {
        &self.0
    }
}

impl<const N: usize> Borrow<Bytes> for &BytesN<N> {
    fn borrow(&self) -> &Bytes {
        &self.0
    }
}

impl<const N: usize> Borrow<Bytes> for &mut BytesN<N> {
    fn borrow(&self) -> &Bytes {
        &self.0
    }
}

impl<const N: usize> AsRef<Bytes> for BytesN<N> {
    fn as_ref(&self) -> &Bytes {
        &self.0
    }
}

impl<const N: usize> From<EnvType<[u8; N]>> for BytesN<N> {
    #[inline(always)]
    fn from(ev: EnvType<[u8; N]>) -> Self {
        let mut bin = Bytes::new(&ev.env);
        for b in ev.val {
            bin.push(b);
        }
        BytesN(bin)
    }
}

impl<const N: usize> IntoVal<Env, BytesN<N>> for [u8; N] {
    fn into_val(self, env: &Env) -> BytesN<N> {
        EnvType {
            env: env.clone(),
            val: self,
        }
        .into()
    }
}

impl<const N: usize> TryFrom<EnvVal> for BytesN<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<const N: usize> TryFrom<EnvObj> for BytesN<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        let bin: Bytes = obj.try_into()?;
        bin.try_into()
    }
}

impl<const N: usize> TryIntoVal<Env, BytesN<N>> for RawVal {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<BytesN<N>, Self::Error> {
        EnvType {
            env: env.clone(),
            val: self,
        }
        .try_into()
    }
}

impl<const N: usize> TryFrom<Bytes> for BytesN<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(bin: Bytes) -> Result<Self, Self::Error> {
        if bin.len() == { N as u32 } {
            Ok(Self(bin))
        } else {
            Err(ConversionError {})
        }
    }
}

impl<const N: usize> From<BytesN<N>> for RawVal {
    #[inline(always)]
    fn from(v: BytesN<N>) -> Self {
        v.0.into()
    }
}

impl<const N: usize> From<BytesN<N>> for EnvVal {
    #[inline(always)]
    fn from(v: BytesN<N>) -> Self {
        v.0.into()
    }
}

impl<const N: usize> From<BytesN<N>> for EnvObj {
    #[inline(always)]
    fn from(v: BytesN<N>) -> Self {
        v.0.into()
    }
}

impl<const N: usize> From<BytesN<N>> for Bytes {
    #[inline(always)]
    fn from(v: BytesN<N>) -> Self {
        v.0
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<&BytesN<N>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &BytesN<N>) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<BytesN<N>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: BytesN<N>) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryIntoVal<Env, BytesN<N>> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<BytesN<N>, Self::Error> {
        let o: Object = self.try_into_val(env).map_err(|_| ConversionError)?;
        let env = env.clone();
        EnvObj { val: o, env }.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<EnvType<ScVal>> for BytesN<N> {
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        ScVal::try_into_val(v.val, &v.env)
    }
}

impl<const N: usize> BytesN<N> {
    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub(crate) fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub(crate) fn as_object(&self) -> &Object {
        self.0.as_object()
    }

    pub(crate) fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub(crate) fn to_object(&self) -> Object {
        self.0.to_object()
    }

    #[inline(always)]
    pub fn from_array(env: &Env, items: [u8; N]) -> BytesN<N> {
        let mut bin = Bytes::new(env);
        bin.extend_from_array(items);
        BytesN(bin)
    }

    #[inline(always)]
    pub fn set(&mut self, i: u32, v: u8) {
        self.0.set(i, v);
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> Option<u8> {
        self.0.get(i)
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i: u32) -> u8 {
        self.0.get_unchecked(i)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        false
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        N as u32
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

impl<const N: usize> IntoIterator for BytesN<N> {
    type Item = u8;

    type IntoIter = BinIter;

    fn into_iter(self) -> Self::IntoIter {
        BinIter(self.0)
    }
}

impl<const N: usize> TryFrom<Bytes> for [u8; N] {
    type Error = ConversionError;

    fn try_from(bin: Bytes) -> Result<Self, Self::Error> {
        let fixed: BytesN<N> = bin.try_into()?;
        Ok(fixed.into())
    }
}

impl<const N: usize> From<BytesN<N>> for [u8; N] {
    fn from(bin: BytesN<N>) -> Self {
        let mut res = [0u8; N];
        for (i, b) in bin.into_iter().enumerate() {
            res[i] = b;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bin_macro() {
        let env = Env::default();
        assert_eq!(bytes![&env], Bytes::new(&env));
        assert_eq!(bytes![&env, 1], {
            let mut b = Bytes::new(&env);
            b.push(1);
            b
        });
        assert_eq!(bytes![&env, 1,], {
            let mut b = Bytes::new(&env);
            b.push(1);
            b
        });
        assert_eq!(bytes![&env, 3, 2, 1,], {
            let mut b = Bytes::new(&env);
            b.push(3);
            b.push(2);
            b.push(1);
            b
        });
    }

    #[test]
    fn test_bin() {
        let env = Env::default();

        let mut bin = Bytes::new(&env);
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

        let bad_fixed: Result<BytesN<4>, ConversionError> = bin.try_into();
        assert!(bad_fixed.is_err());
        let fixed: BytesN<3> = bin_copy.try_into().unwrap();
        println!("{:?}", fixed);
    }

    #[test]
    fn test_bin_iter() {
        let env = Env::default();
        let mut bin = Bytes::new(&env);
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

        let fixed: BytesN<3> = bin.try_into().unwrap();
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
        fn get_len(b: impl Borrow<Bytes>) -> u32 {
            let b: &Bytes = b.borrow();
            b.len()
        }

        let env = Env::default();
        let mut bin = Bytes::new(&env);
        bin.push(10);
        bin.push(20);
        bin.push(30);
        assert_eq!(bin.len(), 3);

        let arr_bin: BytesN<3> = bin.clone().try_into().unwrap();
        assert_eq!(arr_bin.len(), 3);

        assert_eq!(get_len(&bin), 3);
        assert_eq!(get_len(bin), 3);
        assert_eq!(get_len(&arr_bin), 3);
        assert_eq!(get_len(arr_bin), 3);
    }

    #[test]
    fn bytesn_debug() {
        let env = Env::default();
        let mut bin = Bytes::new(&env);
        bin.push(10);
        bin.push(20);
        bin.push(30);
        let arr_bin: BytesN<3> = bin.clone().try_into().unwrap();
        assert_eq!(format!("{:?}", arr_bin), "BytesN<3>(10, 20, 30)");
    }
}
