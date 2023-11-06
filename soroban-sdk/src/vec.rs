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
    iter::{UnwrappedEnumerable, UnwrappedIter},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
};

use super::{
    env::internal::{Env as _, EnvBase as _, VecObject},
    ConversionError, Env, IntoVal, TryFromVal, TryIntoVal, Val,
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
        impl<$($typ),*> TryFromVal<Env, ($($typ,)*)> for Vec<Val>
        where
            $($typ: IntoVal<Env, Val>),*
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
/// the environment as [Val]s, and when retrieved from the Vec are
/// transmitted back and converted from [Val] back into their type.
///
/// The values in a Vec are not guaranteed to be of type `T` and conversion will
/// fail if they are not. Most functions on Vec have a `try_` variation that
/// returns a `Result` that will be `Err` if the conversion fails. Functions
/// that are not prefixed with `try_` will panic if conversion fails.
///
/// There are some cases where this lack of guarantee is important:
///
/// - When storing a Vec that has been provided externally as a contract
/// function argument, be aware there is no guarantee that all items in the Vec
/// will be of type `T`. It may be necessary to validate all values, either
/// before storing, or when loading with `try_` variation functions.
///
/// - When accessing and iterating over a Vec that has been provided externally
/// as a contract function input, and the contract needs to be resilient to
/// failure, use the `try_` variation functions.
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
pub struct Vec<T> {
    env: Env,
    obj: VecObject,
    _t: PhantomData<T>,
}

impl<T> Clone for Vec<T> {
    fn clone(&self) -> Self {
        Self {
            env: self.env.clone(),
            obj: self.obj.clone(),
            _t: self._t.clone(),
        }
    }
}

impl<T> Eq for Vec<T> where T: IntoVal<Env, Val> + TryFromVal<Env, Val> {}

impl<T> PartialEq for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<T> PartialOrd for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T> Ord for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.env.check_same_env(&other.env).unwrap_infallible();
        let v = self
            .env
            .obj_cmp(self.obj.to_val(), other.obj.to_val())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl<T> Debug for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val> + Debug + Clone,
    T::Error: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vec(")?;
        let mut iter = self.try_iter();
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

impl<T> TryFromVal<Env, Vec<T>> for Vec<Val> {
    type Error = Infallible;

    fn try_from_val(env: &Env, v: &Vec<T>) -> Result<Self, Self::Error> {
        Ok(unsafe { Vec::unchecked_new(env.clone(), v.obj.clone()) })
    }
}

// This conflicts with the previous definition unless we add the spurious &,
// which is not .. great. Maybe don't define this particular blanket, or add
// a to_other<T>() method?
impl<T> TryFromVal<Env, &Vec<Val>> for Vec<T> {
    type Error = Infallible;

    fn try_from_val(env: &Env, v: &&Vec<Val>) -> Result<Self, Self::Error> {
        Ok(unsafe { Vec::unchecked_new(env.clone(), v.obj.clone()) })
    }
}

impl<T> TryFromVal<Env, VecObject> for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = Infallible;

    #[inline(always)]
    fn try_from_val(env: &Env, obj: &VecObject) -> Result<Self, Self::Error> {
        Ok(unsafe { Vec::<T>::unchecked_new(env.clone(), obj.clone()) })
    }
}

impl<T> TryFromVal<Env, Val> for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(VecObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl<T> TryFromVal<Env, Vec<T>> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Vec<T>) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl<T> From<Vec<T>> for Val
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.obj.into()
    }
}

impl<T> From<Vec<T>> for VecObject
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.obj
    }
}

