use core::{
    borrow::Borrow,
    cmp::Ordering,
    convert::Infallible,
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

use crate::{
    iter::{UncheckedEnumerable, UncheckedIter},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
};

use super::{
    env::internal::{Env as _, EnvBase as _, VecObject},
    ConversionError, Env, IntoVal, RawVal, Set, TryFromVal, TryIntoVal,
};

#[cfg(doc)]
use crate::{storage::Storage, Bytes, BytesN, Map};

/// Create a [Vec] with the given items.
///
/// The first argument in the list must be a reference to an [Env], then the
/// items follow.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, vec};
///
/// let env = Env::default();
/// let vec = vec![&env, 0, 1, 2, 3];
/// assert_eq!(vec.len(), 4);
/// ```
#[macro_export]
macro_rules! vec {
    ($env:expr $(,)?) => {
        $crate::Vec::new($env)
    };
    ($env:expr, $($x:expr),+ $(,)?) => {
        $crate::Vec::from_array($env, [$($x),+])
    };
}

macro_rules! impl_into_vec_for_tuple {
    ( $($typ:ident $idx:tt)* ) => {
        impl<$($typ),*> TryFromVal<Env, ($($typ,)*)> for Vec<RawVal>
        where
            $($typ: IntoVal<Env, RawVal>),*
        {
            type Error = ConversionError;
            fn try_from_val(env: &Env, v: &($($typ,)*)) -> Result<Self, Self::Error> {
                Ok(vec![&env, $(v.$idx.into_val(env), )*])
            }
        }
    };
}
impl_into_vec_for_tuple! { T0 0 }
impl_into_vec_for_tuple! { T0 0 T1 1 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 }
impl_into_vec_for_tuple! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10 T11 11 T12 12 }

/// Vec is a sequential and indexable growable collection type.
///
/// Values are stored in the environment and are available to contract through
/// the functions defined on Vec.  Values stored in the Vec are transmitted to
/// the environment as [RawVal]s, and when retrieved from the Vec are
/// transmitted back and converted from [RawVal] back into their type.
///
/// The values in a Vec are not guaranteed to be of type `T` and conversion will
/// fail if they are not. Most functions on Vec have a `try_` variation that
/// returns a `Result` that will be `Err` if the conversion fails. Functions
/// that are not prefixed with `try_` will panic if conversion fails.
///
/// Functions with an `_unchecked` suffix will panic if called with indexes that
/// are out-of-bounds.
///
/// To store `u8`s and binary data, use [Bytes]/[BytesN] instead.
///
/// Vec values can be stored as [Storage], or in other types like [Vec], [Map],
/// etc.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{vec, Env};
///
/// let env = Env::default();
/// let vec = vec![&env, 0, 1, 2, 3];
/// assert_eq!(vec.len(), 4);
/// ```
///
/// ### Implementation Details
///
/// The following information is implementation details that has no consistency
/// guarantee.
///
/// Values are stored in the environment backed by a [RRB-vector][rrb] using
/// [im_rc::Vector]. Most operations are O(log n). Push/pop are O(1)
/// amortised, and O(log n) in the worst case.
///
/// [im_rc::Vector]: https://docs.rs/im-rc/latest/im_rc/struct.Vector.html
/// [rrb]: https://infoscience.epfl.ch/record/213452/files/rrbvector.pdf
#[derive(Clone)]
pub struct Vec<T> {
    env: Env,
    obj: VecObject,
    _t: PhantomData<T>,
}

impl<T> Eq for Vec<T> where T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}

