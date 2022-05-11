use core::marker::PhantomData;

use super::{xdr::ScObjectType, EnvObj, EnvRawValConvertible, RawVal, Vec};

#[repr(transparent)]
#[derive(Clone)]
pub struct Map<K, V>(EnvObj, PhantomData<K>, PhantomData<V>);

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> TryFrom<EnvObj> for Map<K, V> {
    type Error = ();

    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::ScoMap) {
            Ok(Map(obj, PhantomData, PhantomData))
        } else {
            Err(())
        }
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> TryFrom<RawVal> for Map<K, V> {
    type Error = ();

    fn try_from(_val: RawVal) -> Result<Self, Self::Error> {
        // let obj: Object = val.try_into()?;
        // obj.try_into()
        todo!()
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> From<Map<K, V>> for EnvObj {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> From<Map<K, V>> for RawVal {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> Map<K, V> {
    #[inline(always)]
    pub fn new() -> Map<K, V> {
        // unsafe { Self::unchecked_from_obj(c.map_new().try_into().or_abort()) }
        todo!()
    }

    #[inline(always)]
    pub fn has(&self, _k: K) -> bool {
        // unsafe { <bool as EnvRawValConvertible<Env>>::unchecked_from_val(host::map::has(self.0.into(), k.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn get(&self, _k: K) -> V {
        // unsafe { <V as EnvRawValConvertible<Env>>::unchecked_from_val(host::map::get(self.0.into(), k.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn put(&mut self, _k: K, _v: V) {
        // unsafe { Self::unchecked_from_obj(host::map::put(self.0.into(), k.into(), v.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn del(&mut self, _k: K) {
        // unsafe { Self::unchecked_from_obj(host::map::del(self.0.into(), k.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        // let m: RawVal = unsafe { host::map::len(self.0.into()) };
        // m.try_into().or_abort()
        todo!()
    }

    #[inline(always)]
    pub fn keys(&self) -> Vec<K> {
        // let v: Object = unsafe { host::map::keys(self.0.into()) };
        // unsafe { <Vec<K> as ObjType>::unchecked_from_obj(v) }
        todo!()
    }
}
