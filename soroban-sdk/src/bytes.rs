use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::Debug,
    iter::FusedIterator,
    ops::{Bound, RangeBounds},
};

use super::{
    env::internal::{Env as _, EnvBase as _, RawValConvertible},
    env::IntoVal,
    xdr::ScObjectType,
    ConversionError, Env, Object, RawVal, TryFromVal, TryIntoVal,
};

use crate::unwrap::UnwrapOptimized;
#[cfg(doc)]
use crate::{storage::Storage, Map, Vec};

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

/// Create a [Bytes] with an array, or an integer or hex literal.
///
/// The first argument in the list must be a reference to an [Env].
///
/// The second argument can be an [u8] array, or an integer literal of unbounded
/// size in any form: base10, hex, etc.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, bytes};
///
/// let env = Env::default();
/// let bytes = bytes!(&env, 0xfded3f55dec47250a52a8c0bb7038e72fa6ffaae33562f77cd2b629ef7fd424d);
/// assert_eq!(bytes.len(), 32);
/// ```
///
/// ```
/// use soroban_sdk::{Env, bytes};
///
/// let env = Env::default();
/// let bytes = bytes!(&env, [2, 0]);
/// assert_eq!(bytes.len(), 2);
/// ```
///
/// ```
/// use soroban_sdk::{Env, bytes};
///
/// let env = Env::default();
/// let bytes = bytes!(&env);
/// assert_eq!(bytes.len(), 0);
/// ```
#[macro_export]
macro_rules! bytes {
    ($env:expr $(,)?) => {
        $crate::Bytes::new($env)
    };
    ($env:expr, [$($x:expr),+ $(,)?] $(,)?) => {
        $crate::Bytes::from_array($env, &[$($x),+])
    };
    ($env:expr, $x:tt $(,)?) => {
        $crate::Bytes::from_array($env, &$crate::__bytes_lit_bytes!($x))
    };
}

/// Create a [BytesN] with an array, or an integer or hex literal.
///
/// The first argument in the list must be a reference to an [Env].
///
/// The second argument can be an [u8] array, or an integer literal of unbounded
/// size in any form: base10, hex, etc.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, bytesn};
///
/// let env = Env::default();
/// let bytes = bytesn!(&env, 0xfded3f55dec47250a52a8c0bb7038e72fa6ffaae33562f77cd2b629ef7fd424d);
/// assert_eq!(bytes.len(), 32);
/// ```
///
/// ```
/// use soroban_sdk::{Env, bytesn};
///
/// let env = Env::default();
/// let bytes = bytesn!(&env, [2, 0]);
/// assert_eq!(bytes.len(), 2);
/// ```
#[macro_export]
macro_rules! bytesn {
    ($env:expr, [$($x:expr),+ $(,)?] $(,)?) => {
        $crate::BytesN::from_array($env, &[$($x),+])
    };
    ($env:expr, $x:tt $(,)?) => {
        $crate::BytesN::from_array($env, &$crate::__bytes_lit_bytes!($x))
    };
}

/// Bytes is a contiguous growable array type containing `u8`s.
///
/// The array is stored in the Host and available to the Guest through the
/// functions defined on Bytes.
///
/// Bytes values can be stored as [Storage], or in other types like [Vec],
/// [Map], etc.
///
/// ### Examples
///
/// Bytes values can be created from slices:
/// ```
/// use soroban_sdk::{Bytes, Env};
///
/// let env = Env::default();
/// let bytes = Bytes::from_slice(&env, &[1; 32]);
/// assert_eq!(bytes.len(), 32);
/// let mut slice = [0u8; 32];
/// bytes.copy_into_slice(&mut slice);
/// assert_eq!(slice, [1u8; 32]);
/// ```
#[derive(Clone)]
pub struct Bytes {
    env: Env,
    obj: Object,
}

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
        self.env.check_same_env(&other.env);
        let v = self.env.obj_cmp(self.obj.to_raw(), other.obj.to_raw());
        v.cmp(&0)
    }
}

impl IntoVal<Env, Bytes> for Bytes {
    fn into_val(self, _env: &Env) -> Bytes {
        self
    }
}

impl IntoVal<Env, Bytes> for &Bytes {
    fn into_val(self, _env: &Env) -> Bytes {
        self.clone()
    }
}

impl TryFromVal<Env, Object> for Bytes {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: Object) -> Result<Self, Self::Error> {
        if val.is_obj_type(ScObjectType::Bytes) {
            Ok(unsafe { Bytes::unchecked_new(env.clone(), val) })
        } else {
            Err(ConversionError {})
        }
    }
}

