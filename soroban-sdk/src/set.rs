use core::{cmp::Ordering, fmt::Debug};

use super::{Env, IntoVal, Map, RawVal, TryFromVal};

#[macro_export]
macro_rules! set {
    ($env:expr) => {
        $crate::Set::new($env)
    };
    ($env:expr, $($x:expr),+ $(,)?) => {
        $crate::Set::from_array($env, [$($x), +])
    };
}

pub struct Set<T>(Map<T, ()>);

impl<T> Set<T>
where
    T: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn new(env: &Env) -> Set<T> {
        let map = Map::<T, ()>::new(env);
        Self(map)
    }

    pub fn from_array<const N: usize>(env: &Env, items: [T; N]) -> Set<T> {
        let mut set = Set::new(env);
        set.extend_from_array(items);
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
        // let mut iter = self.iter();
        // if let Some(x) = iter.next() {
        //     write!(f, "{:?}", x)?;
        // }
        // for x in iter {
        //     write!(f, ", {:?}", x)?;
        // }
        // write!(f, ")")?;
        Ok(())
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
}
