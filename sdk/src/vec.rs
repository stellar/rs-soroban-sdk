use core::marker::PhantomData;

use super::{xdr::ScObjectType, Env, EnvObj, EnvTrait, EnvValType, OrAbort, RawVal};

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec<T>(EnvObj, PhantomData<T>);

impl<V: EnvValType> TryFrom<EnvObj> for Vec<V> {
    type Error = ();

    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.is_obj_type(ScObjectType::ScoVec) {
            Ok(Vec(obj, PhantomData))
        } else {
            Err(())
        }
    }
}

impl<T: EnvValType> From<Vec<T>> for EnvObj {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0
    }
}

impl<T: EnvValType> From<Vec<T>> for RawVal {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<T: EnvValType> Vec<T> {
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj, PhantomData)
    }

    fn env(&self) -> &Env {
        &self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Vec<T> {
        let obj = env.vec_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> T {
        let env = self.env();
        let val = env.vec_get(self.0.as_raw_obj(), i.into());
        T::try_from_raw_val(env, val).or_abort()
    }

    // TODO: Do we need to check_same_env for the env potentially stored in
    // values of T? T values may be objects containing an Env?

    #[inline(always)]
    pub fn put(&mut self, i: u32, v: T) {
        let env = self.env();
        let vec = env.vec_put(self.0.as_raw_obj(), i.into(), v.into_raw_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn del(&mut self, i: u32) {
        let env = self.env();
        let vec = env.vec_del(self.0.as_raw_obj(), i.into());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let val = env.vec_len(self.0.as_raw_obj());
        u32::try_from(val).or_abort()
    }

    #[inline(always)]
    pub fn push(&mut self, x: T) {
        let env = self.env();
        let vec = env.vec_push(self.0.as_raw_obj(), x.into_raw_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn pop(&mut self) {
        let env = self.env();
        let vec = env.vec_pop(self.0.as_raw_obj());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn take(&mut self, n: u32) {
        let env = self.env();
        let vec = env.vec_take(self.0.as_raw_obj(), n.into());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn drop(&mut self, n: u32) {
        let env = self.0.env();
        let vec = env.vec_drop(self.0.as_raw_obj(), n.into());
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn front(&self) -> T {
        let env = self.0.env();
        let val = env.vec_front(self.0.as_raw_obj());
        T::try_from_raw_val(env, val).or_abort()
    }

    #[inline(always)]
    pub fn back(&self) -> T {
        let env = self.env();
        let val = env.vec_back(self.0.as_raw_obj());
        T::try_from_raw_val(env, val).or_abort()
    }

    #[inline(always)]
    pub fn insert(&mut self, i: u32, x: T) {
        let env = self.env();
        let vec = env.vec_put(self.0.as_raw_obj(), i.into(), x.into_raw_val(env));
        self.0 = vec.in_env(env);
    }

    #[inline(always)]
    pub fn append(&mut self, other: Vec<T>) {
        let env = self.env();
        let vec = env.vec_append(self.0.as_raw_obj(), other.0.as_raw_obj());
        self.0 = vec.in_env(env);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_raw_val_type() {
        let env = Env::default();

        let mut vec = Vec::<u32>::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push(10);
        assert_eq!(vec.len(), 1);
        vec.push(20);
        assert_eq!(vec.len(), 2);
        vec.push(30);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push(40);
        assert_eq!(vec_copy.len(), 4);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);
    }

    #[test]
    fn test_vec_env_val_type() {
        let env = Env::default();

        let mut vec = Vec::<i64>::new(&env);
        assert_eq!(vec.len(), 0);
        vec.push(-10);
        assert_eq!(vec.len(), 1);
        vec.push(20);
        assert_eq!(vec.len(), 2);
        vec.push(-30);
        assert_eq!(vec.len(), 3);

        let vec_ref = &vec;
        assert_eq!(vec_ref.len(), 3);

        let mut vec_copy = vec.clone();
        assert_eq!(vec_copy.len(), 3);
        vec_copy.push(40);
        assert_eq!(vec_copy.len(), 4);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec_ref.len(), 3);
    }
}
