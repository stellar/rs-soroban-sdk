use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

use crate::iter::{UncheckedEnumerable, UncheckedIter};

use super::{
    env::{internal::Env as _, internal::EnvBase as _, RawValConvertible},
    xdr::ScObjectType,
    ConversionError, Env, IntoVal, Object, RawVal, Set, TryFromVal, TryIntoVal,
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
        impl<$($typ),*> IntoVal<Env, Vec<RawVal>> for ($($typ,)*)
        where
            $($typ: IntoVal<Env, RawVal>),*
        {
            fn into_val(self, env: &Env) -> Vec<RawVal> {
                vec![&env, $(self.$idx.into_val(env), )*]
            }
        }

        impl<$($typ),*> IntoVal<Env, Vec<RawVal>> for &($($typ,)*)
        where
            $(for <'a> &'a $typ: IntoVal<Env, RawVal>),*
        {
            fn into_val(self, env: &Env) -> Vec<RawVal> {
                vec![&env, $((&self.$idx).into_val(env), )*]
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
/// fail if they are not. Most functions on Vec return a `Result` due to this.
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
    obj: Object,
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
        let v = self.env.obj_cmp(self.obj.to_raw(), other.obj.to_raw());
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

impl<T> IntoVal<Env, Vec<RawVal>> for Vec<T> {
    fn into_val(self, _env: &Env) -> Vec<RawVal> {
        unsafe { Vec::unchecked_new(self.env, self.obj) }
    }
}

impl<T> IntoVal<Env, Vec<RawVal>> for &Vec<T> {
    fn into_val(self, _env: &Env) -> Vec<RawVal> {
        unsafe { Vec::unchecked_new(self.env.clone(), self.obj.clone()) }
    }
}

impl<T> TryFromVal<Env, Object> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from_val(env: &Env, obj: Object) -> Result<Self, Self::Error> {
        if obj.is_obj_type(ScObjectType::Vec) {
            Ok(unsafe { Vec::<T>::unchecked_new(env.clone(), obj) })
        } else {
            Err(ConversionError {})
        }
    }
}

impl<T> TryFromVal<Env, RawVal> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = <Vec<T> as TryFromVal<Env, Object>>::Error;

    #[inline(always)]
    fn try_from_val(env: &Env, val: RawVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, val.try_into()?)
    }
}

impl<T> TryIntoVal<Env, Vec<T>> for Object
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl<T> TryIntoVal<Env, Vec<T>> for RawVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        <_ as TryFromVal<_, _>>::try_from_val(env, self)
    }
}

impl<T> IntoVal<Env, RawVal> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn into_val(self, _env: &Env) -> RawVal {
        self.into()
    }
}

impl<T> IntoVal<Env, RawVal> for &Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
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

impl<T> IntoVal<Env, Object> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn into_val(self, _env: &Env) -> Object {
        self.into()
    }
}