impl TryFromVal<Env, RawVal> for Bytes {
    type Error = <Bytes as TryFromVal<Env, Object>>::Error;

    fn try_from_val(env: &Env, val: RawVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, val.try_into()?)
    }
}

impl TryIntoVal<Env, Bytes> for Object {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Bytes, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl TryIntoVal<Env, Bytes> for RawVal {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Bytes, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl IntoVal<Env, RawVal> for Bytes {
    fn into_val(self, _env: &Env) -> RawVal {
        self.into()
    }
}

impl IntoVal<Env, RawVal> for &Bytes {
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
    }
}

impl From<Bytes> for RawVal {
    #[inline(always)]
    fn from(v: Bytes) -> Self {
        v.obj.into()
    }
}

impl From<Bytes> for Object {
    #[inline(always)]
    fn from(v: Bytes) -> Self {
        v.obj
    }
}

impl From<&Bytes> for Object {
    #[inline(always)]
    fn from(v: &Bytes) -> Self {
        v.obj
    }
}

impl From<&Bytes> for Bytes {
    #[inline(always)]
    fn from(v: &Bytes) -> Self {
        v.clone()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Bytes> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Bytes) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.env, v.obj.to_raw())
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
impl TryFromVal<Env, ScVal> for Bytes {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(
            env,
            val.try_into_val(env).map_err(|_| ConversionError)?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, Bytes> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Bytes, Self::Error> {
        Bytes::try_from_val(env, self)
    }
}

impl IntoVal<Env, Bytes> for &str {
    fn into_val(self, env: &Env) -> Bytes {
        Bytes::from_slice(env, self.as_bytes())
    }
}

impl IntoVal<Env, Bytes> for &[u8] {
    fn into_val(self, env: &Env) -> Bytes {
        Bytes::from_slice(env, self)
    }
}

impl<const N: usize> IntoVal<Env, Bytes> for [u8; N] {
    fn into_val(self, env: &Env) -> Bytes {
        Bytes::from_array(env, &self)
    }
}

impl<const N: usize> IntoVal<Env, Bytes> for &[u8; N] {
    fn into_val(self, env: &Env) -> Bytes {
        Bytes::from_array(env, self)
    }
}

impl Bytes {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: Object) -> Self {
        Self { env, obj }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_raw(&self) -> &RawVal {
        self.obj.as_raw()
    }

    pub fn to_raw(&self) -> RawVal {
        self.obj.to_raw()
    }

    pub fn as_object(&self) -> &Object {
        &self.obj
    }

    pub fn to_object(&self) -> Object {
        self.obj
    }

    /// Create an empty Bytes.
    #[inline(always)]
    pub fn new(env: &Env) -> Bytes {
        let obj = env.bytes_new();
        unsafe { Self::unchecked_new(env.clone(), obj) }
    }

    /// Create a Bytes from the given `[u8]`.
    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: &[u8; N]) -> Bytes {
        Self::from_slice(env, items)
    }

    #[inline(always)]
    pub fn from_slice(env: &Env, items: &[u8]) -> Bytes {
        Bytes {
            env: env.clone(),
            obj: env.bytes_new_from_slice(items).unwrap_optimized(),
        }
    }

    #[inline(always)]
    pub fn set(&mut self, i: u32, v: u8) {
        let v32: u32 = v.into();
        self.obj = self.env().bytes_put(self.obj, i.into(), v32.into())
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
        let res = self
            .env()
            .bytes_get(self.obj, i.into())
            .try_into()
            .unwrap_optimized();
        let res32: u32 = unsafe { <_ as RawValConvertible>::unchecked_from_val(res) };
        res32 as u8
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.env().bytes_len(self.obj).is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let len = self.env().bytes_len(self.obj);
        unsafe { <_ as RawValConvertible>::unchecked_from_val(len) }
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
        let res = self.env().bytes_front(self.obj);
        let res32: u32 = unsafe { <_ as RawValConvertible>::unchecked_from_val(res) };
        res32 as u8
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
        let res = self.env().bytes_back(self.obj);
        let res32: u32 = unsafe { <_ as RawValConvertible>::unchecked_from_val(res) };
        res32 as u8
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
        self.obj = self.env().bytes_del(self.obj, i.into())
    }

