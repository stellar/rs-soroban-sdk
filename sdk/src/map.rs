use core::{cmp::Ordering, marker::PhantomData};

use super::{
    env::internal::Env as _, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal,
    IntoTryFromVal, RawVal, TryFromVal, Vec,
};

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
    pub fn has(&self, k: K) -> bool {
        let env = self.env();
        let has = env.map_has(self.0.to_tagged(), k.into_val(env));
        bool::try_from_val(env, has).unwrap()
    }

    #[inline(always)]
    pub fn get(&self, k: K) -> V {
        let env = self.env();
        let v = env.map_get(self.0.to_tagged(), k.into_val(env));
        V::try_from_val(env, v).ok().unwrap()
    }

    #[inline(always)]
    pub fn put(&mut self, k: K, v: V) {
        let env = self.env();
        let map = env.map_put(self.0.to_tagged(), k.into_val(env), v.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn del(&mut self, k: K) {
        let env = self.env();
        let map = env.map_del(self.0.to_tagged(), k.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let len = env.map_len(self.0.to_tagged());
        u32::try_from_val(env, len).unwrap()
    }

    #[inline(always)]
    pub fn keys(&self) -> Vec {
        let env = self.env();
        let vec = env.map_keys(self.0.to_tagged());
        Vec::try_from_val(env, vec).unwrap()
    }
}