impl<T> From<Vec<T>> for Object
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
use super::xdr::{ScObject, ScVal, ScVec};

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.env, v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScObject {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        let v: ScVal = ScVal::try_from(v)?;
        if let ScVal::Object(Some(o)) = v {
            Ok(o)
        } else {
            Err(ConversionError)
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScVec {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        let o: ScObject = ScObject::try_from(v)?;
        if let ScObject::Vec(vec) = o {
            Ok(vec)
        } else {
            Err(ConversionError)
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<Vec<T>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<Vec<T>> for ScObject {
    type Error = ConversionError;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<Vec<T>> for ScVec {
    type Error = ConversionError;
    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScVal> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(
            env,
            val.try_into_val(env).map_err(|_| ConversionError)?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScObject> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScObject) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, ScVal>>::try_from_val(env, ScVal::Object(Some(val)))
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScVec> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVec) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, ScObject>>::try_from_val(env, ScObject::Vec(val))
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryIntoVal<Env, Vec<T>> for ScVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        Vec::try_from_val(env, self)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryIntoVal<Env, Vec<T>> for ScObject
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        Vec::try_from_val(env, self)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryIntoVal<Env, Vec<T>> for ScVec
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        Vec::try_from_val(env, self)
    }
}

impl<T> Vec<T> {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: Object) -> Self {
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

    pub fn as_object(&self) -> &Object {
        &self.obj
    }

    pub fn to_object(&self) -> Object {
        self.obj
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    pub fn new(env: &Env) -> Vec<T> {
        unsafe { Self::unchecked_new(env.clone(), env.vec_new(().into())) }
    }

    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [T; N]) -> Vec<T> {
        let mut vec = Vec::new(env);
        vec.extend_from_array(items);
        vec
    }

    #[inline(always)]
    pub fn from_slice(env: &Env, items: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::new(env);
        vec.extend_from_slice(items);
        vec
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> Option<Result<T, T::Error>> {
        if i < self.len() {
            let env = self.env();
            let val = env.vec_get(self.obj, i.into());
            Some(T::try_from_val(env, val))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i: u32) -> Result<T, T::Error>
    where
        T::Error: Debug,
    {
        let env = self.env();
        let val = env.vec_get(self.obj, i.into());
        T::try_from_val(env, val)
    }

    #[inline(always)]
    pub fn set(&mut self, i: u32, v: T) {
        let env = self.env();
        self.obj = env.vec_put(self.obj, i.into(), v.into_val(env));
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
        self.obj = env.vec_del(self.obj, i.into());
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let env = self.env();
        let val = env.vec_len(self.obj);
        val.is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let val = env.vec_len(self.obj);
        unsafe { <_ as RawValConvertible>::unchecked_from_val(val) }
    }

    #[inline(always)]
    pub fn push_front(&mut self, x: T) {
        let env = self.env();
        self.obj = env.vec_push_front(self.obj, x.into_val(env));
    }

    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<Result<T, T::Error>> {
        let last = self.last()?;
        let env = self.env();
        self.obj = env.vec_pop_front(self.obj);
        Some(last)
    }

    #[inline(always)]
    pub fn pop_front_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.first_unchecked();
        let env = self.env();
        self.obj = env.vec_pop_front(self.obj);
        last
    }

    #[inline(always)]
    pub fn push_back(&mut self, x: T) {
        let env = self.env();
        self.obj = env.vec_push_back(self.obj, x.into_val(env));
    }

    #[inline(always)]
    pub fn pop_back(&mut self) -> Option<Result<T, T::Error>> {
        let last = self.last()?;
        let env = self.env();
        self.obj = env.vec_pop_back(self.obj);
        Some(last)
    }

    #[inline(always)]
    pub fn pop_back_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.last_unchecked();
        let env = self.env();
        self.obj = env.vec_pop_back(self.obj);
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
            let val = env.vec_front(self.obj);
            Some(T::try_from_val(env, val))
        }
    }

    #[inline(always)]
    pub fn first_unchecked(&self) -> Result<T, T::Error> {
        let env = &self.env;
        let val = env.vec_front(self.obj);
        T::try_from_val(env, val)
    }

    #[inline(always)]
    pub fn last(&self) -> Option<Result<T, T::Error>> {
        if self.is_empty() {
            None
        } else {
            let env = self.env();
            let val = env.vec_back(self.obj);
            Some(T::try_from_val(env, val))
        }
    }

    #[inline(always)]
    pub fn last_unchecked(&self) -> Result<T, T::Error> {
        let env = self.env();
        let val = env.vec_back(self.obj);
        T::try_from_val(env, val)
    }

    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: T) {
        let env = self.env();
        self.obj = env.vec_put(self.obj, i.into(), x.into_val(env));
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Vec<T>) {
        let env = self.env();
        self.obj = env.vec_append(self.obj, other.obj);
    }

    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, items: [T; N]) {
        for item in items {
            self.push_back(item);
        }
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
        let obj = env.vec_slice(self.obj, start_bound.into(), end_bound.into());
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
    for<'a> &'a T: IntoVal<Env, RawVal>,
{
    /// Returns true if the Vec contains the item.
    #[inline(always)]
    pub fn contains(&self, item: impl Borrow<T>) -> bool {
        let env = self.env();
        let val = item.borrow().into_val(env);
        !env.vec_first_index_of(self.obj, val).is_void()
    }

    /// Returns the index of the first occurrence of the item.
    ///
    /// If the item cannot be found [None] is returned.
    #[inline(always)]
    pub fn first_index_of(&self, item: impl Borrow<T>) -> Option<u32> {
        let env = self.env();
        let val = item.borrow().into_val(env);
        env.vec_first_index_of(self.obj, val)
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
        let high_low = env.vec_binary_search(self.obj, val);
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
            let val = self.0.env().vec_front(self.0.obj);
            self.0 = self.0.slice(1..);
            Some(T::try_from_val(self.0.env(), val))
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
            let val = self.0.env().vec_back(self.0.obj);
            self.0 = self.0.slice(..len - 1);
            Some(T::try_from_val(self.0.env(), val))
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
    fn contains() {
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
    fn first_index_of() {
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
    fn last_index_of() {
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
    fn binary_search() {
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
        let roundtrip = Vec::<i64>::try_from_val(&env, val).unwrap();
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
}
