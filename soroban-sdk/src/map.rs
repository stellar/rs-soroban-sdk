use core::{
    cmp::Ordering, convert::Infallible, fmt::Debug, iter::FusedIterator, marker::PhantomData,
};

use crate::{
    iter::{UnwrappedEnumerable, UnwrappedIter},
    unwrap::{UnwrapInfallible, UnwrapOptimized},
};

use super::{
    env::internal::{Env as _, EnvBase as _, MapObject, U32Val},
    ConversionError, Env, IntoVal, TryFromVal, TryIntoVal, Val, Vec,
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
/// Host as [Val]s, and when retrieved from the Map are transmitted back and
/// converted from [Val] back into their type.
///
/// The pairs of keys and values in a Map are not guaranteed to be of type
/// `K`/`V` and conversion will fail if they are not. Most functions on Map
/// return a `Result` due to this.
///
/// There are some cases where this lack of guarantee is important:
///
/// - When storing a Map that has been provided externally as a contract
/// function argument, be aware there is no guarantee that all pairs in the Map
/// will be of type `K` and `V`. It may be necessary to validate all pairs,
/// either before storing, or when loading with `try_` variation functions.
///
/// - When accessing and iterating over a Map that has been provided externally
/// as a contract function input, and the contract needs to be resilient to
/// failure, use the `try_` variation functions.
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
/// assert_eq!(map.iter().next(), Some((1, 10)));
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
    obj: MapObject,
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

impl<K, V> Eq for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
}

impl<K, V> PartialEq for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<K, V> PartialOrd for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<K, V> Ord for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
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

impl<K, V> Debug for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Debug + Clone,
    K::Error: Debug,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val> + Debug + Clone,
    V::Error: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Map(")?;
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

impl<K, V> TryFromVal<Env, MapObject> for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = Infallible;

    #[inline(always)]
    fn try_from_val(env: &Env, obj: &MapObject) -> Result<Self, Self::Error> {
        Ok(Map {
            env: env.clone(),
            obj: *obj,
            _k: PhantomData,
            _v: PhantomData,
        })
    }
}

