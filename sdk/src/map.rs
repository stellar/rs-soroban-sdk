use core::{cmp::Ordering, fmt::Debug};

use super::{
    env::internal::Env as _, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal,
    IntoTryFromVal, IntoVal, RawVal, TryFromVal, Vec,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct Map(EnvObj);

impl Eq for Map {}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl TryFrom<EnvVal> for Map {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for Map {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Map) {
            Ok(Map(obj))
        } else {
            Err(ConversionError {})
        }
    }
}

impl From<Map> for RawVal {
    #[inline(always)]
    fn from(m: Map) -> Self {
        m.0.into()
    }
}

impl From<Map> for EnvVal {
    #[inline(always)]
    fn from(m: Map) -> Self {
        m.0.into()
    }
}

impl From<Map> for EnvObj {
    #[inline(always)]
    fn from(m: Map) -> Self {
        m.0
    }
}

impl Map {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Map {
        let obj = env.map_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    #[inline(always)]
    pub fn has<K: IntoVal<Env, RawVal>>(&self, k: K) -> bool {
        let env = self.env();
        let has = env.map_has(self.0.to_tagged(), k.into_val(env));
        bool::try_from_val(env, has).unwrap()
    }

    #[inline(always)]
    pub fn get<K: IntoVal<Env, RawVal>, V: TryFromVal<Env, RawVal>>(&self, k: K) -> V
    where
        V::Error: Debug,
    {
        let env = self.env();
        let v = env.map_get(self.0.to_tagged(), k.into_val(env));
        V::try_from_val(env, v).unwrap()
    }

    #[inline(always)]
    pub fn put<K: IntoVal<Env, RawVal>, V: IntoVal<Env, RawVal>>(&mut self, k: K, v: V) {
        let env = self.env();
        let map = env.map_put(self.0.to_tagged(), k.into_val(env), v.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn del<K: IntoVal<Env, RawVal>>(&mut self, k: K) {
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
    pub fn keys<K: IntoTryFromVal>(&self) -> Vec<K> {
        let env = self.env();
        let vec = env.map_keys(self.0.to_tagged());
        Vec::try_from_val(env, vec).unwrap()
    }
}