#[cfg(not(target_family = "wasm"))]
use super::xdr::{ScVal, ScVec, VecM};

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, ConversionError> {
        ScVal::try_from_val(&v.env, &v.obj.to_val())
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
impl<T> TryFrom<Vec<T>> for VecM<ScVal> {
    type Error = ConversionError;
    fn try_from(v: Vec<T>) -> Result<Self, ConversionError> {
        Ok(ScVec::try_from(v)?.0)
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
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, ConversionError> {
        Ok(VecObject::try_from_val(env, &Val::try_from_val(env, val)?)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, ScVec> for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVec) -> Result<Self, Self::Error> {
        ScVal::Vec(Some(val.clone())).try_into_val(env)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFromVal<Env, VecM<ScVal>> for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &VecM<ScVal>) -> Result<Self, Self::Error> {
        ScVec(val.clone()).try_into_val(env)
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

    pub fn as_val(&self) -> &Val {
        self.obj.as_val()
    }

    pub fn to_val(&self) -> Val {
        self.obj.to_val()
    }

    pub fn as_object(&self) -> &VecObject {
        &self.obj
    }

    pub fn to_object(&self) -> VecObject {
        self.obj
    }

    pub fn to_vals(&self) -> Vec<Val> {
        unsafe { Vec::<Val>::unchecked_new(self.env().clone(), self.obj) }
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    /// Create an empty Vec.
    #[inline(always)]
    pub fn new(env: &Env) -> Vec<T> {
        unsafe { Self::unchecked_new(env.clone(), env.vec_new().unwrap_infallible()) }
    }

    /// Create a Vec from the array of items.
    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [T; N]) -> Vec<T> {
        let mut tmp: [Val; N] = [Val::VOID.to_val(); N];
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

    /// Sets the item at the position with new value.
    ///
    /// ### Panics
    ///
    /// If the position is out-of-bounds.
    #[inline(always)]
    pub fn set(&mut self, i: u32, v: T) {
        let env = self.env();
        self.obj = env
            .vec_put(self.obj, i.into(), v.into_val(env))
            .unwrap_infallible();
    }

    /// Removes the item at the position.
    ///
    /// Returns `None` if out-of-bounds.
    #[inline(always)]
    pub fn remove(&mut self, i: u32) -> Option<()> {
        if i < self.len() {
            self.remove_unchecked(i);
            Some(())
        } else {
            None
        }
    }

    /// Removes the item at the position.
    ///
    /// ### Panics
    ///
    /// If the position is out-of-bounds.
    #[inline(always)]
    pub fn remove_unchecked(&mut self, i: u32) {
        let env = self.env();
        self.obj = env.vec_del(self.obj, i.into()).unwrap_infallible();
    }

    /// Adds the item to the front.
    ///
    /// Increases the length by one, shifts all items up by one, and puts the
    /// item in the first position.
    #[inline(always)]
    pub fn push_front(&mut self, x: T) {
        let env = self.env();
        self.obj = env
            .vec_push_front(self.obj, x.into_val(env))
            .unwrap_infallible();
    }

    /// Removes and returns the first item or None if empty.
    ///
    /// ### Panics
    ///
    /// If the value at the first position cannot be converted to type T.
    #[inline(always)]
    pub fn pop_front(&mut self) -> Option<T> {
        self.try_pop_front().unwrap_optimized()
    }

    /// Removes and returns the first item or None if empty.
    ///
    /// ### Errors
    ///
    /// If the value at the first position cannot be converted to type T.
    #[inline(always)]
    pub fn try_pop_front(&mut self) -> Result<Option<T>, T::Error> {
        if self.is_empty() {
            Ok(None)
        } else {
            self.try_pop_front_unchecked().map(|val| Some(val))
        }
    }

    /// Removes and returns the first item.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    ///
    /// If the value at the first position cannot be converted to type T.
    #[inline(always)]
    pub fn pop_front_unchecked(&mut self) -> T {
        self.try_pop_front_unchecked().unwrap_optimized()
    }

    /// Removes and returns the first item.
    ///
    /// ### Errors
    ///
    /// If the value at the first position cannot be converted to type T.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    #[inline(always)]
    pub fn try_pop_front_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.try_first_unchecked()?;
        let env = self.env();
        self.obj = env.vec_pop_front(self.obj).unwrap_infallible();
        Ok(last)
    }

    /// Adds the item to the back.
    ///
    /// Increases the length by one and puts the item in the last position.
    #[inline(always)]
    pub fn push_back(&mut self, x: T) {
        let env = self.env();
        self.obj = env
            .vec_push_back(self.obj, x.into_val(env))
            .unwrap_infallible();
    }

    /// Removes and returns the last item or None if empty.
    ///
    /// ### Panics
    ///
    /// If the value at the last position cannot be converted to type T.
    #[inline(always)]
    pub fn pop_back(&mut self) -> Option<T> {
        self.try_pop_back().unwrap_optimized()
    }

    /// Removes and returns the last item or None if empty.
    ///
    /// ### Errors
    ///
    /// If the value at the last position cannot be converted to type T.
    #[inline(always)]
    pub fn try_pop_back(&mut self) -> Result<Option<T>, T::Error> {
        if self.is_empty() {
            Ok(None)
        } else {
            self.try_pop_back_unchecked().map(|val| Some(val))
        }
    }

    /// Removes and returns the last item.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    ///
    /// If the value at the last position cannot be converted to type T.
    #[inline(always)]
    pub fn pop_back_unchecked(&mut self) -> T {
        self.try_pop_back_unchecked().unwrap_optimized()
    }

    /// Removes and returns the last item.
    ///
    /// ### Errors
    ///
    /// If the value at the last position cannot be converted to type T.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    #[inline(always)]
    pub fn try_pop_back_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.try_last_unchecked()?;
        let env = self.env();
        self.obj = env.vec_pop_back(self.obj).unwrap_infallible();
        Ok(last)
    }

    /// Returns the first item or None if empty.
    ///
    /// ### Panics
    ///
    /// If the value at the first position cannot be converted to type T.
    #[inline(always)]
    pub fn first(&self) -> Option<T> {
        self.try_first().unwrap_optimized()
    }

    /// Returns the first item or None if empty.
    ///
    /// ### Errors
    ///
    /// If the value at the first position cannot be converted to type T.
    #[inline(always)]
    pub fn try_first(&self) -> Result<Option<T>, T::Error> {
        if self.is_empty() {
            Ok(None)
        } else {
            self.try_first_unchecked().map(|val| Some(val))
        }
    }

    /// Returns the first item.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    ///
    /// If the value at the first position cannot be converted to type T.
    #[inline(always)]
    pub fn first_unchecked(&self) -> T {
        self.try_first_unchecked().unwrap_optimized()
    }

    /// Returns the first item.
    ///
    /// ### Errors
    ///
    /// If the value at the first position cannot be converted to type T.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    #[inline(always)]
    pub fn try_first_unchecked(&self) -> Result<T, T::Error> {
        let env = &self.env;
        let val = env.vec_front(self.obj).unwrap_infallible();
        T::try_from_val(env, &val)
    }

    /// Returns the last item or None if empty.
    ///
    /// ### Panics
    ///
    /// If the value at the last position cannot be converted to type T.
    #[inline(always)]
    pub fn last(&self) -> Option<T> {
        self.try_last().unwrap_optimized()
    }

    /// Returns the last item or None if empty.
    ///
    /// ### Errors
    ///
    /// If the value at the last position cannot be converted to type T.
    #[inline(always)]
    pub fn try_last(&self) -> Result<Option<T>, T::Error> {
        if self.is_empty() {
            Ok(None)
        } else {
            self.try_last_unchecked().map(|val| Some(val))
        }
    }

    /// Returns the last item.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    ///
    /// If the value at the last position cannot be converted to type T.
    #[inline(always)]
    pub fn last_unchecked(&self) -> T {
        self.try_last_unchecked().unwrap_optimized()
    }

    /// Returns the last item.
    ///
    /// ### Errors
    ///
    /// If the value at the last position cannot be converted to type T.
    ///
    /// ### Panics
    ///
    /// If the vec is empty.
    #[inline(always)]
    pub fn try_last_unchecked(&self) -> Result<T, T::Error> {
        let env = self.env();
        let val = env.vec_back(self.obj).unwrap_infallible();
        T::try_from_val(env, &val)
    }

    /// Inserts an item at the position.
    ///
    /// ### Panics
    ///
    /// If the position is out-of-bounds.
    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: T) {
        let env = self.env();
        self.obj = env
            .vec_insert(self.obj, i.into(), x.into_val(env))
            .unwrap_infallible();
    }

    /// Append the items.
    #[inline(always)]
    pub fn append(&mut self, other: &Vec<T>) {
        let env = self.env();
        self.obj = env.vec_append(self.obj, other.obj).unwrap_infallible();
    }

    /// Extend with the items in the array.
    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, items: [T; N]) {
        self.append(&Self::from_array(&self.env, items))
    }

    /// Extend with the items in the slice.
    #[inline(always)]
    pub fn extend_from_slice(&mut self, items: &[T])
    where
        T: Clone,
    {
        for item in items {
            self.push_back(item.clone());
        }
    }
}

impl<T> Vec<T> {
    /// Returns a subset of the bytes as defined by the start and end bounds of
    /// the range.
    ///
    /// ### Panics
    ///
    /// If the range is out-of-bounds.
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

    /// Returns copy of the vec shuffled using the NOT-SECURE PRNG.
    ///
    /// In tests, must be called from within a running contract.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator used to perform the shuffle is not
    /// suitable for security-sensitive work.**
    pub fn shuffle(&mut self) {
        let env = self.env();
        env.prng().shuffle(self);
    }

    /// Returns copy of the vec shuffled using the NOT-SECURE PRNG.
    ///
    /// In tests, must be called from within a running contract.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator used to perform the shuffle is not
    /// suitable for security-sensitive work.**
    #[must_use]
    pub fn to_shuffled(&self) -> Self {
        let mut copy = self.clone();
        copy.shuffle();
        copy
    }

    /// Returns true if the vec is empty and contains no items.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of items in the vec.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env.vec_len(self.obj).unwrap_infallible().into()
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, Val>,
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
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    T: Clone,
{
    #[inline(always)]
    pub fn concat(&self) -> Vec<T> {
        let mut concatenated = vec![self.env()];
        for vec in self.iter() {
            concatenated.append(&vec);
        }
        concatenated
    }
}

impl<T> IntoIterator for Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Item = T;
    type IntoIter = UnwrappedIter<VecTryIter<T>, T, T::Error>;

    fn into_iter(self) -> Self::IntoIter {
        VecTryIter::new(self).unwrapped()
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    #[inline(always)]
    pub fn iter(&self) -> UnwrappedIter<VecTryIter<T>, T, T::Error>
    where
        T: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
        T::Error: Debug,
    {
        self.try_iter().unwrapped()
    }

    #[inline(always)]
    pub fn try_iter(&self) -> VecTryIter<T>
    where
        T: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
    {
        VecTryIter::new(self.clone())
    }

    #[inline(always)]
    pub fn into_try_iter(self) -> VecTryIter<T>
    where
        T: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
        T::Error: Debug,
    {
        VecTryIter::new(self.clone())
    }
}

#[derive(Clone)]
pub struct VecTryIter<T> {
    vec: Vec<T>,
    start: u32, // inclusive
    end: u32,   // exclusive
}

impl<T> VecTryIter<T> {
    fn new(vec: Vec<T>) -> Self {
        Self {
            start: 0,
            end: vec.len(),
            vec,
        }
    }

    fn into_vec(self) -> Vec<T> {
        self.vec.slice(self.start..self.end)
    }
}

impl<T> Iterator for VecTryIter<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Item = Result<T, T::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let val = self.vec.try_get_unchecked(self.start);
            self.start += 1;
            Some(val)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end - self.start) as usize;
        (len, Some(len))
    }

    // TODO: Implement other functions as optimizations since the iterator is
    // backed by an indexable collection.
}