impl<T> PartialEq for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<T> PartialOrd for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T> Ord for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.env.check_same_env(&other.env);
        let v = self
            .env
            .obj_cmp(self.obj.to_raw(), other.obj.to_raw())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl<T> Debug for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Debug + Clone,
    T::Error: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec(")?;
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

impl<T> TryFromVal<Env, Vec<T>> for Vec<RawVal> {
    type Error = Infallible;

    fn try_from_val(env: &Env, v: &Vec<T>) -> Result<Self, Self::Error> {
        Ok(unsafe { Vec::unchecked_new(env.clone(), v.obj.clone()) })
    }
}

// This conflicts with the previous definition unless we add the spurious &,
// which is not .. great. Maybe don't define this particular blanket, or add
// a to_other<T>() method?
impl<T> TryFromVal<Env, &Vec<RawVal>> for Vec<T> {
    type Error = Infallible;

    fn try_from_val(env: &Env, v: &&Vec<RawVal>) -> Result<Self, Self::Error> {
        Ok(unsafe { Vec::unchecked_new(env.clone(), v.obj.clone()) })
    }
}

impl<T> TryFromVal<Env, VecObject> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = Infallible;

    #[inline(always)]
    fn try_from_val(env: &Env, obj: &VecObject) -> Result<Self, Self::Error> {
        Ok(unsafe { Vec::<T>::unchecked_new(env.clone(), obj.clone()) })
    }
}

impl<T> TryFromVal<Env, RawVal> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(VecObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl<T> TryFromVal<Env, Vec<T>> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Vec<T>) -> Result<Self, Self::Error> {
        Ok(v.to_raw())
    }
}

impl<T> From<Vec<T>> for RawVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.obj.into()
    }
}

impl<T> From<Vec<T>> for VecObject
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.obj
    }
}

impl<T> From<Vec<T>> for Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn from(v: Vec<T>) -> Self {
        let mut s: Set<T> = Set::new(v.env());
        for i in v.into_iter().flatten() {
            s.insert(i);
        }
        s
    }
}