impl<K, V> TryFromVal<Env, Val> for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(MapObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl<K, V> TryFromVal<Env, Map<K, V>> for Val
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = Infallible;

    fn try_from_val(_env: &Env, v: &Map<K, V>) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl<K, V> From<Map<K, V>> for Val
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.obj.into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFrom<&Map<K, V>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Map<K, V>) -> Result<Self, ConversionError> {
        ScVal::try_from_val(&v.env, &v.obj.to_val())
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFrom<Map<K, V>> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Map<K, V>) -> Result<Self, ConversionError> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFromVal<Env, Map<K, V>> for ScVal {
    type Error = ConversionError;
    fn try_from_val(_e: &Env, v: &Map<K, V>) -> Result<Self, ConversionError> {
        v.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFromVal<Env, ScVal> for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(MapObject::try_from_val(env, &Val::try_from_val(env, val)?)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl<K, V> Map<K, V> {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: MapObject) -> Self {
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
    pub fn as_val(&self) -> &Val {
        self.obj.as_val()
    }

    #[inline(always)]
    pub fn to_val(&self) -> Val {
        self.obj.to_val()
    }

    #[inline(always)]
    pub(crate) fn as_object(&self) -> &MapObject {
        &self.obj
    }

    #[inline(always)]
    pub(crate) fn to_object(&self) -> MapObject {
        self.obj
    }
}

impl<K, V> Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    /// Create an empty Map.
    #[inline(always)]
    pub fn new(env: &Env) -> Map<K, V> {
        unsafe { Self::unchecked_new(env.clone(), env.map_new().unwrap_infallible()) }
    }

    /// Create a Map from the key-value pairs in the array.
    #[inline(always)]
    pub fn from_array<const N: usize>(env: &Env, items: [(K, V); N]) -> Map<K, V> {
        let mut map = Map::<K, V>::new(env);
        for (k, v) in items {
            map.set(k, v);
        }
        map
    }

    /// Returns true if a key-value pair exists in the map with the given key.
    #[inline(always)]
    pub fn contains_key(&self, k: K) -> bool {
        self.env
            .map_has(self.obj, k.into_val(&self.env))
            .unwrap_infallible()
            .into()
    }

    /// Returns the value corresponding to the key or None if the map does not
    /// contain a value with the specified key.
    ///
    /// ### Panics
    ///
    /// If the value corresponding to the key cannot be converted to type V.
    #[inline(always)]
    pub fn get(&self, k: K) -> Option<V> {
        self.try_get(k).unwrap_optimized()
    }

    /// Returns the value corresponding to the key or None if the map does not
    /// contain a value with the specified key.
    ///
    /// ### Errors
    ///
    /// If the value corresponding to the key cannot be converted to type V.
    #[inline(always)]
    pub fn try_get(&self, k: K) -> Result<Option<V>, V::Error> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.obj, k).unwrap_infallible().into();
        if has {
            let v = env.map_get(self.obj, k).unwrap_infallible();
            V::try_from_val(env, &v).map(|val| Some(val))
        } else {
            Ok(None)
        }
    }

    /// Returns the value corresponding to the key.
    ///
    /// ### Panics
    ///
    /// If the map does not contain a value with the specified key.
    ///
    /// If the value corresponding to the key cannot be converted to type V.
    #[inline(always)]
    pub fn get_unchecked(&self, k: K) -> V {
        self.try_get_unchecked(k).unwrap_optimized()
    }

    /// Returns the value corresponding to the key.
    ///
    /// ### Errors
    ///
    /// If the value corresponding to the key cannot be converted to type V.
    ///
    /// ### Panics
    ///
    /// If the map does not contain a value with the specified key.
    #[inline(always)]
    pub fn try_get_unchecked(&self, k: K) -> Result<V, V::Error> {
        let env = self.env();
        let v = env.map_get(self.obj, k.into_val(env)).unwrap_infallible();
        V::try_from_val(env, &v)
    }

    /// Set the value for the specified key.
    ///
    /// If the map contains a value corresponding to the key, the value is
    /// replaced with the given value.
    #[inline(always)]
    pub fn set(&mut self, k: K, v: V) {
        let env = self.env();
        self.obj = env
            .map_put(self.obj, k.into_val(env), v.into_val(env))
            .unwrap_infallible();
    }

    /// Remove the value corresponding to the key.
    ///
    /// Returns `None` if the map does not contain a value with the specified
    /// key.
    #[inline(always)]
    pub fn remove(&mut self, k: K) -> Option<()> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.obj, k).unwrap_infallible().into();
        if has {
            self.obj = env.map_del(self.obj, k).unwrap_infallible();
            Some(())
        } else {
            None
        }
    }

    /// Remove the value corresponding to the key.
    ///
    /// ### Panics
    ///
    /// If the map does not contain a value with the specified key.
    #[inline(always)]
    pub fn remove_unchecked(&mut self, k: K) {
        let env = self.env();
        self.obj = env.map_del(self.obj, k.into_val(env)).unwrap_infallible();
    }

    /// Returns a [Vec] of all keys in the map.
    #[inline(always)]
    pub fn keys(&self) -> Vec<K> {
        let env = self.env();
        let vec = env.map_keys(self.obj).unwrap_infallible();
        Vec::<K>::try_from_val(env, &vec).unwrap()
    }

    /// Returns a [Vec] of all values in the map.
    #[inline(always)]
    pub fn values(&self) -> Vec<V> {
        let env = self.env();
        let vec = env.map_values(self.obj).unwrap_infallible();
        Vec::<V>::try_from_val(env, &vec).unwrap()
    }
}

impl<K, V> Map<K, V> {
    /// Returns true if the map is empty and contains no key-values.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of key-value pairs in the map.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env().map_len(self.obj).unwrap_infallible().into()
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Item = (K, V);
    type IntoIter = UnwrappedIter<MapTryIter<K, V>, (K, V), ConversionError>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        MapTryIter::new(self).unwrapped()
    }
}

