use core::{cmp::Ordering, fmt::Debug, iter::FusedIterator, marker::PhantomData};

use crate::iter::{UncheckedEnumerable, UncheckedIter};

use super::{
    env::internal::Env as _,
    env::{EnvObj, EnvType},
    xdr::ScObjectType,
    ConversionError, Env, EnvVal, IntoVal, RawVal, Status, TryFromVal, TryIntoVal, Vec,
};

#[cfg(not(target_family = "wasm"))]
use super::{env::Object, xdr::ScVal};

#[cfg(doc)]
use crate::ContractData;

/// Create a [Map] with the given key-value pairs.
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
    ($env:expr) => {
        $crate::Map::new($env)
    };
    ($env:expr, $(($k:expr, $v:expr $(,)?)),+ $(,)?) => {
        $crate::Map::from_array($env, [$(($k, $v)),+])
    };
}

/// Map is a key-value dictionary.
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
/// Map values can be stored as [ContractData], or in other
/// types like [Vec], [Map], etc.
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
#[repr(transparent)]
#[derive(Clone)]
pub struct Map<K, V>(EnvObj, PhantomData<K>, PhantomData<V>);

impl<K, V> Eq for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
}

impl<K, V> PartialEq for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl<K, V> PartialOrd for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<K, V> Ord for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl<K, V> Debug for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Debug + Clone,
    K::Error: Debug,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Debug + Clone,
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

impl<K, V> TryFrom<EnvVal> for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<K, V> TryFrom<EnvObj> for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_object().is_obj_type(ScObjectType::Map) {
            Ok(Map(obj, PhantomData, PhantomData))
        } else {
            Err(ConversionError {})
        }
    }
}

impl<K, V> TryIntoVal<Env, Map<K, V>> for RawVal
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;

    fn try_into_val(self, env: &Env) -> Result<Map<K, V>, Self::Error> {
        EnvType {
            env: env.clone(),
            val: self,
        }
        .try_into()
    }
}

impl<K, V> From<Map<K, V>> for RawVal
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K, V> From<Map<K, V>> for EnvVal
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K, V> From<Map<K, V>> for EnvObj
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0
    }
}

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
impl<K, V> TryIntoVal<Env, Map<K, V>> for ScVal
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Map<K, V>, Self::Error> {
        let o: Object = self.try_into_val(env).map_err(|_| ConversionError)?;
        let env = env.clone();
        EnvObj { val: o, env }.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl<K, V> TryFrom<EnvType<ScVal>> for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Error = ConversionError;
    fn try_from(v: EnvType<ScVal>) -> Result<Self, Self::Error> {
        ScVal::try_into_val(v.val, &v.env)
    }
}

impl<K, V> Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj, PhantomData, PhantomData)
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
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
            map.set(k, v);
        }
        map
    }

    #[inline(always)]
    pub fn contains_key(&self, k: K) -> bool {
        let env = self.env();
        let has = env.map_has(self.0.to_object(), k.into_val(env));
        has.is_true()
    }

    #[inline(always)]
    pub fn get(&self, k: K) -> Option<Result<V, V::Error>> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.0.to_object(), k);
        if has.is_true() {
            let v = env.map_get(self.0.to_object(), k);
            Some(V::try_from_val(env, v))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_unchecked(&self, k: K) -> Result<V, V::Error> {
        let env = self.env();
        let v = env.map_get(self.0.to_object(), k.into_val(env));
        V::try_from_val(env, v)
    }

    #[inline(always)]
    pub fn set(&mut self, k: K, v: V) {
        let env = self.env();
        let map = env.map_put(self.0.to_object(), k.into_val(env), v.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn remove(&mut self, k: K) -> Option<()> {
        let env = self.env();
        let k = k.into_val(env);
        let has = env.map_has(self.0.to_object(), k);
        if has.is_true() {
            let map = env.map_del(self.0.to_object(), k);
            self.0 = map.in_env(env);
            Some(())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn remove_unchecked(&mut self, k: K) {
        let env = self.env();
        let map = env.map_del(self.0.to_object(), k.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        let env = self.env();
        let len = env.map_len(self.0.to_object());
        len.is_u32_zero()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let len = env.map_len(self.0.to_object());
        u32::try_from_val(env, len).unwrap()
    }

    #[inline(always)]
    pub fn keys(&self) -> Vec<K> {
        let env = self.env();
        let vec = env.map_keys(self.0.to_object());
        Vec::<K>::try_from_val(env, vec).unwrap()
    }

    #[inline(always)]
    pub fn values(&self) -> Vec<V> {
        let env = self.env();
        let vec = env.map_values(self.0.to_object());
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
        K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
        K::Error: Debug,
        V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
        V::Error: Debug,
    {
        self.iter().unchecked()
    }

    #[inline(always)]
    pub fn into_iter_unchecked(self) -> UncheckedIter<MapIter<K, V>, (K, V), ConversionError>
    where
        K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
        K::Error: Debug,
        V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> + Clone,
        V::Error: Debug,
    {
        self.into_iter().unchecked()
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
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
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    type Item = Result<(K, V), ConversionError>;

    fn next(&mut self) -> Option<Self::Item> {
        let env = &self.0 .0.env;
        let key = env.map_min_key(self.0 .0.to_object());
        if Status::try_from(key).is_ok() {
            return None;
        }
        let value = env.map_get(self.0 .0.to_object(), key);
        self.0 .0.val = env.map_del(self.0 .0.to_object(), key);
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
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let env = &self.0 .0.env;
        let key = env.map_max_key(self.0 .0.to_object());
        if Status::try_from(key).is_ok() {
            return None;
        }
        let value = env.map_get(self.0 .0.to_object(), key);
        self.0 .0.val = env.map_del(self.0 .0.to_object(), key);
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
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
{
}

impl<K, V> ExactSizeIterator for MapIter<K, V>
where
    K: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
    V: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal>,
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
}