    #[inline(always)]
    pub fn push(&mut self, x: u8) {
        let x32: u32 = x.into();
        self.obj = self.env().bytes_push(self.obj, x32.into())
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u8> {
        let last = self.last()?;
        self.obj = self.env().bytes_pop(self.obj);
        Some(last)
    }

    #[inline(always)]
    pub fn pop_unchecked(&mut self) -> u8 {
        let last = self.last_unchecked();
        self.obj = self.env().bytes_pop(self.obj);
        last
    }

    /// Insert the byte into this [Bytes] at position indicated by `i`, and
    /// growing the size of [Bytes] by 1.
    ///
    /// ### Panics
    ///
    /// When `i` is greater than the length of [Bytes].
    #[inline(always)]
    pub fn insert(&mut self, i: u32, b: u8) {
        let b32: u32 = b.into();
        self.obj = self.env().bytes_insert(self.obj, i.into(), b32.into())
    }

    /// Insert the bytes in `bytes` into this [Bytes] starting at position
    /// indicated by `i`, and growing the size of [Bytes] by the length of
    /// `bytes.
    ///
    /// ### Panics
    ///
    /// When `i` is greater than the length of [Bytes].
    #[inline(always)]
    pub fn insert_from_bytes(&mut self, i: u32, bytes: Bytes) {
        let mut result = self.slice(..i);
        result.append(&bytes);
        result.append(&self.slice(i..));
        *self = result
    }

    /// Insert the bytes in `array` into this [Bytes] starting at position
    /// indicated by `i`, and growing the size of [Bytes] by the length of
    /// `bytes.
    ///
    /// ### Panics
    ///
    /// When `i` is greater than the length of [Bytes].
    #[inline(always)]
    pub fn insert_from_array<const N: usize>(&mut self, i: u32, array: &[u8; N]) {
        self.insert_from_slice(i, array)
    }

    /// Insert the bytes in `slice` into this [Bytes] starting at position
    /// indicated by `i`, and growing the size of [Bytes] by the length of
    /// `bytes.
    ///
    /// ### Panics
    ///
    /// When `i` is greater than the length of [Bytes].
    #[inline(always)]
    pub fn insert_from_slice(&mut self, i: u32, slice: &[u8]) {
        self.insert_from_bytes(i, Bytes::from_slice(self.env(), slice))
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Bytes) {
        self.obj = self.env().bytes_append(self.obj, other.obj)
    }

    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, array: &[u8; N]) {
        self.extend_from_slice(array)
    }

    #[inline(always)]
    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.obj = self
            .env()
            .bytes_copy_from_slice(self.to_object(), self.len().into(), slice)
            .unwrap_optimized()
    }

    /// Copy the bytes from slice into [Bytes].
    ///
    /// The full number of bytes in slice are always copied and [Bytes] is grown
    /// if necessary.
    #[inline(always)]
    pub fn copy_from_slice(&mut self, i: u32, slice: &[u8]) {
        self.obj = self
            .env()
            .bytes_copy_from_slice(self.to_object(), i.into(), slice)
            .unwrap_optimized()
    }

