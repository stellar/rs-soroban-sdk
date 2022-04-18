use core::marker::PhantomData;

use super::object::ObjType;

use super::host;
use super::val::ValType;
use super::OrAbort;
use super::{object::OBJ_VEC, status, Object, Status, Val};

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Vec<T>(Object, PhantomData<T>);

impl<V: ValType> TryFrom<Object> for Vec<V> {
    type Error = Status;

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        if obj.is_type(OBJ_VEC) {
            Ok(Vec(obj, PhantomData))
        } else {
            Err(status::UNKNOWN_ERROR)
        }
    }
}

impl<V: ValType> TryFrom<Val> for Vec<V> {
    type Error = Status;

    fn try_from(val: Val) -> Result<Self, Self::Error> {
        let obj: Object = val.try_into()?;
        obj.try_into()
    }
}

impl<T: ValType> From<Vec<T>> for Object {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0
    }
}

impl<T: ValType> From<Vec<T>> for Val {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<V: ValType> ObjType for Vec<V> {
    fn is_obj_type(obj: Object) -> bool {
        obj.is_type(OBJ_VEC)
    }

    unsafe fn unchecked_from_obj(obj: Object) -> Self {
        Self(obj, PhantomData)
    }
}

impl<T: ValType> Vec<T> {
    unsafe fn unchecked_new(obj: Object) -> Self {
        Self(obj, PhantomData)
    }

    #[inline(always)]
    pub fn new() -> Vec<T> {
        unsafe { host::vec::new().try_into().or_abort() }
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> T {
        let i: Val = i.into();
        unsafe { <T as ValType>::unchecked_from_val(host::vec::get(self.0.into(), i)) }
    }

    #[inline(always)]
    pub fn put(&self, i: u32, v: T) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::put(self.0.into(), i.into(), v.into())) }
    }

    #[inline(always)]
    pub fn del(&self, i: u32) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::del(self.0.into(), i.into())) }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        unsafe { host::vec::len(self.0.into()).as_u32() }
    }

    #[inline(always)]
    pub fn push(&self, x: T) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::push(self.0.into(), x.into())) }
    }

    #[inline(always)]
    pub fn pop(&self) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::pop(self.0.into())) }
    }

    #[inline(always)]
    pub fn take(&self, n: u32) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::take(self.0.into(), n.into())) }
    }

    #[inline(always)]
    pub fn drop(&self, n: u32) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::drop(self.0.into(), n.into())) }
    }

    #[inline(always)]
    pub fn front(&self) -> T {
        unsafe { <T as ValType>::unchecked_from_val(host::vec::front(self.0.into())) }
    }

    #[inline(always)]
    pub fn back(&self) -> T {
        unsafe { <T as ValType>::unchecked_from_val(host::vec::back(self.0.into())) }
    }

    #[inline(always)]
    pub fn insert(&self, i: u32, x: T) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::insert(self.0.into(), i.into(), x.into())) }
    }

    #[inline(always)]
    pub fn append(&self, other: Vec<T>) -> Vec<T> {
        unsafe { Self::unchecked_from_obj(host::vec::append(self.0.into(), other.into())) }
    }
}
