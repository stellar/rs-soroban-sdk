use core::marker::PhantomData;

use crate::object_type::ObjectType;

use super::{xdr::ScObjectType, Env, Object, RawVal, RawValType};

#[derive(Clone)]
#[repr(transparent)]
pub struct Vec<T>(Object, PhantomData<T>);

impl<V: RawValType> TryFrom<Object> for Vec<V> {
    type Error = ();

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        if obj.is_obj_type(ScObjectType::ScoVec) {
            Ok(Vec(obj, PhantomData))
        } else {
            Err(())
        }
    }
}

// impl<V: RawValType> TryFrom<RawVal> for Vec<V> {
//     type Error = ();

//     fn try_from(val: RawVal) -> Result<Self, Self::Error> {
//         let obj: Object = val.try_into()?;
//         obj.try_into()
//     }
// }

impl<T: RawValType> From<Vec<T>> for Object {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0
    }
}

impl<T: RawValType> From<Vec<T>> for RawVal {
    #[inline(always)]
    fn from(v: Vec<T>) -> Self {
        v.0.into()
    }
}

impl<V: RawValType> ObjectType for Vec<V> {
    fn is_obj_type(obj: Object) -> bool {
        obj.is_obj_type(ScObjectType::ScoVec)
    }

    unsafe fn unchecked_from_obj(obj: Object) -> Self {
        Self(obj, PhantomData)
    }
}

impl<T: RawValType> Vec<T> {
    unsafe fn unchecked_new(obj: Object) -> Self {
        Self(obj, PhantomData)
    }

    #[inline(always)]
    pub fn new() -> Vec<T> {
        // unsafe { host::vec::new().try_into().or_abort() }
        todo!()
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> T {
        // let i: RawVal = i.into();
        // unsafe { <T as RawValType>::unchecked_from_val(host::vec::get(self.0.into(), i)) }
        todo!()
    }

    #[inline(always)]
    pub fn put(&self, i: u32, v: T) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::put(self.0.into(), i.into(), v.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn del(&self, i: u32) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::del(self.0.into(), i.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        // unsafe { host::vec::len(self.0.into()).try_into().or_abort() }
        todo!()
    }

    #[inline(always)]
    pub fn push(&self, x: T) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::push(self.0.into(), x.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn pop(&self) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::pop(self.0.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn take(&self, n: u32) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::take(self.0.into(), n.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn drop(&self, n: u32) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::drop(self.0.into(), n.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn front(&self) -> T {
        // unsafe { <T as RawValType>::unchecked_from_val(host::vec::front(self.0.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn back(&self) -> T {
        // unsafe { <T as RawValType>::unchecked_from_val(host::vec::back(self.0.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn insert(&self, i: u32, x: T) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::insert(self.0.into(), i.into(), x.into())) }
        todo!()
    }

    #[inline(always)]
    pub fn append(&self, other: Vec<T>) -> Vec<T> {
        // unsafe { Self::unchecked_from_obj(host::vec::append(self.0.into(), other.into())) }
        todo!()
    }
}