    /// Copy the bytes in [Bytes] into the given slice.
    ///
    /// The minimum number of bytes are copied to either exhaust [Bytes] or fill
    /// slice.
    #[inline(always)]
    pub fn copy_into_slice(&self, slice: &mut [u8]) {
        let env = self.env();
        env.bytes_copy_to_slice(self.to_object(), RawVal::U32_ZERO, slice)
            .unwrap_optimized();
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
        let bin = env.bytes_slice(self.obj, start_bound.into(), end_bound.into());
        unsafe { Self::unchecked_new(env.clone(), bin) }
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
        if self.0.is_empty() {
            None
        } else {
            let val = self.0.env().bytes_front(self.0.obj);
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
            let val = self.0.env().bytes_back(self.0.obj);
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
/// Bytes values can be stored as [Storage], or in other types like [Vec], [Map],
/// etc.
///
/// ### Examples
///
/// BytesN values can be created from arrays:
/// ```
/// use soroban_sdk::{Bytes, BytesN, Env};
///
/// let env = Env::default();
/// let bytes = BytesN::from_array(&env, &[0; 32]);
/// assert_eq!(bytes.len(), 32);
/// ```
///
/// BytesN and Bytes values are convertible:
/// ```
/// use soroban_sdk::{Bytes, BytesN, Env};
///
/// let env = Env::default();
/// let bytes = Bytes::from_slice(&env, &[0; 32]);
/// let bytes: BytesN<32> = bytes.try_into().expect("bytes to have length 32");
/// assert_eq!(bytes.len(), 32);
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

impl<const N: usize> PartialEq<[u8; N]> for BytesN<N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        let other: BytesN<N> = other.into_val(self.env());
        self.eq(&other)
    }
}

impl<const N: usize> PartialEq<BytesN<N>> for [u8; N] {
    fn eq(&self, other: &BytesN<N>) -> bool {
        let self_: BytesN<N> = self.into_val(other.env());
        self_.eq(other)
    }
}

impl<const N: usize> PartialOrd for BytesN<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<const N: usize> PartialOrd<[u8; N]> for BytesN<N> {
    fn partial_cmp(&self, other: &[u8; N]) -> Option<Ordering> {
        let other: BytesN<N> = other.into_val(self.env());
        self.partial_cmp(&other)
    }
}

impl<const N: usize> PartialOrd<BytesN<N>> for [u8; N] {
    fn partial_cmp(&self, other: &BytesN<N>) -> Option<Ordering> {
        let self_: BytesN<N> = self.into_val(other.env());
        self_.partial_cmp(other)
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

impl<const N: usize> IntoVal<Env, BytesN<N>> for BytesN<N> {
    fn into_val(self, _env: &Env) -> BytesN<N> {
        self
    }
}

impl<const N: usize> IntoVal<Env, BytesN<N>> for &BytesN<N> {
    fn into_val(self, _env: &Env) -> BytesN<N> {
        self.clone()
    }
}

impl<const N: usize> IntoVal<Env, Bytes> for BytesN<N> {
    fn into_val(self, _env: &Env) -> Bytes {
        self.0
    }
}

impl<const N: usize> IntoVal<Env, Bytes> for &BytesN<N> {
    fn into_val(self, _env: &Env) -> Bytes {
        self.0.clone()
    }
}

impl<const N: usize> IntoVal<Env, BytesN<N>> for [u8; N] {
    fn into_val(self, env: &Env) -> BytesN<N> {
        BytesN::from_array(env, &self)
    }
}

impl<const N: usize> IntoVal<Env, BytesN<N>> for &[u8; N] {
    fn into_val(self, env: &Env) -> BytesN<N> {
        BytesN::from_array(env, self)
    }
}

impl<const N: usize> IntoVal<Env, [u8; N]> for BytesN<N> {
    fn into_val(self, _env: &Env) -> [u8; N] {
        self.to_array()
    }
}

impl<const N: usize> IntoVal<Env, [u8; N]> for &BytesN<N> {
    fn into_val(self, _env: &Env) -> [u8; N] {
        self.to_array()
    }
}

impl<const N: usize> TryFromVal<Env, Object> for BytesN<N> {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: Object) -> Result<Self, Self::Error> {
        Bytes::try_from_val(env, val)?.try_into()
    }
}

impl<const N: usize> TryFromVal<Env, RawVal> for BytesN<N> {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: RawVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, val.try_into()?)
    }
}

impl<const N: usize> TryIntoVal<Env, BytesN<N>> for Object {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<BytesN<N>, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl<const N: usize> TryIntoVal<Env, BytesN<N>> for RawVal {
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<BytesN<N>, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl<const N: usize> IntoVal<Env, RawVal> for BytesN<N> {
    fn into_val(self, _env: &Env) -> RawVal {
        self.into()
    }
}

impl<const N: usize> IntoVal<Env, RawVal> for &BytesN<N> {
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
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

impl<const N: usize> TryFrom<&Bytes> for BytesN<N> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(bin: &Bytes) -> Result<Self, Self::Error> {
        bin.clone().try_into()
    }
}

impl<const N: usize> From<BytesN<N>> for RawVal {
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

impl<const N: usize> From<&BytesN<N>> for Bytes {
    #[inline(always)]
    fn from(v: &BytesN<N>) -> Self {
        v.0.clone()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryFrom<&BytesN<N>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &BytesN<N>) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.0.env, v.0.obj.to_raw())
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
impl<const N: usize> TryFromVal<Env, ScVal> for BytesN<N> {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(
            env,
            val.try_into_val(env).map_err(|_| ConversionError)?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl<const N: usize> TryIntoVal<Env, BytesN<N>> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<BytesN<N>, Self::Error> {
        BytesN::try_from_val(env, self)
    }
}

impl<const N: usize> BytesN<N> {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: Object) -> Self {
        Self(Bytes::unchecked_new(env, obj))
    }

    pub fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub fn as_object(&self) -> &Object {
        self.0.as_object()
    }

    pub fn to_object(&self) -> Object {
        self.0.to_object()
    }

