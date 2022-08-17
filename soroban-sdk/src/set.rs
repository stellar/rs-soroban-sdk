use super::{Env, IntoVal, Map, RawVal, TryFromVal};

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

    pub fn insert(&mut self, x: T) {
        self.0.set(x, ())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_idempotent_insert() {
        let env = Env::default();
        let mut s1 = Set::new(&env);
        s1.insert(3);

        let mut s2 = Set::new(&env);
        s2.insert(3);
        s2.insert(3);

        assert_eq!(s1.len(), s2.len());

        let mut s3 = Set::new(&env);
        s3.insert(3);
        s3.insert(4);

        assert_ne!(s1.len(), s3.len());
    }

    #[test]
    fn test_contains() {
        let env = Env::default();
        let mut s = Set::new(&env);

        s.insert(3);
        s.insert(4);

        assert_eq!(s.contains(3), true);
        assert_eq!(s.contains(4), true);
        assert_eq!(s.contains(5), false);
    }

    #[test]
    fn test_is_empty() {
        let env = Env::default();
        let mut s = Set::new(&env);
        assert_eq!(s.is_empty(), true);

        s.insert(3);
        assert_eq!(s.is_empty(), false);
    }

    #[test]
    fn test_remove() {
        let env = Env::default();
        let mut s = Set::new(&env);

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
}