impl<T> DoubleEndedIterator for VecTryIter<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let val = self.vec.try_get_unchecked(self.end - 1);
            self.end -= 1;
            Some(val)
        } else {
            None
        }
    }

    // TODO: Implement other functions as optimizations since the iterator is
    // backed by an indexable collection.
}

impl<T> FusedIterator for VecTryIter<T> where T: IntoVal<Env, Val> + TryFromVal<Env, Val> {}

impl<T> ExactSizeIterator for VecTryIter<T>
where
    T: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn test_vec_to_vals() {
        let env = Env::default();
        let vec = vec![&env, 0, 1, 2, 3, 4];
        let vals = vec.to_vals();
        assert_eq!(
            vals,
            vec![
                &env,
                Val::from_i32(0).to_val(),
                Val::from_i32(1).to_val(),
                Val::from_i32(2).to_val(),
                Val::from_i32(3).to_val(),
                Val::from_i32(4).to_val(),
            ]
        );
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
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let vec = vec![&env, 0, 1, 2, 3, 4];

        let mut iter = vec.iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = vec.iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = vec.iter().rev();
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_vec_iter_panic_on_conversion() {
        let env = Env::default();

        let vec: Val = (1i32,).try_into_val(&env).unwrap();
        let vec: Vec<i64> = vec.try_into_val(&env).unwrap();

        let mut iter = vec.iter();
        iter.next();
    }

    #[test]
    fn test_vec_try_iter() {
        let env = Env::default();

        let vec: Vec<()> = vec![&env];
        let mut iter = vec.try_iter();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let vec = vec![&env, 0, 1, 2, 3, 4];

        let mut iter = vec.try_iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(Ok(0)));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(Ok(3)));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(Ok(4)));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = vec.try_iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(Ok(0)));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some(Ok(4)));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some(Ok(3)));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = vec.try_iter().rev();
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
    fn test_vec_try_iter_error_on_conversion() {
        let env = Env::default();

        let vec: Val = (1i64, 2i32).try_into_val(&env).unwrap();
        let vec: Vec<i64> = vec.try_into_val(&env).unwrap();

        let mut iter = vec.try_iter();
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.next(), Some(Err(ConversionError.into())));
    }

    #[test]
    fn test_vec_iter_into_vec() {
        let env = Env::default();

        let vec = vec![&env, 0, 1, 2, 3, 4];

        let mut iter = vec.try_iter();
        assert_eq!(iter.next(), Some(Ok(0)));
        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.into_vec(), vec![&env, 2, 3, 4]);
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
        v.push_front(5);
        assert_eq!(v, vec![&env, 5, 1, 42]);
        v.push_front(7);
        assert_eq!(v, vec![&env, 7, 5, 1, 42]);

        let popped = v.pop_front();
        assert_eq!(popped, Some(7));
        assert_eq!(v, vec![&env, 5, 1, 42]);

        let popped = v.try_pop_front();
        assert_eq!(popped, Ok(Some(5)));
        assert_eq!(v, vec![&env, 1, 42]);

        let popped = v.pop_front_unchecked();
        assert_eq!(popped, 1);
        assert_eq!(v, vec![&env, 42]);

        let popped = v.try_pop_front_unchecked();
        assert_eq!(popped, Ok(42));
        assert_eq!(v, vec![&env]);

        assert_eq!(v.pop_front(), None);
    }

    #[test]
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_pop_front_panics_on_conversion() {
        let env = Env::default();

        let v: Val = (1i32,).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        v.pop_front();
    }

    #[test]
    fn test_try_pop_front_errors_on_conversion() {
        let env = Env::default();

        let v: Val = (1i64, 2i32).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        assert_eq!(v.try_pop_front(), Ok(Some(1)));
        assert_eq!(v.try_pop_front(), Err(ConversionError.into()));
    }

    #[test]
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_pop_front_unchecked_panics_on_conversion() {
        let env = Env::default();

        let v: Val = (1i32,).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        v.pop_front_unchecked();
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_pop_front_unchecked_panics_on_out_of_bounds() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);

        v.pop_front_unchecked();
    }

    #[test]
    fn test_try_pop_front_unchecked_errors_on_conversion() {
        let env = Env::default();

        let v: Val = (1i64, 2i32).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        assert_eq!(v.try_pop_front_unchecked(), Ok(1));
        assert_eq!(v.try_pop_front_unchecked(), Err(ConversionError.into()));
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_try_pop_front_unchecked_panics_on_out_of_bounds() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);

        let _ = v.try_pop_front_unchecked();
    }

    #[test]
    fn test_push_pop_back() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);
        v.push_back(42);
        assert_eq!(v, vec![&env, 42]);
        v.push_back(1);
        assert_eq!(v, vec![&env, 42, 1]);
        v.push_back(5);
        assert_eq!(v, vec![&env, 42, 1, 5]);
        v.push_back(7);
        assert_eq!(v, vec![&env, 42, 1, 5, 7]);

        let popped = v.pop_back();
        assert_eq!(popped, Some(7));
        assert_eq!(v, vec![&env, 42, 1, 5]);

        let popped = v.try_pop_back();
        assert_eq!(popped, Ok(Some(5)));
        assert_eq!(v, vec![&env, 42, 1]);

        let popped = v.pop_back_unchecked();
        assert_eq!(popped, 1);
        assert_eq!(v, vec![&env, 42]);

        let popped = v.try_pop_back_unchecked();
        assert_eq!(popped, Ok(42));
        assert_eq!(v, vec![&env]);

        assert_eq!(v.pop_back(), None);
    }

    #[test]
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_pop_back_panics_on_conversion() {
        let env = Env::default();

        let v: Val = (1i32,).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        v.pop_back();
    }

    #[test]
    fn test_try_pop_back_errors_on_conversion() {
        let env = Env::default();

        let v: Val = (1i32, 2i64).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        assert_eq!(v.try_pop_back(), Ok(Some(2)));
        assert_eq!(v.try_pop_back(), Err(ConversionError.into()));
    }

    #[test]
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_pop_back_unchecked_panics_on_conversion() {
        let env = Env::default();

        let v: Val = (1i32,).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        v.pop_back_unchecked();
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_pop_back_unchecked_panics_on_out_of_bounds() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);

        v.pop_back_unchecked();
    }

    #[test]
    fn test_try_pop_back_unchecked_errors_on_conversion() {
        let env = Env::default();

        let v: Val = (1i32, 2i64).try_into_val(&env).unwrap();
        let mut v: Vec<i64> = v.try_into_val(&env).unwrap();

        assert_eq!(v.try_pop_back_unchecked(), Ok(2));
        assert_eq!(v.try_pop_back_unchecked(), Err(ConversionError.into()));
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, IndexBounds)")]
    fn test_try_pop_back_unchecked_panics_on_out_of_bounds() {
        let env = Env::default();

        let mut v = Vec::<i64>::new(&env);

        let _ = v.try_pop_back_unchecked();
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
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_get_panics_on_conversion() {
        let env = Env::default();

        let v: Val = (1i64, 2i32).try_into_val(&env).unwrap();
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
        let v: Val = (1i64, 2i32).try_into_val(&env).unwrap();
        let v: Vec<i64> = v.try_into_val(&env).unwrap();
        assert_eq!(v.try_get(0), Ok(Some(1)));
        assert_eq!(v.try_get(1), Err(ConversionError.into()));
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
    #[should_panic(expected = "Error(Value, UnexpectedType)")]
    fn test_get_unchecked_panics_on_conversion() {
        let env = Env::default();

        let v: Val = (1i64, 2i32).try_into_val(&env).unwrap();
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
        let v: Val = (1i64, 2i32).try_into_val(&env).unwrap();
        let v: Vec<i64> = v.try_into_val(&env).unwrap();
        assert_eq!(v.try_get_unchecked(0), Ok(1));
        assert_eq!(v.try_get_unchecked(1), Err(ConversionError.into()));
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
