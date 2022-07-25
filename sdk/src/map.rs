use core::{cmp::Ordering, fmt::Debug, iter::FusedIterator, marker::PhantomData};

use crate::iter::{UncheckedEnumerable, UncheckedIter};

use super::{
    env::internal::Env as _, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal,
    IntoTryFromVal, RawVal, Status, TryFromVal, Vec,
};

#[macro_export]
macro_rules! map {
    ($env:expr) => {
        $crate::Map::new($env)
    };
    ($env:expr, $(($k:expr, $v:expr $(,)?)),+ $(,)?) => {
        $crate::Map::from_array($env, [$(($k, $v)),+])
    };
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Map<K, V>(EnvObj, PhantomData<K>, PhantomData<V>);

impl<K: IntoTryFromVal, V: IntoTryFromVal> Eq for Map<K, V> {}

impl<K: IntoTryFromVal, V: IntoTryFromVal> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> PartialOrd for Map<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> Ord for Map<K, V> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl<K, V> Debug for Map<K, V>
where
    K: IntoTryFromVal + Debug + Clone,
    K::Error: Debug,
    V: IntoTryFromVal + Debug + Clone,
    V::Error: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Map(")?;
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

impl<K: IntoTryFromVal, V: IntoTryFromVal> TryFrom<EnvVal> for Map<K, V> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> TryFrom<EnvObj> for Map<K, V> {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Map) {
            Ok(Map(obj, PhantomData, PhantomData))
        } else {
            Err(ConversionError {})
        }
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> From<Map<K, V>> for RawVal {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> From<Map<K, V>> for EnvVal {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> From<Map<K, V>> for EnvObj {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0
    }
}

#[cfg(not(target_family = "wasm"))]
use super::{
    env::{EnvType, TryIntoEnvVal},
    xdr::ScVal,
};

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFrom<&Map<K, V>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Map<K, V>) -> Result<Self, Self::Error> {
        (&v.0).try_into().map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFrom<Map<K, V>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Map<K, V>) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K: IntoTryFromVal, V: IntoTryFromVal> TryFrom<EnvType<ScVal>> for Map<K, V> {
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        let ev: EnvObj = v
            .val
            .try_into_env_val(&v.env)
            .map_err(|_| ConversionError)?;
        ev.try_into()
    }
}

impl<K: IntoTryFromVal, V: IntoTryFromVal> Map<K, V> {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj, PhantomData, PhantomData)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Map<K, V> {
        let obj = env.map_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [(K, V); N]) -> Map<K, V> {
        let mut map = Map::<K, V>::new(env);
        for (k, v) in items {
            map.insert(k, v);
        }
        map
    }

    #[inline(always)]
    pub fn contains_key(&self, k: K) -> bool {
        let env = self.env();
        let has = env.map_has(self.0.to_tagged(), k.into_val(env));
        has.is_true()
    }

    #[inline(always)]
    pub fn get(&self, k: K) -> Option<Result<V, V::Error>> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.0.to_tagged(), k);
        if has.is_true() {
            let v = env.map_get(self.0.to_tagged(), k);
            Some(V::try_from_val(env, v))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, k: K) -> Result<V, V::Error> {
        let env = self.env();
        let v = env.map_get(self.0.to_tagged(), k.into_val(env));
        V::try_from_val(env, v)
    }

    #[inline(always)]
    pub fn insert(&mut self, k: K, v: V) {
        let env = self.env();
        let map = env.map_put(self.0.to_tagged(), k.into_val(env), v.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn remove(&mut self, k: K) -> Option<()> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.0.to_tagged(), k);
        if has.is_true() {
            let map = env.map_del(self.0.to_tagged(), k);
            self.0 = map.in_env(env);
            Some(())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn remove_unchecked(&mut self, k: K) {
        let env = self.env();
        let map = env.map_del(self.0.to_tagged(), k.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let env = self.env();
        let len = env.map_len(self.0.to_tagged());
        len.is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let len = env.map_len(self.0.to_tagged());
        u32::try_from_val(env, len).unwrap()
    }

    #[inline(always)]
    pub fn keys(&self) -> Vec<K> {
        let env = self.env();
        let vec = env.map_keys(self.0.to_tagged());
        Vec::<K>::try_from_val(env, vec).unwrap()
    }

    #[inline(always)]
    pub fn values(&self) -> Vec<V> {
        let env = self.env();
        let vec = env.map_values(self.0.to_tagged());
        Vec::<V>::try_from_val(env, vec).unwrap()
    }

    pub fn iter(&self) -> MapIter<K, V>
    where
        K: Clone,
        V: Clone,
    {
        self.clone().into_iter()
    }

    #[inline(always)]
    pub fn iter_unchecked(&self) -> UncheckedIter<MapIter<K, V>, (K, V), ConversionError>
    where
        K: IntoTryFromVal + Clone,
        K::Error: Debug,
        V: IntoTryFromVal + Clone,
        V::Error: Debug,
    {
        self.iter().unchecked()
    }

    #[inline(always)]
    pub fn into_iter_unchecked(self) -> UncheckedIter<MapIter<K, V>, (K, V), ConversionError>
    where
        K: IntoTryFromVal + Clone,
        K::Error: Debug,
        V: IntoTryFromVal + Clone,
        V::Error: Debug,
    {
        self.into_iter().unchecked()
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: IntoTryFromVal,
    V: IntoTryFromVal,
{
    type Item = Result<(K, V), ConversionError>;
    type IntoIter = MapIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        MapIter(self)
    }
}

#[derive(Clone)]
pub struct MapIter<K, V>(Map<K, V>);

impl<K, V> MapIter<K, V> {
    fn into_map(self) -> Map<K, V> {
        self.0
    }
}

impl<K, V> Iterator for MapIter<K, V>
where
    K: IntoTryFromVal,
    V: IntoTryFromVal,
{
    type Item = Result<(K, V), ConversionError>;

    fn next(&mut self) -> Option<Self::Item> {
        let env = &self.0 .0.env;
        let key = env.map_min_key(self.0 .0.to_object());
        if Status::try_from(key).is_ok() {
            return None;
        }
        let value = env.map_get(self.0 .0.to_tagged(), key);
        self.0 .0.val = env.map_del(self.0 .0.to_tagged(), key);
        Some(Ok((
            match K::try_from_val(env, key) {
                Ok(k) => k,
                Err(_) => return Some(Err(ConversionError)),
            },
            match V::try_from_val(env, value) {
                Ok(v) => v,
                Err(_) => return Some(Err(ConversionError)),
            },
        )))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.0.len() as usize;
        (len, Some(len))
    }

    // TODO: Implement other functions as optimizations.
}

impl<K, V> DoubleEndedIterator for MapIter<K, V>
where
    K: IntoTryFromVal,
    V: IntoTryFromVal,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let env = &self.0 .0.env;
        let key = env.map_max_key(self.0 .0.to_object());
        if Status::try_from(key).is_ok() {
            return None;
        }
        let value = env.map_get(self.0 .0.to_tagged(), key);
        self.0 .0.val = env.map_del(self.0 .0.to_tagged(), key);
        Some(Ok((
            match K::try_from_val(env, key) {
                Ok(k) => k,
                Err(_) => return Some(Err(ConversionError)),
            },
            match V::try_from_val(env, value) {
                Ok(v) => v,
                Err(_) => return Some(Err(ConversionError)),
            },
        )))
    }

    // TODO: Implement other functions as optimizations.
}

impl<K, V> FusedIterator for MapIter<K, V>
where
    K: IntoTryFromVal,
    V: IntoTryFromVal,
{
}

impl<K, V> ExactSizeIterator for MapIter<K, V>
where
    K: IntoTryFromVal,
    V: IntoTryFromVal,
{
    fn len(&self) -> usize {
        self.0.len() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_macro() {
        let env = Env::default();
        assert_eq!(map![&env], Map::<i32, i32>::new(&env));
        assert_eq!(map![&env, (1, 10)], {
            let mut v = Map::new(&env);
            v.insert(1, 10);
            v
        });
        assert_eq!(map![&env, (1, 10),], {
            let mut v = Map::new(&env);
            v.insert(1, 10);
            v
        });
        assert_eq!(map![&env, (3, 30), (2, 20), (1, 10),], {
            let mut v = Map::new(&env);
            v.insert(3, 30);
            v.insert(2, 20);
            v.insert(1, 10);
            v
        });
        assert_eq!(map![&env, (3, 30,), (2, 20,), (1, 10,),], {
            let mut v = Map::new(&env);
            v.insert(3, 30);
            v.insert(2, 20);
            v.insert(1, 10);
            v
        });
    }

    #[test]
    fn test_empty() {
        let env = Env::default();

        let map: Map<(), ()> = map![&env];
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_raw_vals() {
        let env = Env::default();

        let map: Map<u32, bool> = map![&env, (1, true), (2, false)];
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(1), Some(Ok(true)));
        assert_eq!(map.get(2), Some(Ok(false)));
        assert_eq!(map.get(3), None);
    }

    #[test]
    fn test_iter() {
        let env = Env::default();

        let map: Map<(), ()> = map![&env];
        let mut iter = map.iter();
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let map = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some(Ok((0, 0))));
        assert_eq!(iter.next(), Some(Ok((1, 10))));
        assert_eq!(iter.next(), Some(Ok((2, 20))));
        assert_eq!(iter.next(), Some(Ok((3, 30))));
        assert_eq!(iter.next(), Some(Ok((4, 40))));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some(Ok((0, 0))));
        assert_eq!(iter.next_back(), Some(Ok((4, 40))));
        assert_eq!(iter.next_back(), Some(Ok((3, 30))));
        assert_eq!(iter.next(), Some(Ok((1, 10))));
        assert_eq!(iter.next(), Some(Ok((2, 20))));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = map.iter().rev();
        assert_eq!(iter.next(), Some(Ok((4, 40))));
        assert_eq!(iter.next_back(), Some(Ok((0, 0))));
        assert_eq!(iter.next_back(), Some(Ok((1, 10))));
        assert_eq!(iter.next(), Some(Ok((3, 30))));
        assert_eq!(iter.next(), Some(Ok((2, 20))));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }
}