    #[inline(always)]
    pub fn from_array(env: &Env, items: &[u8; N]) -> BytesN<N> {
        BytesN(Bytes::from_slice(env, items))
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

    /// Copy the bytes in [BytesN] into the given slice.
    ///
    /// The minimum number of bytes are copied to either exhaust [BytesN] or
    /// fill slice.
    #[inline(always)]
    pub fn copy_into_slice(&self, slice: &mut [u8]) {
        let env = self.env();
        env.bytes_copy_to_slice(self.to_object(), RawVal::U32_ZERO, slice)
            .unwrap_optimized();
    }

    /// Copy the bytes in [BytesN] into an array.
    #[inline(always)]
    pub fn to_array(&self) -> [u8; N] {
        let mut array = [0u8; N];
        self.copy_into_slice(&mut array);
        array
    }

    pub fn iter(&self) -> BinIter {
        self.clone().into_iter()
    }
}

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl<const N: usize> crate::testutils::BytesN<N> for BytesN<N> {
    fn random(env: &Env) -> BytesN<N> {
        BytesN::from_array(env, &crate::testutils::random())
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

impl<const N: usize> TryFrom<&Bytes> for [u8; N] {
    type Error = ConversionError;

    fn try_from(bin: &Bytes) -> Result<Self, Self::Error> {
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

impl<const N: usize> From<&BytesN<N>> for [u8; N] {
    fn from(bin: &BytesN<N>) -> Self {
        let mut res = [0u8; N];
        for (i, b) in bin.iter().enumerate() {
            res[i] = b;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bytes_from_and_to_slices() {
        let env = Env::default();

        let b = Bytes::from_slice(&env, &[1, 2, 3, 4]);
        let mut out = [0u8; 4];
        b.copy_into_slice(&mut out);
        assert_eq!([1, 2, 3, 4], out);

        let mut b = Bytes::from_slice(&env, &[1, 2, 3, 4]);
        b.extend_from_slice(&[5, 6, 7, 8]);
        b.insert_from_slice(1, &[9, 10]);
        b.insert_from_bytes(4, Bytes::from_slice(&env, &[0, 0]));
        let mut out = [0u8; 12];
        b.copy_into_slice(&mut out);
        assert_eq!([1, 9, 10, 2, 0, 0, 3, 4, 5, 6, 7, 8], out);
        b.copy_from_slice(3, &[7, 6, 5]);
        b.copy_into_slice(&mut out);
        assert_eq!([1, 9, 10, 7, 6, 5, 3, 4, 5, 6, 7, 8], out);
    }

    #[test]
    fn bytesn_from_and_to_slices() {
        let env = Env::default();

        let b = BytesN::from_array(&env, &[1, 2, 3, 4]);
        let mut out = [0u8; 4];
        b.copy_into_slice(&mut out);
        assert_eq!([1, 2, 3, 4], out);
    }

    #[test]
    fn macro_bytes() {
        let env = Env::default();
        assert_eq!(bytes!(&env), Bytes::new(&env));
        assert_eq!(bytes!(&env, 1), {
            let mut b = Bytes::new(&env);
            b.push(1);
            b
        });
        assert_eq!(bytes!(&env, 1,), {
            let mut b = Bytes::new(&env);
            b.push(1);
            b
        });
        assert_eq!(bytes!(&env, [3, 2, 1,]), {
            let mut b = Bytes::new(&env);
            b.push(3);
            b.push(2);
            b.push(1);
            b
        });
    }

    #[test]
    fn macro_bytes_hex() {
        let env = Env::default();
        assert_eq!(bytes!(&env), Bytes::new(&env));
        assert_eq!(bytes!(&env, 1), {
            let mut b = Bytes::new(&env);
            b.push(1);
            b
        });
        assert_eq!(bytes!(&env, 1,), {
            let mut b = Bytes::new(&env);
            b.push(1);
            b
        });
        assert_eq!(bytes!(&env, 0x30201), {
            let mut b = Bytes::new(&env);
            b.push(3);
            b.push(2);
            b.push(1);
            b
        });
        assert_eq!(bytes!(&env, 0x0000030201), {
            Bytes::from_array(&env, &[0, 0, 3, 2, 1])
        });
    }

    #[test]
    fn macro_bytesn() {
        let env = Env::default();
        assert_eq!(bytesn!(&env, 1), { BytesN::from_array(&env, &[1]) });
        assert_eq!(bytesn!(&env, 1,), { BytesN::from_array(&env, &[1]) });
        assert_eq!(bytesn!(&env, [3, 2, 1,]), {
            BytesN::from_array(&env, &[3, 2, 1])
        });
    }

    #[test]
    fn macro_bytesn_hex() {
        let env = Env::default();
        assert_eq!(bytesn!(&env, 0x030201), {
            BytesN::from_array(&env, &[3, 2, 1])
        });
        assert_eq!(bytesn!(&env, 0x0000030201), {
            BytesN::from_array(&env, &[0, 0, 3, 2, 1])
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
