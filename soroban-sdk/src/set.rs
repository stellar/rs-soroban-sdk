use core::{cmp::Ordering, fmt::Debug};

use super::{
    env::internal::Env as _,
    Env, IntoVal, Map, RawVal, TryFromVal
};

/// Create a [Set] with the given items.
///
/// The first argument in the list must be a reference to an [Env], then the
/// items follow.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, set};
///
/// let env = Env::default();
/// let set = set![&env, 0, 1, 2, 3, 3];
/// assert_eq!(set.len(), 4);
/// ```
#[macro_export]
macro_rules! set {
    ($env:expr) => {
        $crate::Set::new($env)
    };
    ($env:expr, $($x:expr),+ $(,)?) => {
        $crate::Set::from_array($env, [$($x), +])
    };
}

/// Set is a growable collection composed of unique items.
///
/// A Set can be seen as syntactic sugar on top of the soroban-sdk Map
/// implementation, where all of the Set's items are keys in a (hash) Map.
/// This forces all items to be unique, and the action of adding an
/// already-existing item to the Map is an idempotent operation.
///
/// Set imposes a fixed ordering, so Set<1, 2, 3> == Set<2, 3, 1>.
///
/// The array is stored in the Host and available to the Guest through the
/// functions defined on Set. Values stored in the Set are transmitted to the
/// Host as [RawVal]s, and when retrieved from the Set are transmitted back and
/// converted from [RawVal] back into their type.
///
/// The values in a Set are not guaranteed to be of type `T` and conversion will
/// fail if they are not.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, set};
///
/// let env = Env::default();
/// let mut set = set![&env, 1, 2, 3];
/// set.insert(3);
/// assert_eq!(set.len(), 3);
/// ```
#[derive(Clone)]
pub struct Set<T>(Map<T, ()>);

impl<T> Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn new(env: &Env) -> Set<T> {
        let map = Map::new(env);
        Self(map)
    }

    pub fn from_array<const N: usize>(env: &Env, items: [T; N]) -> Set<T> {
        let mut set = Set::new(env);
        set.extend_from_array(items);
        set
    }

    pub fn from_slice(env: &Env, items: &[T]) -> Set<T>
    where
        T: Clone,
    {
        let mut set = Set::new(env);
        set.extend_from_slice(items);
        set
    }

    pub fn insert(&mut self, x: T) {
        self.0.set(x, ())
    }

    pub fn extend_from_array<const N: usize>(&mut self, items: [T; N]) {
        for item in items {
            self.insert(item);
        }
    }

    pub fn extend_from_slice(&mut self, items: &[T])
    where
        T: Clone,
    {
        for item in items {
            self.insert(item.clone());
        }
    }

    pub fn remove(&mut self, x: T) -> Option<()> {
        self.0.remove(x)
    }

    pub fn contains(&self, x: T) -> bool {
        self.0.contains_key(x)
    }

    pub fn len(&self) -> u32 {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    pub fn first(&self) -> Option<Result<T, T::Error>> {
        let env = self.env();
        if self.is_empty() {
            None
        } else {
            let min_key = env.map_min_key(self.0.to_object());
            Some(T::try_from_val(env, min_key))
        }
    }

    pub fn iter(&self) -> SetIter<T>
    where
        T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
    {
        self.clone().into_iter()
    }
}


impl<T> Eq for Set<T> where T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}

impl<T> PartialEq for Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<T> PartialOrd for Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T> Ord for Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Debug for Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Debug + Clone,
    T::Error: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Set(")?;
        for k in self.iter() {
            write!(f, "{:?}", k)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<T> IntoIterator for Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Item = Result<T, T::Error>;
    type IntoIter = SetIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SetIter(self)
    }
}

#[derive(Clone)]
pub struct SetIter<T>(Set<T>);

impl<T> SetIter<T> {
    fn into_set(self) -> Set<T> {
        self.0
    }
}