#[cfg(not(target_family = "wasm"))]
use super::xdr::{ScVal, ScVec};

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, ConversionError> {
        ScVal::try_from_val(&v.env, &v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScVec {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, ConversionError> {
        if let ScVal::Vec(Some(vec)) = ScVal::try_from(v)? {
            Ok(vec)
        } else {
            Err(ConversionError)
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<Vec<T>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Vec<T>) -> Result<Self, ConversionError> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<Vec<T>> for ScVec {
    type Error = ConversionError;
    fn try_from(v: Vec<T>) -> Result<Self, ConversionError> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScVal> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, ConversionError> {
        Ok(
            VecObject::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScVec> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVec) -> Result<Self, Self::Error> {
        ScVal::Vec(Some(val.clone())).try_into_val(env)
    }
}

impl<T> Vec<T> {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: VecObject) -> Self {
        Self {
            env,
            obj,
            _t: PhantomData,
        }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_raw(&self) -> &RawVal {
        self.obj.as_raw()
    }

    pub fn to_raw(&self) -> RawVal {
        self.obj.to_raw()
    }

    pub fn as_object(&self) -> &VecObject {
        &self.obj
    }

    pub fn to_object(&self) -> VecObject {
        self.obj
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    /// Create an empty Vec.
    #[inline(always)]
    pub fn new(env: &Env) -> Vec<T> {
        unsafe { Self::unchecked_new(env.clone(), env.vec_new(().into()).unwrap_infallible()) }
    }

    /// Create a Vec from the array of items.
    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [T; N]) -> Vec<T> {
        let mut tmp: [RawVal; N] = [RawVal::VOID.to_raw(); N];
        for (dst, src) in tmp.iter_mut().zip(items.iter()) {
            *dst = src.into_val(env)
        }
        let vec = env.vec_new_from_slice(&tmp).unwrap_infallible();
        unsafe { Self::unchecked_new(env.clone(), vec) }
    }

    /// Create a Vec from the slice of items.
    #[inline(always)]
    pub fn from_slice(env: &Env, items: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::new(env);
        vec.extend_from_slice(items);
        vec
    }

    /// Returns the item at the position or None if out-of-bounds.
    ///
    /// ### Panics
    ///
    /// If the value at the position cannot be converted to type T.
    #[inline(always)]
    pub fn get(&self, i: u32) -> Option<T> {
        self.try_get(i).unwrap_optimized()
    }

    /// Returns the item at the position or None if out-of-bounds.
    ///
    /// ### Errors
    ///
    /// If the value at the position cannot be converted to type T.
    #[inline(always)]
    pub fn try_get(&self, i: u32) -> Result<Option<T>, T::Error> {
        if i < self.len() {
            self.try_get_unchecked(i).map(|val| Some(val))
        } else {
            Ok(None)
        }
    }

    /// Returns the item at the position.
    ///
    /// ### Panics
    ///
    /// If the position is out-of-bounds.
    ///
    /// If the value at the position cannot be converted to type T.
    #[inline(always)]
    pub fn get_unchecked(&self, i: u32) -> T {
        self.try_get_unchecked(i).unwrap_optimized()
    }

    /// Returns the item at the position.
    ///
    /// ### Errors
    ///
    /// If the value at the position cannot be converted to type T.
    ///
    /// ### Panics
    ///
    /// If the position is out-of-bounds.
    #[inline(always)]
    pub fn try_get_unchecked(&self, i: u32) -> Result<T, T::Error> {
        let env = self.env();
        let val = env.vec_get(self.obj, i.into()).unwrap_infallible();
        T::try_from_val(env, &val)
    }

    #[inline(always)]
    pub fn set(&mut self, i: u32, v: T) {
        let env = self.env();
        self.obj = env
            .vec_put(self.obj, i.into(), v.into_val(env))
            .unwrap_infallible();
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
        self.obj = env.vec_del(self.obj, i.into()).unwrap_infallible();
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env.vec_len(self.obj).unwrap_infallible().into()
    }

    #[inline(always)]
    pub fn push_front(&mut self, x: T) {
        let env = self.env();
        self.obj = env
            .vec_push_front(self.obj, x.into_val(env))
            .unwrap_infallible();
    }

    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<Result<T, T::Error>> {
        let last = self.first()?;
        let env = self.env();
        self.obj = env.vec_pop_front(self.obj).unwrap_infallible();
        Some(last)
    }

    #[inline(always)]
    pub fn pop_front_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.first_unchecked();
        let env = self.env();
        self.obj = env.vec_pop_front(self.obj).unwrap_infallible();
        last
    }

    #[inline(always)]
    pub fn push_back(&mut self, x: T) {
        let env = self.env();
        self.obj = env
            .vec_push_back(self.obj, x.into_val(env))
            .unwrap_infallible();
    }

    #[inline(always)]
    pub fn pop_back(&mut self) -> Option<Result<T, T::Error>> {
        let last = self.last()?;
        let env = self.env();
        self.obj = env.vec_pop_back(self.obj).unwrap_infallible();
        Some(last)
    }

    #[inline(always)]
    pub fn pop_back_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.last_unchecked();
        let env = self.env();
        self.obj = env.vec_pop_back(self.obj).unwrap_infallible();
        last
    }

    #[deprecated(note = "use [Vec::push_back]")]
    #[inline(always)]
    pub fn push(&mut self, x: T) {
        self.push_back(x)
    }

    #[deprecated(note = "use [Vec::pop_back]")]
    #[inline(always)]
    pub fn pop(&mut self) -> Option<Result<T, T::Error>> {
        self.pop_back()
    }

    #[deprecated(note = "use [Vec::push_back_unchecked]")]
    #[inline(always)]
    pub fn pop_unchecked(&mut self) -> Result<T, T::Error> {
        self.pop_back_unchecked()
    }

    #[inline(always)]
    pub fn first(&self) -> Option<Result<T, T::Error>> {
        if self.is_empty() {
            None
        } else {
            let env = &self.env;
            let val = env.vec_front(self.obj).unwrap_infallible();
            Some(T::try_from_val(env, &val))
        }
    }

    #[inline(always)]
    pub fn first_unchecked(&self) -> Result<T, T::Error> {
        let env = &self.env;
        let val = env.vec_front(self.obj).unwrap_infallible();
        T::try_from_val(env, &val)
    }

    #[inline(always)]
    pub fn last(&self) -> Option<Result<T, T::Error>> {
        if self.is_empty() {
            None
        } else {
            let env = self.env();
            let val = env.vec_back(self.obj).unwrap_infallible();
            Some(T::try_from_val(env, &val))
        }
    }

    #[inline(always)]
    pub fn last_unchecked(&self) -> Result<T, T::Error> {
        let env = self.env();
        let val = env.vec_back(self.obj).unwrap_infallible();
        T::try_from_val(env, &val)
    }

    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: T) {
        let env = self.env();
        self.obj = env
            .vec_insert(self.obj, i.into(), x.into_val(env))
            .unwrap_infallible();
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Vec<T>) {
        let env = self.env();
        self.obj = env.vec_append(self.obj, other.obj).unwrap_infallible();
    }

    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, items: [T; N]) {
        self.append(&Self::from_array(&self.env, items))
    }

    #[inline(always)]
    pub fn extend_from_slice(&mut self, items: &[T])
    where
        T: Clone,
    {
        for item in items {
            self.push_back(item.clone());
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
        let obj = env
            .vec_slice(self.obj, start_bound.into(), end_bound.into())
            .unwrap_infallible();
        unsafe { Self::unchecked_new(env.clone(), obj) }
    }

    pub fn iter(&self) -> VecIter<T>
    where
        T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
    {
        self.clone().into_iter()
    }

    #[inline(always)]
    pub fn iter_unchecked(&self) -> UncheckedIter<VecIter<T>, T, T::Error>
    where
        T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
        T::Error: Debug,
    {
        self.iter().unchecked()
    }

    #[inline(always)]
    pub fn into_iter_unchecked(self) -> UncheckedIter<VecIter<T>, T, T::Error>
    where
        T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
        T::Error: Debug,
    {
        self.into_iter().unchecked()
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, RawVal>,
{
    /// Returns true if the Vec contains the item.
    #[inline(always)]
    pub fn contains(&self, item: impl Borrow<T>) -> bool {
        let env = self.env();
        let val = item.borrow().into_val(env);
        !env.vec_first_index_of(self.obj, val)
            .unwrap_infallible()
            .is_void()
    }

    /// Returns the index of the first occurrence of the item.
    ///
    /// If the item cannot be found [None] is returned.
    #[inline(always)]
    pub fn first_index_of(&self, item: impl Borrow<T>) -> Option<u32> {
        let env = self.env();
        let val = item.borrow().into_val(env);
        env.vec_first_index_of(self.obj, val)
            .unwrap_infallible()
            .try_into_val(env)
            .unwrap()
    }

    /// Returns the index of the last occurrence of the item.
    ///
    /// If the item cannot be found [None] is returned.
    #[inline(always)]
    pub fn last_index_of(&self, item: impl Borrow<T>) -> Option<u32> {
        let env = self.env();
        let val = item.borrow().into_val(env);
        env.vec_last_index_of(self.obj, val)
            .unwrap_infallible()
            .try_into_val(env)
            .unwrap()
    }

    /// Returns the index of an occurrence of the item in an already sorted
    /// [Vec], or the index of where the item can be inserted to keep the [Vec]
    /// sorted.
    ///
    /// If the item is found, [Result::Ok] is returned containing the index of
    /// the item.
    ///
    /// If the item is not found, [Result::Err] is returned containing the index
    /// of where the item could be inserted to retain the sorted ordering.
    #[inline(always)]
    pub fn binary_search(&self, item: impl Borrow<T>) -> Result<u32, u32> {
        let env = self.env();
        let val = item.borrow().into_val(env);
        let high_low = env.vec_binary_search(self.obj, val).unwrap_infallible();
        let high: u32 = (high_low >> u32::BITS) as u32;
        let low: u32 = high_low as u32;
        if high == 1 {
            Ok(low)
        } else {
            Err(low)
        }
    }
}

impl<T> Vec<Vec<T>>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    T: Clone,
{
    #[inline(always)]
    pub fn concat(&self) -> Vec<T> {
        let mut concatenated = vec![self.env()];
        for vec in self.iter_unchecked() {
            concatenated.append(&vec);
        }
        concatenated
    }
}

impl<T> IntoIterator for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Item = Result<T, T::Error>;
    type IntoIter = VecIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        VecIter(self)
    }
}

#[derive(Clone)]
pub struct VecIter<T>(Vec<T>);

impl<T> VecIter<T> {
    fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> Iterator for VecIter<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Item = Result<T, T::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len == 0 {
            None
        } else {
            let val = self.0.env().vec_front(self.0.obj).unwrap_infallible();
            self.0 = self.0.slice(1..);
            Some(T::try_from_val(self.0.env(), &val))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as usize;
        (len, Some(len))
    }

    // TODO: Implement other functions as optimizations since the iterator is
    // backed by an indexable collection.
}

impl<T> DoubleEndedIterator for VecIter<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len == 0 {
            None
        } else {
            let val = self.0.env().vec_back(self.0.obj).unwrap_infallible();
            self.0 = self.0.slice(..len - 1);
            Some(T::try_from_val(self.0.env(), &val))
        }
    }

    // TODO: Implement other functions as optimizations since the iterator is
    // backed by an indexable collection.
}