impl<K, V> Map<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    #[inline(always)]
    pub fn iter(&self) -> UnwrappedIter<MapTryIter<K, V>, (K, V), ConversionError>
    where
        K: Clone,
        V: Clone,
    {
        self.clone().into_iter()
    }

    #[inline(always)]
    pub fn try_iter(&self) -> MapTryIter<K, V>
    where
        K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
        V: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
    {
        MapTryIter::new(self.clone())
    }

    #[inline(always)]
    pub fn into_try_iter(self) -> MapTryIter<K, V>
    where
        K: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
        V: IntoVal<Env, Val> + TryFromVal<Env, Val> + Clone,
    {
        MapTryIter::new(self.clone())
    }
}

#[derive(Clone)]
pub struct MapTryIter<K, V> {
    map: Map<K, V>,
    begin: u32,
    end: u32,
}

impl<K, V> MapTryIter<K, V> {
    fn new(map: Map<K, V>) -> Self {
        Self {
            begin: 0,
            end: map.len(),
            map,
        }
    }
}

impl<K, V> Iterator for MapTryIter<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    type Item = Result<(K, V), ConversionError>;

    fn next(&mut self) -> Option<Self::Item> {
        let env = self.map.env();
        if self.begin >= self.end {
            return None;
        }
        let map_obj = self.map.to_object();
        let index_val: U32Val = self.begin.into();
        let key = env.map_key_by_pos(map_obj, index_val).unwrap_infallible();
        let value = env.map_val_by_pos(map_obj, index_val).unwrap_infallible();
        self.begin += 1;

        Some(Ok((
            match K::try_from_val(env, &key) {
                Ok(k) => k,
                Err(_) => return Some(Err(ConversionError)),
            },
            match V::try_from_val(env, &value) {
                Ok(v) => v,
                Err(_) => return Some(Err(ConversionError)),
            },
        )))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end - self.begin) as usize;
        (len, Some(len))
    }

    // TODO: Implement other functions as optimizations.
}

impl<K, V> DoubleEndedIterator for MapTryIter<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let env = self.map.env();
        if self.begin >= self.end {
            return None;
        }
        self.end -= 1;
        let map_obj = self.map.to_object();
        let index_val: U32Val = self.end.into();
        let key = env.map_key_by_pos(map_obj, index_val).unwrap_infallible();
        let value = env.map_val_by_pos(map_obj, index_val).unwrap_infallible();

        Some(Ok((
            match K::try_from_val(env, &key) {
                Ok(k) => k,
                Err(_) => return Some(Err(ConversionError)),
            },
            match V::try_from_val(env, &value) {
                Ok(v) => v,
                Err(_) => return Some(Err(ConversionError)),
            },
        )))
    }

    // TODO: Implement other functions as optimizations.
}

impl<K, V> FusedIterator for MapTryIter<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
}