impl<T> Iterator for SetIter<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Item = Result<T, T::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.0.first();
        if let Some(Ok(k)) = self.0.first() {
            self.0.remove(k);
        }
        first
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as usize;
        (len, Some(len))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_idempotent_insert() {
        let env = Env::default();
        let s1 = set![&env, 3];

        let mut s2 = set![&env, 3];
        s2.insert(3);
        assert_eq!(s1.len(), s2.len());

        let s3 = set![&env, 3, 4];
        assert_ne!(s2.len(), s3.len());
    }

    #[test]
    fn test_contains() {
        let env = Env::default();
        let s = set![&env, 3, 4];
        assert_eq!(s.contains(3), true);
        assert_eq!(s.contains(4), true);
        assert_eq!(s.contains(5), false);
    }

    #[test]
    fn test_is_empty() {
        let env = Env::default();
        let mut s = set![&env];
        assert_eq!(s.is_empty(), true);

        s.insert(3);
        assert_eq!(s.is_empty(), false);
    }

    #[test]
    fn test_remove() {
        let env = Env::default();
        let mut s = set![&env];

        assert_eq!(s.contains(1), false);

        s.insert(1);
        s.insert(2);
        assert_eq!(s.len(), 2);

        assert_eq!(s.contains(1), true);
        assert_eq!(s.contains(2), true);

        s.remove(1);
        assert_eq!(s.len(), 1);
        assert_eq!(s.contains(1), false);
        assert_eq!(s.contains(2), true);
    }

    #[test]
    fn test_from_array() {
        let env = Env::default();
        let s = Set::from_array(&env, [0, 1, 2, 3, 4]);

        assert_eq!(s.contains(0), true);
        assert_eq!(s.contains(1), true);
        assert_eq!(s.contains(4), true);
        assert_eq!(s.contains(5), false);
    }

    #[test]
    fn test_from_array_removes_duplicates() {
        let env = Env::default();
        let s = set![&env, 1, 1, 2, 3, 3, 3, 4, 5, 5, 5, 5];

        assert_eq!(s.contains(1), true);
        assert_eq!(s.contains(2), true);
        assert_eq!(s.contains(3), true);
        assert_eq!(s.contains(4), true);
        assert_eq!(s.contains(5), true);
        assert_eq!(s.contains(5), true);
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_comparison() {
        let env = Env::default();
        let s1 = set![&env, 0, 1, 2, 3, 3, 3, 3, 3, 3, 4];

        assert_eq!(s1, set![&env, 0, 0, 0, 1, 2, 3, 4]);
        assert_ne!(s1, set![&env, 1, 2, 3, 4]);
    }

    #[test]
    fn test_fixed_ordering() {
        let env = Env::default();
        let s1 = set![&env, 1, 2, 3];

        assert_eq!(s1, set![&env, 2, 3, 1]);
    }

    #[test]
    fn test_from_slice() {
        let env = Env::default();
        let s1 = set![&env, 1, 2, 3];
        let mut s2 = Set::from_slice(&env, &[1, 2, 3]);

        assert_eq!(s1, s2);

        s2.extend_from_slice(&[4, 5, 6]);
        assert_eq!(s2, set![&env, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_forward_iter() {
        let env = Env::default();
        let s = set![&env, 1, 2, 3, 4, 5];
        let mut iter = s.iter();

        assert_eq!(iter.next(), Some(Ok(1)));
        assert_eq!(iter.next(), Some(Ok(2)));
        assert_eq!(iter.next(), Some(Ok(3)));
        assert_eq!(iter.next(), Some(Ok(4)));
        assert_eq!(iter.next(), Some(Ok(5)));
        assert_eq!(iter.next(), None);

        // Ensure values are not deleted from original set during iter:
        assert_eq!(s, set![&env, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_iter_forloop() {
        let env = Env::default();
        let s = set![&env, 1, 2, 3, 4, 5];
        let mut c = 1;

        for i in s {
            assert_eq!(i, Ok(c));
            c += 1;
        }
    }
}
