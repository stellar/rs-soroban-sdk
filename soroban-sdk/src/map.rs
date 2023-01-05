use core::{cmp::Ordering, fmt::Debug, iter::FusedIterator, marker::PhantomData};

use crate::iter::{UncheckedEnumerable, UncheckedIter};

use super::{
    env::internal::{try_convert_to, Env as _, EnvBase as _, RawValConvertible},
    xdr::ScObjectType,
    ConversionError, Env, IntoVal, Object, RawVal, Status, TryIntoVal, Vec,
};

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

#[cfg(doc)]
use crate::storage::Storage;

/// Create a [Map] with the given key-value pairs.
///
/// The first argument in the list must be a reference to an [Env], then the
/// key-value pairs follow in a tuple `(key, value)`.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::{Env, Map, map};
///
/// let env = Env::default();
/// let map = map![&env, (1, 10), (2, 20)];
/// assert_eq!(map.len(), 2);
/// ```
#[macro_export]
macro_rules! map {
    ($env:expr $(,)?) => {
        $crate::Map::new($env)
    };
    ($env:expr, $(($k:expr, $v:expr $(,)?)),+ $(,)?) => {
        $crate::Map::from_array($env, [$(($k, $v)),+])
    };
}

/// Map is a ordered key-value dictionary.
///
/// The map is ordered by its keys. Iterating a map is stable and always returns
/// the keys and values in order of the keys.
///
/// The map is stored in the Host and available to the Guest through the
/// functions defined on Map. Values stored in the Map are transmitted to the
/// Host as [RawVal]s, and when retrieved from the Map are transmitted back and
/// converted from [RawVal] back into their type.
///
/// The keys and values in a Map are not guaranteed to be of type `K`/`V` and
/// conversion will fail if they are not. Most functions on Map return a
/// `Result` due to this.
///
/// Maps have at most one entry per key. Setting a value for a key in the map
/// that already has a value for that key replaces the value.
///
/// Map values can be stored as [Storage], or in other types like [Vec], [Map],
/// etc.
///
/// ### Examples
///
/// Maps can be created and iterated.
///
/// ```
/// use soroban_sdk::{Env, Map, map};
///
/// let env = Env::default();
/// let map = map![&env, (2, 20), (1, 10)];
/// assert_eq!(map.len(), 2);
/// assert_eq!(map.iter().next(), Some(Ok((1, 10))));
/// ```
///
/// Maps are ordered and so maps created with elements in different order will
/// be equal.
///
/// ```
/// use soroban_sdk::{Env, Map, map};
///
/// let env = Env::default();
/// assert_eq!(
///     map![&env, (1, 10), (2, 20)],
///     map![&env, (2, 20), (1, 10)],
/// )
/// ```
#[derive(Clone)]
pub struct Map<K, V> {
    env: Env,
    obj: Object,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V> Eq for Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
}

impl<K, V> PartialEq for Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<K, V> PartialOrd for Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<K, V> Ord for Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.env.check_same_env(&other.env);
        let v = self.env.obj_cmp(self.obj.to_raw(), other.obj.to_raw());
        v.cmp(&0)
    }
}

impl<K, V> Debug for Map<K, V>
where
    K: IntoVal<Env, RawVal> + Debug + Clone,
    <RawVal as TryIntoVal<Env, K>>::Error: Debug,
    V: IntoVal<Env, RawVal> + Debug + Clone,
    <RawVal as TryIntoVal<Env, V>>::Error: Debug,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
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

impl<K, V> TryIntoVal<Env, Map<K, V>> for Object
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Map<K, V>, Self::Error> {
        if self.is_obj_type(ScObjectType::Map) {
            Ok(Map {
                env: env.clone(),
                obj: self,
                _k: PhantomData,
                _v: PhantomData,
            })
        } else {
            Err(ConversionError {})
        }
    }
}

impl<K, V> TryIntoVal<Env, Map<K, V>> for RawVal
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Map<K, V>, Self::Error> {
        try_convert_to::<_, _, Object>(self.try_into()?, env)
    }
}

impl<K, V> IntoVal<Env, RawVal> for Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn into_val(self, _env: &Env) -> RawVal {
        self.into()
    }
}

impl<K, V> IntoVal<Env, RawVal> for &Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
    }
}

impl<K, V> From<Map<K, V>> for RawVal
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.obj.into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFrom<&Map<K, V>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Map<K, V>) -> Result<Self, Self::Error> {
        v.obj.to_raw().try_into_val(&v.env)
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
impl<K, V> TryIntoVal<Env, Map<K, V>> for ScVal
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Map<K, V>, Self::Error> {
        try_convert_to::<_, _, Object>(self.try_into_val(env).map_err(|_| ConversionError)?, env)
    }
}

impl<K, V> Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: Object) -> Self {
        Self {
            env,
            obj,
            _k: PhantomData,
            _v: PhantomData,
        }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    #[inline(always)]
    pub fn as_raw(&self) -> &RawVal {
        self.obj.as_raw()
    }

    #[inline(always)]
    pub fn to_raw(&self) -> RawVal {
        self.obj.to_raw()
    }

    #[inline(always)]
    pub(crate) fn as_object(&self) -> &Object {
        &self.obj
    }

    #[inline(always)]
    pub(crate) fn to_object(&self) -> Object {
        self.obj
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Map<K, V> {
        unsafe { Self::unchecked_new(env.clone(), env.map_new()) }
    }

    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [(K, V); N]) -> Map<K, V> {
        let mut map = Map::<K, V>::new(env);
        for (k, v) in items {
            map.set(k, v);
        }
        map
    }

