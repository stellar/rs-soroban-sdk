use core::marker::PhantomData;

use super::{
    xdr::ScObjectType, Env, EnvI, EnvValType, HasEnv, Object, OrAbort, RawVal, RawValType,
};

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec<T>(Object, PhantomData<T>);

impl<V: EnvValType + RawValType> TryFrom<Object> for Vec<V> {
    type Error = ();

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        if obj.is_obj_type(ScObjectType::ScoVec) {
            Ok(Vec(obj, PhantomData))
        } else {
            Err(())
        }
    }
}

impl<T: EnvValType + RawValType> From<Vec<T>> for Object {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0
    }
}

impl<T: EnvValType + RawValType> From<Vec<T>> for RawVal {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<T: EnvValType + RawValType> Vec<T> {
    unsafe fn unchecked_new(obj: Object) -> Self {
        Self(obj, PhantomData)
    }

    #[inline(always)]
    pub fn new(e: &Env) -> Vec<T> {
        let mut env = e.clone();
        let val = env.vec_new();
        let obj: Object = val.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(obj) }
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> T {
        let mut env = self.0.env().clone();
        let val = env.vec_get(self.0.as_ref().val, i.into());
        T::try_from_raw_val(env, val).or_abort()
    }

    // TODO: Do we need to check_same_env for the env potentially stored in
    // values of T? T values may be objects containing an Env?

    #[inline(always)]
    pub fn put(&self, i: u32, v: T) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_put(self.0.as_ref().val, i.into(), v.into_raw_val(env.clone()));
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn del(&self, i: u32) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_del(self.0.as_ref().val, i.into());
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let mut env = self.0.env().clone();
        let val = env.vec_len(self.0.as_ref().val);
        u32::try_from(val).or_abort()
    }

    #[inline(always)]
    pub fn push(&self, x: T) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_push(self.0.as_ref().val, x.into_raw_val(env.clone()));
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn pop(&self) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_pop(self.0.as_ref().val);
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn take(&self, n: u32) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_take(self.0.as_ref().val, n.into());
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn drop(&self, n: u32) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_drop(self.0.as_ref().val, n.into());
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn front(&self) -> T {
        let mut env = self.0.env().clone();
        let val = env.vec_front(self.0.as_ref().val);
        T::try_from_raw_val(env, val).or_abort()
    }

    #[inline(always)]
    pub fn back(&self) -> T {
        let mut env = self.0.env().clone();
        let val = env.vec_back(self.0.as_ref().val);
        T::try_from_raw_val(env, val).or_abort()
    }

    #[inline(always)]
    pub fn insert(&self, i: u32, x: T) -> Vec<T> {
        let mut env = self.0.env().clone();
        let vec = env.vec_put(self.0.as_ref().val, i.into(), x.into_raw_val(env.clone()));
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
    }

    #[inline(always)]
    pub fn append(&self, other: Vec<T>) -> Vec<T> {
        let mut env = self.0.env().clone();
        env.check_same_env(&other.0.as_ref().env);
        let vec = env.vec_append(self.0.as_ref().val, other.0.as_ref().val);
        let vec: Object = vec.in_env(&env).try_into().or_abort();
        unsafe { Self::unchecked_new(vec) }
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
        vec = vec.push(10);
        assert_eq!(vec.len(), 1);
        vec = vec.push(20);
        assert_eq!(vec.len(), 2);
        vec = vec.push(30);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.front(), 10);
        assert_eq!(vec.get(1), 20);
        assert_eq!(vec.back(), 30);
    }

    #[test]
    fn test_vec_env_val_type() {
        // TODO: Add test using a type that is not a RawValType, such as i64 or
        // u64.
    }
}