impl<T> FusedIterator for VecIter<T> where T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}

impl<T> ExactSizeIterator for VecIter<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn len(&self) -> usize {
        self.0.len() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::set;

    #[test]
    fn test_vec_macro() {
        let env = Env::default();
        assert_eq!(vec![&env], Vec::<i32>::new(&env));
        assert_eq!(vec![&env,], Vec::<i32>::new(&env));
        assert_eq!(vec![&env, 1], {
            let mut v = Vec::new(&env);
            v.push_back(1);
            v
        });
        assert_eq!(vec![&env, 1,], {
            let mut v = Vec::new(&env);
            v.push_back(1);
            v
        });
        assert_eq!(vec![&env, 3, 2, 1,], {
            let mut v = Vec::new(&env);
            v.push_back(3);
            v.push_back(2);
            v.push_back(1);
            v
        });
    }

    #[test]
    fn test_vec_raw_val_type() {
        let env = Env::default();

        let mut vec = Vec::<u32>::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push_back(10);
        assert_eq!(vec.len(), 1);
        vec.push_back(20);
        assert_eq!(vec.len(), 2);
        vec.push_back(30);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert!(vec == vec_copy);
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push_back(40);
        assert_eq!(vec_copy.len(), 4);
        assert!(vec != vec_copy);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);