    #[inline(always)]
    pub fn contains_key(&self, k: K) -> bool {
        let env = self.env();
        let has = env.map_has(self.obj, k.into_val(env));
        has.is_true()
    }

    #[inline(always)]
    pub fn get(&self, k: K) -> Option<Result<V, <RawVal as TryIntoVal<Env, V>>::Error>> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.obj, k);
        if has.is_true() {
            let v = env.map_get(self.obj, k);
            Some(v.try_into_val(env))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, k: K) -> Result<V, <RawVal as TryIntoVal<Env, V>>::Error> {
        let env = self.env();
        let v = env.map_get(self.obj, k.into_val(env));
        v.try_into_val(env)
    }

    #[inline(always)]
    pub fn set(&mut self, k: K, v: V) {
        let env = self.env();
        self.obj = env.map_put(self.obj, k.into_val(env), v.into_val(env));
    }

    #[inline(always)]
    pub fn remove(&mut self, k: K) -> Option<()> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.obj, k);
        if has.is_true() {
            self.obj = env.map_del(self.obj, k);
            Some(())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn remove_unchecked(&mut self, k: K) {
        let env = self.env();
        self.obj = env.map_del(self.obj, k.into_val(env));
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let env = self.env();
        let len = env.map_len(self.obj);
        len.is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let len = env.map_len(self.obj);
        unsafe { <u32 as RawValConvertible>::unchecked_from_val(len) }
    }

    #[inline(always)]
    pub fn keys(&self) -> Vec<K> {
        let env = self.env();
        let vec = env.map_keys(self.obj);
        vec.try_into_val(env).unwrap()
    }

    #[inline(always)]
    pub fn values(&self) -> Vec<V> {
        let env = self.env();
        let vec = env.map_values(self.obj);
        vec.try_into_val(env).unwrap()
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
        K: IntoVal<Env, RawVal> + Clone,
        <RawVal as TryIntoVal<Env, K>>::Error: Debug,
        V: IntoVal<Env, RawVal> + Clone,
        <RawVal as TryIntoVal<Env, V>>::Error: Debug,
        RawVal: TryIntoVal<Env, K>,
        RawVal: TryIntoVal<Env, V>,
    {
        self.iter().unchecked()
    }

    #[inline(always)]
    pub fn into_iter_unchecked(self) -> UncheckedIter<MapIter<K, V>, (K, V), ConversionError>
    where
        K: IntoVal<Env, RawVal> + Clone,
        <RawVal as TryIntoVal<Env, K>>::Error: Debug,
        V: IntoVal<Env, RawVal> + Clone,
        <RawVal as TryIntoVal<Env, V>>::Error: Debug,
        RawVal: TryIntoVal<Env, K>,
        RawVal: TryIntoVal<Env, V>,
    {
        self.into_iter().unchecked()
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
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
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    type Item = Result<(K, V), ConversionError>;

    fn next(&mut self) -> Option<Self::Item> {
        let env = &self.0.env;
        let key = env.map_min_key(self.0.obj);
        if Status::try_from(key).is_ok() {
            return None;
        }
        let value = env.map_get(self.0.obj, key);
        self.0.obj = env.map_del(self.0.obj, key);
        Some(Ok((
            match key.try_into_val(env) {
                Ok(k) => k,
                Err(_) => return Some(Err(ConversionError)),
            },
            match value.try_into_val(env) {
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
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let env = &self.0.env;
        let key = env.map_max_key(self.0.obj);
        if Status::try_from(key).is_ok() {
            return None;
        }
        let value = env.map_get(self.0.obj, key);
        self.0.obj = env.map_del(self.0.obj, key);
        Some(Ok((
            match key.try_into_val(env) {
                Ok(k) => k,
                Err(_) => return Some(Err(ConversionError)),
            },
            match value.try_into_val(env) {
                Ok(v) => v,
                Err(_) => return Some(Err(ConversionError)),
            },
        )))
    }

    // TODO: Implement other functions as optimizations.
}

impl<K, V> FusedIterator for MapIter<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
}

impl<K, V> ExactSizeIterator for MapIter<K, V>
where
    K: IntoVal<Env, RawVal>,
    V: IntoVal<Env, RawVal>,
    RawVal: TryIntoVal<Env, K>,
    RawVal: TryIntoVal<Env, V>,
{
    fn len(&self) -> usize {
        self.0.len() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vec;

    #[test]
    fn test_map_macro() {
        let env = Env::default();
        assert_eq!(map![&env], Map::<i32, i32>::new(&env));
        assert_eq!(map![&env, (1, 10)], {
            let mut v = Map::new(&env);
            v.set(1, 10);
            v
        });
        assert_eq!(map![&env, (1, 10),], {
            let mut v = Map::new(&env);
            v.set(1, 10);
            v
        });
        assert_eq!(map![&env, (3, 30), (2, 20), (1, 10),], {
            let mut v = Map::new(&env);
            v.set(3, 30);
            v.set(2, 20);
            v.set(1, 10);
            v
        });
        assert_eq!(map![&env, (3, 30,), (2, 20,), (1, 10,),], {
            let mut v = Map::new(&env);
            v.set(3, 30);
            v.set(2, 20);
            v.set(1, 10);
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

    #[test]
    fn test_keys() {
        let env = Env::default();

        let map = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];
        let keys = map.keys();
        assert_eq!(keys, vec![&env, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_values() {
        let env = Env::default();

        let map = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];
        let values = map.values();
        assert_eq!(values, vec![&env, 0, 10, 20, 30, 40]);
    }
}
