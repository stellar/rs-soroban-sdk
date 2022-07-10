use core::{
    cmp::Ordering,
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};

use super::{
    env::internal::Env as _, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal,
    IntoTryFromVal, RawVal,
};

#[macro_export]
macro_rules! vec {
    ($env:expr) => {
        $crate::Vec::new($env)
    };
    ($env:expr, $($x:expr),+) => {
        $crate::Vec::from_array($env, [$($x),+])
    };
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec<T>(EnvObj, PhantomData<T>);

impl<T: IntoTryFromVal> Eq for Vec<T> {}

impl<T: IntoTryFromVal> PartialEq for Vec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<T: IntoTryFromVal> PartialOrd for Vec<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T: IntoTryFromVal> Ord for Vec<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl<T> Debug for Vec<T>
where
    T: IntoTryFromVal + Debug + Clone,
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

impl<T: IntoTryFromVal> TryFrom<EnvVal> for Vec<T> {
    type Error = ConversionError<EnvVal, Vec<T>>;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into().map_err(|_| Self::Error {
            from: PhantomData,
            to: PhantomData,
        })?;
        obj.try_into().map_err(|_| Self::Error {
            from: PhantomData,
            to: PhantomData,
        })
    }
}

impl<T: IntoTryFromVal> TryFrom<EnvObj> for Vec<T> {
    type Error = ConversionError<EnvObj, Vec<T>>;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Vec) {
            Ok(unsafe { Vec::<T>::unchecked_new(obj) })
        } else {
            Err(Self::Error {
                from: PhantomData,
                to: PhantomData,
            })
        }
    }
}

impl<T: IntoTryFromVal> From<Vec<T>> for RawVal {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<T: IntoTryFromVal> From<Vec<T>> for EnvVal {
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<T: IntoTryFromVal> From<Vec<T>> for EnvObj {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0
    }
}

impl<T: IntoTryFromVal> Vec<T> {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj, PhantomData)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Vec<T> {
        let obj = env.vec_new().in_env(env);
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
    pub fn get(&self, i: u32) -> Option<T>
    where
        T::Error: Debug,
    {
        if i < self.len() {
            Some(self.get_unchecked(i))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, i: u32) -> T
    where
        T::Error: Debug,
    {
        let env = self.env();
        let val = env.vec_get(self.0.to_tagged(), i.into());
        T::try_from_val(env, val).unwrap()
    }

    #[inline(always)]
    pub fn put(&mut self, i: u32, v: T) {
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
        self.len() == 0
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
    pub fn pop(&mut self) -> Option<T>
    where
        T::Error: Debug,
    {
        if self.is_empty() {
            None
        } else {
            Some(self.pop_unchecked())
        }
    }

    #[inline(always)]
    pub fn pop_unchecked(&mut self) -> T
    where
        T::Error: Debug,
    {
        let env = self.env();
        let last = self.last_unchecked();
        let vec = env.vec_pop(self.0.to_tagged());
        self.0 = vec.in_env(env);
        last
    }

    #[inline(always)]
    pub fn first(&self) -> Option<T>
    where
        T::Error: Debug,
    {
        if self.is_empty() {
            None
        } else {
            Some(self.first_unchecked())
        }
    }

    #[inline(always)]
    pub fn first_unchecked(&self) -> T
    where
        T::Error: Debug,
    {
        let env = self.0.env();
        let val = env.vec_front(self.0.to_tagged());
        T::try_from_val(env, val).unwrap()
    }

    #[inline(always)]
    pub fn last(&self) -> Option<T>
    where
        T::Error: Debug,
    {
        if self.is_empty() {
            None
        } else {
            Some(self.last_unchecked())
        }
    }

    #[inline(always)]
    pub fn last_unchecked(&self) -> T
    where
        T::Error: Debug,
    {
        let env = self.env();
        let val = env.vec_back(self.0.to_tagged());
        T::try_from_val(env, val).unwrap()
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
        T: IntoTryFromVal + Clone,
        T::Error: Debug,
    {
        self.clone().into_iter()
    }
}

impl<T> IntoIterator for Vec<T>
where
    T: IntoTryFromVal,
    T::Error: Debug,
{
    type Item = T;
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
    T: IntoTryFromVal,
    T::Error: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len == 0 {
            None
        } else {
            let item = self.0.first_unchecked();
            self.0 = self.0.slice(1..);
            Some(item)
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
    T: IntoTryFromVal,
    T::Error: Debug,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.0.len();
        if len == 0 {
            None
        } else {
            let item = self.0.last_unchecked();
            self.0 = self.0.slice(..len - 1);
            Some(item)
        }
    }

    // TODO: Implement other functions as optimizations since the iterator is
    // backed by an indexable collection.
}

impl<T> FusedIterator for VecIter<T>
where
    T: IntoTryFromVal,
    T::Error: Debug,
{
}

impl<T> ExactSizeIterator for VecIter<T>
where
    T: IntoTryFromVal,
    T::Error: Debug,
{
    fn len(&self) -> usize {
        self.0.len() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

        vec_copy.pop_unchecked();
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

        vec_copy.pop_unchecked();
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
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
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
}
