use crate::object::OBJ_I64;

use super::OrAbort;
use super::{host, status, BitSet, Object, Status, Symbol};

pub(crate) const TAG_U32: u8 = 0;
pub(crate) const TAG_I32: u8 = 1;
pub(crate) const TAG_STATIC: u8 = 2;
pub(crate) const TAG_OBJECT: u8 = 3;
pub(crate) const TAG_SYMBOL: u8 = 4;
pub(crate) const TAG_BITSET: u8 = 5;
pub(crate) const TAG_STATUS: u8 = 6;

const STATIC_VOID: u32 = 0;
const STATIC_TRUE: u32 = 1;
const STATIC_FALSE: u32 = 2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Val(u64);

pub trait ValType: Into<Val> {
    fn is_val_type(v: Val) -> bool;
    unsafe fn unchecked_from_val(v: Val) -> Self;
}

// Orphan rules mean we have to macro these, can't blanket-impl on V:Valtype.
macro_rules! declare_tryfrom {
    ($T:ty) => {
        impl TryFrom<Val> for $T {
            type Error = Status;
            fn try_from(v: Val) -> Result<Self, Self::Error> {
                if <Self as ValType>::is_val_type(v) {
                    Ok(unsafe { <Self as ValType>::unchecked_from_val(v) })
                } else {
                    Err(status::UNKNOWN_ERROR)
                }
            }
        }
    };
}

declare_tryfrom!(());
declare_tryfrom!(bool);
declare_tryfrom!(i32);
declare_tryfrom!(u32);
declare_tryfrom!(i64);
declare_tryfrom!(Object);
declare_tryfrom!(Symbol);
declare_tryfrom!(BitSet);
declare_tryfrom!(Status);

impl ValType for () {
    fn is_val_type(v: Val) -> bool {
        v.has_tag(TAG_STATIC) && v.get_body() == STATIC_VOID as u64
    }
    unsafe fn unchecked_from_val(_v: Val) -> Self {
        ()
    }
}

impl ValType for bool {
    fn is_val_type(v: Val) -> bool {
        v.has_tag(TAG_STATIC)
            && (v.get_body() == STATIC_TRUE as u64 || v.get_body() == STATIC_FALSE as u64)
    }
    unsafe fn unchecked_from_val(v: Val) -> Self {
        v.get_body() == STATIC_TRUE as u64
    }
}

impl ValType for u32 {
    fn is_val_type(v: Val) -> bool {
        v.has_tag(TAG_U32)
    }
    unsafe fn unchecked_from_val(v: Val) -> Self {
        v.get_body() as u32
    }
}

impl ValType for i32 {
    fn is_val_type(v: Val) -> bool {
        v.has_tag(TAG_I32)
    }
    unsafe fn unchecked_from_val(v: Val) -> Self {
        v.get_body() as i32
    }
}

impl ValType for i64 {
    // TODO: The ValType trait is not particularly efficient for i64 because it
    // has to perform its checks twice. It might be more efficient if the
    // ValType's first function returns an Optional<T> where T is a transform
    // function.
    fn is_val_type(v: Val) -> bool {
        v.is_u63() || (v.is_object() && v.as_object().is_type(OBJ_I64))
    }
    unsafe fn unchecked_from_val(v: Val) -> Self {
        if v.is_u63() {
            v.as_u63()
        } else {
            let o = v.as_object();
            host::i64::to_i64(o)
        }
    }
}

impl From<bool> for Val {
    #[inline(always)]
    fn from(b: bool) -> Self {
        Val::from_bool(b)
    }
}

impl From<()> for Val {
    #[inline(always)]
    fn from(_: ()) -> Self {
        Val::from_void()
    }
}

impl From<u32> for Val {
    #[inline(always)]
    fn from(u: u32) -> Self {
        Val::from_u32(u)
    }
}

impl From<i32> for Val {
    #[inline(always)]
    fn from(i: i32) -> Self {
        Val::from_i32(i)
    }
}

impl From<i64> for Val {
    #[inline(always)]
    fn from(i: i64) -> Self {
        if i >= 0 {
            Val((i as u64) << 1)
        } else {
            unsafe { host::i64::from_i64(i).into() }
        }
    }
}