impl<K, V> ExactSizeIterator for MapTryIter<K, V>
where
    K: IntoVal<Env, Val> + TryFromVal<Env, Val>,
    V: IntoVal<Env, Val> + TryFromVal<Env, Val>,
{
    fn len(&self) -> usize {
        (self.end - self.begin) as usize
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
        assert_eq!(map.get(1), Some(true));
        assert_eq!(map.get(2), Some(false));
        assert_eq!(map.get(3), None);
    }

    #[test]
    fn test_iter() {
        let env = Env::default();

        let map: Map<(), ()> = map![&env];
        let mut iter = map.iter();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let map = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];

        let mut iter = map.iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some((1, 10)));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some((2, 20)));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some((3, 30)));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some((4, 40)));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = map.iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some((0, 0)));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some((4, 40)));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some((3, 30)));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some((1, 10)));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some((2, 20)));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = map.iter().rev();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some((4, 40)));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some((0, 0)));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some((1, 10)));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some((3, 30)));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some((2, 20)));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    #[should_panic(expected = "ConversionError")]
    fn test_iter_panic_on_key_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![&env, (1i64.into_val(&env), 2i32.into_val(&env)),];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();

        let mut iter = map.iter();
        iter.next();
    }

    #[test]
    #[should_panic(expected = "ConversionError")]
    fn test_iter_panic_on_value_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![&env, (1i32.into_val(&env), 2i64.into_val(&env)),];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();

        let mut iter = map.iter();
        iter.next();
    }

    #[test]
    fn test_try_iter() {
        let env = Env::default();

        let map: Map<(), ()> = map![&env];
        let mut iter = map.iter();
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let map = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];

        let mut iter = map.try_iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(Ok((0, 0))));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(Ok((1, 10))));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(Ok((2, 20))));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(Ok((3, 30))));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(Ok((4, 40))));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        let mut iter = map.try_iter();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(Ok((0, 0))));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some(Ok((4, 40))));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some(Ok((3, 30))));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(Ok((1, 10))));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(Ok((2, 20))));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);

        let mut iter = map.try_iter().rev();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(Ok((4, 40))));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next_back(), Some(Ok((0, 0))));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next_back(), Some(Ok((1, 10))));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(Ok((3, 30))));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(Ok((2, 20))));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_iter_error_on_key_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![
            &env,
            (1i32.into_val(&env), 2i32.into_val(&env)),
            (3i64.into_val(&env), 4i32.into_val(&env)),
        ];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();

        let mut iter = map.try_iter();
        assert_eq!(iter.next(), Some(Ok((1, 2))));
        assert_eq!(iter.next(), Some(Err(ConversionError)));
    }

    #[test]
    fn test_iter_error_on_value_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![
            &env,
            (1i32.into_val(&env), 2i32.into_val(&env)),
            (3i32.into_val(&env), 4i64.into_val(&env)),
        ];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();

        let mut iter = map.try_iter();
        assert_eq!(iter.next(), Some(Ok((1, 2))));
        assert_eq!(iter.next(), Some(Err(ConversionError)));
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

    #[test]
    fn test_from_array() {
        let env = Env::default();

        let map = Map::from_array(&env, [(0, 0), (1, 10), (2, 20), (3, 30), (4, 40)]);
        assert_eq!(map, map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)]);

        let map: Map<u32, u32> = Map::from_array(&env, []);
        assert_eq!(map, map![&env]);
    }

    #[test]
    fn test_contains_key() {
        let env = Env::default();

        let map: Map<u32, u32> = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];

        // contains all assigned keys
        for i in 0..map.len() {
            assert_eq!(true, map.contains_key(i));
        }

        // does not contain keys outside range
        assert_eq!(map.contains_key(6), false);
        assert_eq!(map.contains_key(u32::MAX), false);
        assert_eq!(map.contains_key(8), false);
    }

    #[test]
    fn test_is_empty() {
        let env = Env::default();

        let mut map: Map<u32, u32> = Map::new(&env);
        assert_eq!(map.is_empty(), true);
        map.set(0, 0);
        assert_eq!(map.is_empty(), false);
    }

    #[test]
    fn test_get() {
        let env = Env::default();

        let map: Map<u32, u32> = map![&env, (0, 0), (1, 10)];
        assert_eq!(map.get(0), Some(0));
        assert_eq!(map.get(1), Some(10));
        assert_eq!(map.get(2), None);
    }

    #[test]
    fn test_get_none_on_key_type_mismatch() {
        let env = Env::default();

        let map: Map<Val, Val> = map![
            &env,
            (1i32.into_val(&env), 2i32.into_val(&env)),
            (3i64.into_val(&env), 4i32.into_val(&env)),
        ];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        assert_eq!(map.get(1), Some(2));
        assert_eq!(map.get(3), None);
    }

    #[test]
    #[should_panic(expected = "ConversionError")]
    fn test_get_panics_on_value_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![&env, (1i32.into_val(&env), 2i64.into_val(&env)),];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        let _ = map.get(1);
    }

    #[test]
    fn test_try_get() {
        let env = Env::default();

        let map: Map<u32, u32> = map![&env, (0, 0), (1, 10)];
        assert_eq!(map.try_get(0), Ok(Some(0)));
        assert_eq!(map.try_get(1), Ok(Some(10)));
        assert_eq!(map.try_get(2), Ok(None));
    }

    #[test]
    fn test_try_get_none_on_key_type_mismatch() {
        let env = Env::default();

        let map: Map<Val, Val> = map![
            &env,
            (1i32.into_val(&env), 2i32.into_val(&env)),
            (3i64.into_val(&env), 4i32.into_val(&env)),
        ];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        assert_eq!(map.try_get(1), Ok(Some(2)));
        assert_eq!(map.try_get(3), Ok(None));
    }

    #[test]
    fn test_try_get_errors_on_value_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![
            &env,
            (1i32.into_val(&env), 2i32.into_val(&env)),
            (3i32.into_val(&env), 4i64.into_val(&env)),
        ];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        assert_eq!(map.try_get(1), Ok(Some(2)));
        assert_eq!(map.try_get(3), Err(ConversionError));
    }

    #[test]
    fn test_get_unchecked() {
        let env = Env::default();

        let map: Map<u32, u32> = map![&env, (0, 0), (1, 10)];
        assert_eq!(map.get_unchecked(0), 0);
        assert_eq!(map.get_unchecked(1), 10);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, MissingValue)")]
    fn test_get_unchecked_panics_on_key_type_mismatch() {
        let env = Env::default();

        let map: Map<Val, Val> = map![&env, (1i64.into_val(&env), 2i32.into_val(&env)),];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        let _ = map.get_unchecked(1);
    }

    #[test]
    #[should_panic(expected = "ConversionError")]
    fn test_get_unchecked_panics_on_value_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![&env, (1i32.into_val(&env), 2i64.into_val(&env)),];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        let _ = map.get_unchecked(1);
    }

    #[test]
    fn test_try_get_unchecked() {
        let env = Env::default();

        let map: Map<u32, u32> = map![&env, (0, 0), (1, 10)];
        assert_eq!(map.get_unchecked(0), 0);
        assert_eq!(map.get_unchecked(1), 10);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, MissingValue)")]
    fn test_try_get_unchecked_panics_on_key_type_mismatch() {
        let env = Env::default();

        let map: Map<Val, Val> = map![&env, (1i64.into_val(&env), 2i32.into_val(&env)),];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        let _ = map.try_get_unchecked(1);
    }

    #[test]
    fn test_try_get_unchecked_errors_on_value_conversion() {
        let env = Env::default();

        let map: Map<Val, Val> = map![
            &env,
            (1i32.into_val(&env), 2i32.into_val(&env)),
            (3i32.into_val(&env), 4i64.into_val(&env)),
        ];
        let map: Val = map.into();
        let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
        assert_eq!(map.try_get_unchecked(1), Ok(2));
        assert_eq!(map.try_get_unchecked(3), Err(ConversionError));
    }

    #[test]
    fn test_remove() {
        let env = Env::default();

        let mut map: Map<u32, u32> = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];

        assert_eq!(map.len(), 5);
        assert_eq!(map.get(2), Some(20));
        assert_eq!(map.remove(2), Some(()));
        assert_eq!(map.get(2), None);
        assert_eq!(map.len(), 4);

        // remove all items
        map.remove(0);
        map.remove(1);
        map.remove(3);
        map.remove(4);
        assert_eq!(map![&env], map);

        // removing from empty map
        let mut map: Map<u32, u32> = map![&env];
        assert_eq!(map.remove(0), None);
        assert_eq!(map.remove(u32::MAX), None);
    }

    #[test]
    fn test_remove_unchecked() {
        let env = Env::default();

        let mut map: Map<u32, u32> = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];

        assert_eq!(map.len(), 5);
        assert_eq!(map.get(2), Some(20));
        map.remove_unchecked(2);
        assert_eq!(map.get(2), None);
        assert_eq!(map.len(), 4);

        // remove all items
        map.remove_unchecked(0);
        map.remove_unchecked(1);
        map.remove_unchecked(3);
        map.remove_unchecked(4);
        assert_eq!(map![&env], map);
    }

    #[test]
    #[should_panic(expected = "HostError: Error(Object, MissingValue)")]
    fn test_remove_unchecked_panic() {
        let env = Env::default();
        let mut map: Map<u32, u32> = map![&env, (0, 0), (1, 10), (2, 20), (3, 30), (4, 40)];
        map.remove_unchecked(100); // key does not exist
    }
}
