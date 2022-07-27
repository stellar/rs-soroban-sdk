use core::{
    cmp::Ordering,
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

use crate::iter::{UncheckedEnumerable, UncheckedIter};

use super::{
    env::internal::{Env as _, TagObject, TaggedVal},
    env::{EnvObj, EnvType},
    xdr::ScObjectType,
    ConversionError, Env, EnvVal, IntoVal, RawVal, TryFromVal, TryIntoVal,
};

#[macro_export]
macro_rules! vec {
    ($env:expr) => {
        $crate::Vec::new($env)
    };
    ($env:expr, $($x:expr),+ $(,)?) => {
        $crate::Vec::from_array($env, [$($x),+])
    };
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec<T>(EnvObj, PhantomData<T>);

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
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
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

impl<T> TryFrom<EnvVal> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<T> TryFrom<EnvObj> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Vec) {
            Ok(unsafe { Vec::<T>::unchecked_new(obj) })
        } else {
            Err(ConversionError {})
        }
    }
}

impl<T> TryIntoVal<Env, Vec<T>> for RawVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        EnvType {
            env: env.clone(),
            val: self,
        }
        .try_into()
    }
}

impl<T> From<Vec<T>> for RawVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<T> From<Vec<T>> for EnvVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<T> From<Vec<T>> for EnvObj
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0
    }
}

#[cfg(not(target_family = "wasm"))]
use super::{env::Object, xdr::ScVal};

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<&Vec<T>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Vec<T>) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
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
impl<T> TryIntoVal<Env, Vec<T>> for ScVal
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Vec<T>, Self::Error> {
        let o: Object = self.try_into_val(env).map_err(|_| ConversionError)?;
        let env = env.clone();
        EnvObj { val: o, env }.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T> TryFrom<EnvType<ScVal>> for Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        ScVal::try_into_val(v.val, &v.env)
    }
}

impl<T> Vec<T> {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj, PhantomData)
    }

    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub(crate) fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub(crate) fn as_tagged(&self) -> &TaggedVal<TagObject> {
        self.0.as_tagged()
    }

    pub(crate) fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub(crate) fn to_tagged(&self) -> TaggedVal<TagObject> {
        self.0.to_tagged()
    }
}

impl<T> Vec<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    pub fn new(env: &Env) -> Vec<T> {
        let obj = env.vec_new(().into()).in_env(env);
        unsafe { Self::unchecked_new(obj) }
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
            let val = env.vec_get(self.0.to_tagged(), i.into());
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
        let val = env.vec_get(self.0.to_tagged(), i.into());
        T::try_from_val(env, val)
    }

    #[inline(always)]
    pub fn set(&mut self, i: u32, v: T) {
        let env = self.env();
        let vec = env.vec_put(self.0.to_tagged(), i.into(), v.into_val(env));
        self.0 = vec.in_env(env);
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
        let vec = env.vec_del(self.0.to_tagged(), i.into());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let env = self.env();
        let val = env.vec_len(self.0.to_tagged());
        val.is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let val = env.vec_len(self.0.to_tagged());
        u32::try_from(val).unwrap()
    }

    #[inline(always)]
    pub fn push(&mut self, x: T) {
        let env = self.env();
        let vec = env.vec_push(self.0.to_tagged(), x.into_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<Result<T, T::Error>> {
        let last = self.last()?;
        let env = self.env();
        let vec = env.vec_pop(self.0.to_tagged());
        self.0 = vec.in_env(env);
        Some(last)
    }

    #[inline(always)]
    pub fn pop_unchecked(&mut self) -> Result<T, T::Error> {
        let last = self.last_unchecked();
        let env = self.env();
        let vec = env.vec_pop(self.0.to_tagged());
        self.0 = vec.in_env(env);
        last
    }

    #[inline(always)]
    pub fn first(&self) -> Option<Result<T, T::Error>> {
        if self.is_empty() {
            None
        } else {
            let env = self.0.env();
            let val = env.vec_front(self.0.to_tagged());
            Some(T::try_from_val(env, val))
        }
    }

    #[inline(always)]
    pub fn first_unchecked(&self) -> Result<T, T::Error> {
        let env = self.0.env();
        let val = env.vec_front(self.0.to_tagged());
        T::try_from_val(env, val)
    }

    #[inline(always)]
    pub fn last(&self) -> Option<Result<T, T::Error>> {
        if self.is_empty() {
            None
        } else {
            let env = self.env();
            let val = env.vec_back(self.0.to_tagged());
            Some(T::try_from_val(env, val))
        }
    }

    #[inline(always)]
    pub fn last_unchecked(&self) -> Result<T, T::Error> {
        let env = self.env();
        let val = env.vec_back(self.0.to_tagged());
        T::try_from_val(env, val)
    }

    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: T) {
        let env = self.env();
        let vec = env.vec_put(self.0.to_tagged(), i.into(), x.into_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Vec<T>) {
        let env = self.env();
        let vec = env.vec_append(self.0.to_tagged(), other.0.to_tagged());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn extend_from_array<const N: usize>(&mut self, items: [T; N]) {
        for item in items {
            self.push(item);
        }
    }

    #[inline(always)]
    pub fn extend_from_slice(&mut self, items: &[T])
    where
        T: Clone,
    {
        for item in items {
            self.push(item.clone());
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
        let vec = env.vec_slice(self.0.to_tagged(), start_bound.into(), end_bound.into());
        let vec = vec.in_env(env);
        unsafe { Self::unchecked_new(vec) }
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
            let val = self.0.env().vec_front(self.0 .0.to_object());
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
            let val = self.0.env().vec_back(self.0 .0.to_object());
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

    #[test]
    fn test_vec_macro() {
        let env = Env::default();
        assert_eq!(vec![&env], Vec::<i32>::new(&env));
        assert_eq!(vec![&env, 1], {
            let mut v = Vec::new(&env);
            v.push(1);
            v
        });
        assert_eq!(vec![&env, 1,], {
            let mut v = Vec::new(&env);
            v.push(1);
            v
        });
        assert_eq!(vec![&env, 3, 2, 1,], {
            let mut v = Vec::new(&env);
            v.push(3);
            v.push(2);
            v.push(1);
            v
        });
    }

    #[test]
    fn test_vec_raw_val_type() {
        let env = Env::default();

        let mut vec = Vec::<u32>::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push(10);
        assert_eq!(vec.len(), 1);
        vec.push(20);
        assert_eq!(vec.len(), 2);
        vec.push(30);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert!(vec == vec_copy);
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push(40);
        assert_eq!(vec_copy.len(), 4);
        assert!(vec != vec_copy);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);

        _ = vec_copy.pop_unchecked();
        assert!(vec == vec_copy);
    }

    #[test]
    fn test_vec_env_val_type() {
        let env = Env::default();

        let mut vec = Vec::<i64>::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push(-10);
        assert_eq!(vec.len(), 1);
        vec.push(20);
        assert_eq!(vec.len(), 2);
        vec.push(-30);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert!(vec == vec_copy);
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push(40);
        assert_eq!(vec_copy.len(), 4);
        assert!(vec != vec_copy);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);

        _ = vec_copy.pop_unchecked();
        assert!(vec == vec_copy);
    }

    #[test]
    fn test_vec_recursive() {
        let env = Env::default();

        let mut vec_inner = Vec::<i64>::new(&env);
        vec_inner.push(-10);
        assert_eq!(vec_inner.len(), 1);

        let mut vec_outer = Vec::<Vec<i64>>::new(&env);
        vec_outer.push(vec_inner);
        assert_eq!(vec_outer.len(), 1);
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
}