impl Val {
    #[inline(always)]
    pub(crate) const fn is_u63(&self) -> bool {
        (self.0 & 1) == 0
    }

    #[inline(always)]
    pub(crate) const fn as_u63(&self) -> i64 {
        (self.0 >> 1) as i64
    }

    #[inline(always)]
    pub(crate) fn get_tag(&self) -> u8 {
        ((self.0 >> 1) & 7) as u8
    }

    #[inline(always)]
    pub(crate) const fn get_body(&self) -> u64 {
        self.0 >> 4
    }

    #[inline(always)]
    pub(crate) fn has_tag(&self, tag: u8) -> bool {
        !self.is_u63() && self.get_tag() == tag
    }

    #[inline(always)]
    pub fn is_void(self) -> bool {
        <() as ValType>::is_val_type(self)
    }

    #[inline(always)]
    pub fn is_bool(self) -> bool {
        <bool as ValType>::is_val_type(self)
    }

    #[inline(always)]
    pub fn as_bool(&self) -> bool {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_status(&self) -> bool {
        self.has_tag(TAG_STATUS)
    }

    #[inline(always)]
    pub fn as_status(&self) -> Status {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_u32(&self) -> bool {
        self.has_tag(TAG_U32)
    }

    #[inline(always)]
    pub unsafe fn as_u32_unchecked(&self) -> u32 {
        self.get_body() as u32
    }

    #[inline(always)]
    pub fn as_u32(&self) -> u32 {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_i32(&self) -> bool {
        self.has_tag(TAG_I32)
    }

    #[inline(always)]
    pub fn as_i32(&self) -> i32 {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_i64(&self) -> bool {
        self.is_u63() || (self.is_object() && self.as_object().is_type(OBJ_I64))
    }

    #[inline(always)]
    pub fn as_i64(&self) -> i64 {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_symbol(&self) -> bool {
        self.has_tag(TAG_SYMBOL)
    }

    #[inline(always)]
    pub fn as_symbol(&self) -> Symbol {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_bit_set(&self) -> bool {
        self.has_tag(TAG_BITSET)
    }

    #[inline(always)]
    pub fn as_bit_set(&self) -> BitSet {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub fn is_object(&self) -> bool {
        self.has_tag(TAG_OBJECT)
    }

    #[inline(always)]
    pub fn as_object(&self) -> Object {
        (*self).try_into().or_abort()
    }

    #[inline(always)]
    pub(crate) fn from_u63(i: i64) -> Val {
        i.try_into().or_abort()
    }

    #[inline(always)]
    // This does no checking, so it can be used in const fns
    // below; it should not be made public.
    pub(crate) const unsafe fn from_body_and_tag(body: u64, tag: u8) -> Val {
        Val(body << 4 | ((tag << 1) as u64) | 1)
    }

    #[inline(always)]
    pub const fn from_void() -> Val {
        unsafe { Val::from_body_and_tag(STATIC_VOID as u64, TAG_STATIC) }
    }

    #[inline(always)]
    pub const fn from_bool(b: bool) -> Val {
        let body = if b { STATIC_TRUE } else { STATIC_FALSE };
        unsafe { Val::from_body_and_tag(body as u64, TAG_STATIC) }
    }

    #[inline(always)]
    pub const fn from_status(e: u32) -> Val {
        unsafe { Val::from_body_and_tag(e as u64, TAG_STATUS) }
    }

    #[inline(always)]
    pub const fn from_u32(u: u32) -> Val {
        unsafe { Val::from_body_and_tag(u as u64, TAG_U32) }
    }

    #[inline(always)]
    pub const fn from_i32(i: i32) -> Val {
        unsafe { Val::from_body_and_tag((i as u32) as u64, TAG_I32) }
    }

    #[inline(always)]
    pub fn from_i64(i: i64) -> Val {
        i.into()
    }

    #[inline(always)]
    pub const fn from_symbol(s: Symbol) -> Val {
        s.0
    }

    #[inline(always)]
    pub fn from_bit_set(bits: BitSet) -> Val {
        bits.into()
    }

    #[inline(always)]
    pub fn from_object(obj: Object) -> Val {
        obj.into()
    }
}