        _ = vec_copy.pop_back_unchecked();
        assert!(vec == vec_copy);
    }

    #[test]
    fn test_vec_env_val_type() {
        let env = Env::default();

        let mut vec = Vec::<i64>::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push_back(-10);
        assert_eq!(vec.len(), 1);
        vec.push_back(20);
        assert_eq!(vec.len(), 2);
        vec.push_back(-30);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert!(vec == vec_copy);
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push_back(40);
        assert_eq!(vec_copy.len(), 4);
        assert!(vec != vec_copy);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);

        _ = vec_copy.pop_back_unchecked();
        assert!(vec == vec_copy);
    }

    #[test]
    fn test_vec_recursive() {
        let env = Env::default();

        let mut vec_inner = Vec::<i64>::new(&env);
        vec_inner.push_back(-10);
        assert_eq!(vec_inner.len(), 1);

        let mut vec_outer = Vec::<Vec<i64>>::new(&env);
        vec_outer.push_back(vec_inner);
        assert_eq!(vec_outer.len(), 1);
    }

    #[test]
    fn test_vec_concat() {
        let env = Env::default();
        let vec_1: Vec<i64> = vec![&env, 1, 2, 3];
        let vec_2: Vec<i64> = vec![&env, 4, 5, 6];
        let vec = vec![&env, vec_1, vec_2].concat();
        assert_eq!(vec, vec![&env, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_vec_slice() {
        let env = Env::default();

        let vec = vec![&env, 0, 1, 2, 3, 4];
        assert_eq!(vec.len(), 5);

        let slice = vec.slice(..);
        assert_eq!(slice, vec![&env, 0, 1, 2, 3, 4]);

        let slice = vec.slice(0..5);
        assert_eq!(slice, vec![&env, 0, 1, 2, 3, 4]);

        let slice = vec.slice(0..=4);
        assert_eq!(slice, vec![&env, 0, 1, 2, 3, 4]);

        let slice = vec.slice(1..);
        assert_eq!(slice, vec![&env, 1, 2, 3, 4]);

        let slice = vec.slice(..4);
        assert_eq!(slice, vec![&env, 0, 1, 2, 3]);

        let slice = vec.slice(..=3);
        assert_eq!(slice, vec![&env, 0, 1, 2, 3]);

        let slice = vec.slice(1..4);
        assert_eq!(slice, vec![&env, 1, 2, 3]);

        let slice = vec.slice(1..=3);
        assert_eq!(slice, vec![&env, 1, 2, 3]);

        // An exclusive start is technically possible due to the lack of
        // constraints in the RangeBounds trait, however this is unlikely to
        // happen since no syntax shorthand exists for it.
        let slice = vec.slice((Bound::Excluded(0), Bound::Included(3)));
        assert_eq!(slice, vec![&env, 1, 2, 3]);
        let slice = vec.slice((Bound::Excluded(0), Bound::Excluded(3)));
        assert_eq!(slice, vec![&env, 1, 2]);
    }

    #[test]
    fn test_vec_iter() {
        let env = Env::default();

        let vec: Vec<()> = vec![&env];
        let mut iter = vec.iter();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let vec = vec![&env, 0, 1, 2, 3, 4];

        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(Ok(0)));
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.next(), Some(Ok(3)));
        assert_eq!(iter.next(), Some(Ok(4)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(Ok(0)));
        assert_eq!(iter.next_back(), Some(Ok(4)));
        assert_eq!(iter.next_back(), Some(Ok(3)));
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = vec.iter().rev();
        assert_eq!(iter.next(), Some(Ok(4)));
        assert_eq!(iter.next_back(), Some(Ok(0)));
        assert_eq!(iter.next_back(), Some(Ok(1)));
        assert_eq!(iter.next(), Some(Ok(3)));
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_contains() {
        let env = Env::default();
        let vec = vec![&env, 0, 3, 5, 7, 9, 5];
        assert_eq!(vec.contains(&2), false);
        assert_eq!(vec.contains(2), false);
        assert_eq!(vec.contains(&3), true);
        assert_eq!(vec.contains(3), true);
        assert_eq!(vec.contains(&5), true);
        assert_eq!(vec.contains(5), true);
    }

    #[test]
    fn test_first_index_of() {
        let env = Env::default();

        let vec = vec![&env, 0, 3, 5, 7, 9, 5];
        assert_eq!(vec.first_index_of(&2), None);
        assert_eq!(vec.first_index_of(2), None);
        assert_eq!(vec.first_index_of(&3), Some(1));
        assert_eq!(vec.first_index_of(3), Some(1));
        assert_eq!(vec.first_index_of(&5), Some(2));
        assert_eq!(vec.first_index_of(5), Some(2));
    }

    #[test]
    fn test_last_index_of() {
        let env = Env::default();

        let vec = vec![&env, 0, 3, 5, 7, 9, 5];
        assert_eq!(vec.last_index_of(&2), None);
        assert_eq!(vec.last_index_of(2), None);
        assert_eq!(vec.last_index_of(&3), Some(1));
        assert_eq!(vec.last_index_of(3), Some(1));
        assert_eq!(vec.last_index_of(&5), Some(5));
        assert_eq!(vec.last_index_of(5), Some(5));
    }

    #[test]
    fn test_binary_search() {
        let env = Env::default();

        let vec = vec![&env, 0, 3, 5, 5, 7, 9];
        assert_eq!(vec.binary_search(&2), Err(1));
        assert_eq!(vec.binary_search(2), Err(1));
        assert_eq!(vec.binary_search(&3), Ok(1));
        assert_eq!(vec.binary_search(3), Ok(1));
        assert_eq!(vec.binary_search(&5), Ok(3));
        assert_eq!(vec.binary_search(5), Ok(3));
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    fn test_scval_accessibility_from_udt_types() {
        use crate::TryFromVal;
        let env = Env::default();
        let v = vec![&env, 1];
        let val: ScVal = v.clone().try_into().unwrap();
        let roundtrip = Vec::<i64>::try_from_val(&env, &val).unwrap();
        assert_eq!(v, roundtrip);
    }

    #[test]
    fn test_vec_to_set() {
        let env = Env::default();
        let v = vec![&env, 1, 2, 3];
        let s = Set::from(v);
        assert_eq!(s, set![&env, 1, 2, 3]);

        // Ensure this also deduplicates and sorts values
        let v2 = vec![&env, 9, 9, 1, 5, 3, 7, 3, 5, 5];
        let s2 = Set::from(v2);
        assert_eq!(s2, set![&env, 1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_insert_and_set() {
        let env = Env::default();
        let mut v = Vec::<i64>::new(&env);
        v.insert(0, 3);
        v.insert(0, 1);
        v.insert(1, 4);
        v.insert(3, 6);
        assert_eq!(v, vec![&env, 1, 4, 3, 6]);
        v.set(0, 7);
        v.set(1, 6);
        v.set(2, 2);
        v.set(3, 5);
        assert_eq!(v, vec![&env, 7, 6, 2, 5]);
    }

    #[test]
    fn test_is_empty_and_len() {
        let env = Env::default();

        let mut v: Vec<i32> = vec![&env, 1, 4, 3];
        assert_eq!(v.is_empty(), false);
        assert_eq!(v.len(), 3);

        v = vec![&env];
        assert_eq!(v.is_empty(), true);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_push_pop_front() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);
        v.push_front(42);
        assert_eq!(v, vec![&env, 42]);
        v.push_front(1);
        assert_eq!(v, vec![&env, 1, 42]);
        let pop_checked = v.pop_front();
        assert_eq!(pop_checked, Some(Ok(1)));
        assert_eq!(v, vec![&env, 42]);
        let pop_unchecked = v.pop_front_unchecked();
        assert_eq!(pop_unchecked, Ok(42));
        assert_eq!(v, vec![&env]);
        assert_eq!(v.pop_front(), None);
    }

    #[test]
    fn test_push_pop_back() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);
        v.push_back(42);
        assert_eq!(v, vec![&env, 42]);
        v.push_back(1);
        assert_eq!(v, vec![&env, 42, 1]);
        let pop_checked = v.pop_back();
        assert_eq!(pop_checked, Some(Ok(1)));
        assert_eq!(v, vec![&env, 42]);
        let pop_unchecked = v.pop_back_unchecked();
        assert_eq!(pop_unchecked, Ok(42));
        assert_eq!(v, vec![&env]);
        assert_eq!(v.pop_back(), None);
    }

    #[test]
    fn test_get() {
        let env = Env::default();

        let v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];

        // get each item
        assert_eq!(v.get(3), Some(5));
        assert_eq!(v.get(0), Some(0));
        assert_eq!(v.get(1), Some(3));
        assert_eq!(v.get(2), Some(5));
        assert_eq!(v.get(5), Some(9));
        assert_eq!(v.get(4), Some(7));

        assert_eq!(v.get(v.len()), None);
        assert_eq!(v.get(v.len() + 1), None);
        assert_eq!(v.get(u32::MAX), None);

        // tests on an empty vec
        let v = Vec::<i64>::new(&env);
        assert_eq!(v.get(0), None);
        assert_eq!(v.get(v.len()), None);
        assert_eq!(v.get(v.len() + 1), None);
        assert_eq!(v.get(u32::MAX), None);
    }

    #[test]
    #[should_panic(expected = "ConversionError")]
    fn test_get_panics_on_conversion() {
        let env = Env::default();

        let v: RawVal = (1i64, 2i32).try_into_val(&env).unwrap();
        let v: Vec<i64> = v.try_into_val(&env).unwrap();

        // panic because element one is not of the expected type
        assert_eq!(v.get(1), Some(5));
    }

    #[test]
    fn test_try_get() {
        let env = Env::default();

        let v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];

        // get each item
        assert_eq!(v.try_get(3), Ok(Some(5)));
        assert_eq!(v.try_get(0), Ok(Some(0)));
        assert_eq!(v.try_get(1), Ok(Some(3)));
        assert_eq!(v.try_get(2), Ok(Some(5)));
        assert_eq!(v.try_get(5), Ok(Some(9)));
        assert_eq!(v.try_get(4), Ok(Some(7)));

        assert_eq!(v.try_get(v.len()), Ok(None));
        assert_eq!(v.try_get(v.len() + 1), Ok(None));
        assert_eq!(v.try_get(u32::MAX), Ok(None));

        // tests on an empty vec
        let v = Vec::<i64>::new(&env);
        assert_eq!(v.try_get(0), Ok(None));
        assert_eq!(v.try_get(v.len()), Ok(None));
        assert_eq!(v.try_get(v.len() + 1), Ok(None));
        assert_eq!(v.try_get(u32::MAX), Ok(None));

        // errors
        let v: RawVal = (1i64, 2i32).try_into_val(&env).unwrap();
        let v: Vec<i64> = v.try_into_val(&env).unwrap();
        assert_eq!(v.try_get(0), Ok(Some(1)));
        assert_eq!(v.try_get(1), Err(ConversionError));
    }

    #[test]
    fn test_get_unchecked() {
        let env = Env::default();

        let v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];

        // get each item
        assert_eq!(v.get_unchecked(3), 5);
        assert_eq!(v.get_unchecked(0), 0);
        assert_eq!(v.get_unchecked(1), 3);
        assert_eq!(v.get_unchecked(2), 5);
        assert_eq!(v.get_unchecked(5), 9);
        assert_eq!(v.get_unchecked(4), 7);
    }

    #[test]
    #[should_panic(expected = "ConversionError")]
    fn test_get_unchecked_panics_on_conversion() {
        let env = Env::default();

        let v: RawVal = (1i64, 2i32).try_into_val(&env).unwrap();
        let v: Vec<i64> = v.try_into_val(&env).unwrap();

        // panic because element one is not of the expected type
        v.get_unchecked(1);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_get_unchecked_panics_on_out_of_bounds() {
        let env = Env::default();

        let v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];
        _ = v.get_unchecked(v.len()); // out of bound get
    }

    #[test]
    fn test_try_get_unchecked() {
        let env = Env::default();

        let v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];

        // get each item
        assert_eq!(v.try_get_unchecked(3), Ok(5));
        assert_eq!(v.try_get_unchecked(0), Ok(0));
        assert_eq!(v.try_get_unchecked(1), Ok(3));
        assert_eq!(v.try_get_unchecked(2), Ok(5));
        assert_eq!(v.try_get_unchecked(5), Ok(9));
        assert_eq!(v.try_get_unchecked(4), Ok(7));

        // errors
        let v: RawVal = (1i64, 2i32).try_into_val(&env).unwrap();
        let v: Vec<i64> = v.try_into_val(&env).unwrap();
        assert_eq!(v.try_get_unchecked(0), Ok(1));
        assert_eq!(v.try_get_unchecked(1), Err(ConversionError));
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_try_get_unchecked_panics() {
        let env = Env::default();

        let v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];
        _ = v.try_get_unchecked(v.len()); // out of bound get
    }

    #[test]
    fn test_remove() {
        let env = Env::default();
        let mut v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];

        assert_eq!(v.remove(0), Some(()));
        assert_eq!(v.remove(2), Some(()));
        assert_eq!(v.remove(3), Some(()));

        assert_eq!(v, vec![&env, 3, 5, 7]);
        assert_eq!(v.len(), 3);

        // out of bound removes
        assert_eq!(v.remove(v.len()), None);
        assert_eq!(v.remove(v.len() + 1), None);
        assert_eq!(v.remove(u32::MAX), None);

        // remove rest of items
        assert_eq!(v.remove(0), Some(()));
        assert_eq!(v.remove(0), Some(()));
        assert_eq!(v.remove(0), Some(()));
        assert_eq!(v, vec![&env]);
        assert_eq!(v.len(), 0);

        // try remove from empty vec
        assert_eq!(v.remove(0), None);
        assert_eq!(v.remove(v.len()), None);
        assert_eq!(v.remove(v.len() + 1), None);
        assert_eq!(v.remove(u32::MAX), None);
    }

    #[test]
    fn test_remove_unchecked() {
        let env = Env::default();
        let mut v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];

        assert_eq!(v.remove_unchecked(0), ());
        assert_eq!(v.remove_unchecked(2), ());
        assert_eq!(v.remove_unchecked(3), ());

        assert_eq!(v, vec![&env, 3, 5, 7]);
        assert_eq!(v.len(), 3);

        // remove rest of items
        assert_eq!(v.remove_unchecked(0), ());
        assert_eq!(v.remove_unchecked(0), ());
        assert_eq!(v.remove_unchecked(0), ());
        assert_eq!(v, vec![&env]);
        assert_eq!(v.len(), 0);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_remove_unchecked_panics() {
        let env = Env::default();
        let mut v: Vec<i64> = vec![&env, 0, 3, 5, 5, 7, 9];
        v.remove_unchecked(v.len())
    }
}
