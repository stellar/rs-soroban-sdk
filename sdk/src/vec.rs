use core::{cmp::Ordering, fmt::Debug};

use super::{
    env::internal::Env as _, xdr::ScObjectType, ConversionError, Env, EnvObj, EnvVal, IntoVal,
    RawVal, TryFromVal,
};

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec(EnvObj);

impl Eq for Vec {}

impl PartialEq for Vec {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Vec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Vec {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl TryFrom<EnvVal> for Vec {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for Vec {
    type Error = ConversionError;

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Vec) {
            Ok(unsafe { Vec::unchecked_new(obj) })
        } else {
            Err(ConversionError {})
        }
    }
}

impl From<Vec> for RawVal {
    #[inline(always)]
    fn from(v: Vec) -> Self {
        v.0.into()
    }
}

impl From<Vec> for EnvVal {
    fn from(v: Vec) -> Self {
        v.0.into()
    }
}

impl From<Vec> for EnvObj {
    #[inline(always)]
    fn from(v: Vec) -> Self {
        v.0
    }
}

impl Vec {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Vec {
        let obj = env.vec_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    #[inline(always)]
    pub fn get<T: TryFromVal<Env, RawVal>>(&self, i: u32) -> T
    where
        T::Error: Debug,
    {
        let env = self.env();
        let val = env.vec_get(self.0.to_tagged(), i.into());
        T::try_from_val(env, val).unwrap()
    }

    // TODO: Do we need to check_same_env for the env potentially stored in
    // values of T? T values may be objects containing an Env?

    #[inline(always)]
    pub fn put<T: IntoVal<Env, RawVal>>(&mut self, i: u32, v: T) {
        let env = self.env();
        let vec = env.vec_put(self.0.to_tagged(), i.into(), v.into_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn del(&mut self, i: u32) {
        let env = self.env();
        let vec = env.vec_del(self.0.to_tagged(), i.into());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let val = env.vec_len(self.0.to_tagged());
        u32::try_from(val).unwrap()
    }

    #[inline(always)]
    pub fn push<T: IntoVal<Env, RawVal>>(&mut self, x: T) {
        let env = self.env();
        let vec = env.vec_push(self.0.to_tagged(), x.into_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn pop(&mut self) {
        let env = self.env();
        let vec = env.vec_pop(self.0.to_tagged());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn front<T: TryFromVal<Env, RawVal>>(&self) -> T
    where
        T::Error: Debug,
    {
        let env = self.0.env();
        let val = env.vec_front(self.0.to_tagged());
        T::try_from_val(env, val).unwrap()
    }

    #[inline(always)]
    pub fn back<T: TryFromVal<Env, RawVal>>(&self) -> T
    where
        T::Error: Debug,
    {
        let env = self.env();
        let val = env.vec_back(self.0.to_tagged());
        T::try_from_val(env, val).unwrap()
    }

    #[inline(always)]
    pub fn insert<T: IntoVal<Env, RawVal>>(&mut self, i: u32, x: T) {
        let env = self.env();
        let vec = env.vec_put(self.0.to_tagged(), i.into(), x.into_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn append(&mut self, other: &Vec) {
        let env = self.env();
        let vec = env.vec_append(self.0.to_tagged(), other.0.to_tagged());
        self.0 = vec.in_env(env);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_raw_val_type() {
        let env = Env::default();

        let mut vec = Vec::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push(10u32);
        assert_eq!(vec.len(), 1);
        vec.push(20u32);
        assert_eq!(vec.len(), 2);
        vec.push(30u32);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert!(vec == vec_copy);
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push(40u32);
        assert_eq!(vec_copy.len(), 4);
        assert!(vec != vec_copy);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);

        vec_copy.pop();
        assert!(vec == vec_copy);
    }

    #[test]
    fn test_vec_env_val_type() {
        let env = Env::default();

        let mut vec = Vec::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push(-10i64);
        assert_eq!(vec.len(), 1);
        vec.push(20i64);
        assert_eq!(vec.len(), 2);
        vec.push(-30i64);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert!(vec == vec_copy);
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push(40i64);
        assert_eq!(vec_copy.len(), 4);
        assert!(vec != vec_copy);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);

        vec_copy.pop();
        assert!(vec == vec_copy);
    }

    #[test]
    fn test_vec_recursive() {
        let env = Env::default();

        let mut vec_inner = Vec::new(&env);
        vec_inner.push(-10i64);
        assert_eq!(vec_inner.len(), 1);

        let mut vec_outer = Vec::new(&env);
        vec_outer.push(vec_inner);
        assert_eq!(vec_outer.len(), 1);
    }
}
