use core::cmp::Ordering;

use super::{env::internal::Env as _, xdr::ScObjectType, Env, EnvObj, EnvVal, RawVal};

pub trait FixedLengthBinary {
    fn put(&mut self, i: u32, v: u8);

    fn get(&self, i: u32) -> u8;

    fn len(&self) -> u32;

    fn front(&self) -> u8;

    fn back(&self) -> u8;
}

pub trait VariableLengthBinary: FixedLengthBinary {
    fn del(&mut self, i: u32);

    fn push(&mut self, x: u8);

    fn pop(&mut self);

    fn insert(&mut self, i: u32, x: u8);

    fn append(&mut self, other: &Binary);
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Binary(EnvObj);

impl Eq for Binary {}

impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Binary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Binary {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl TryFrom<EnvVal> for Binary {
    type Error = ();

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for Binary {
    type Error = ();

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::Binary) {
            Ok(unsafe { Binary::unchecked_new(obj) })
        } else {
            Err(())
        }
    }
}

impl From<Binary> for RawVal {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0.into()
    }
}

impl From<Binary> for EnvVal {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0.into()
    }
}

impl From<Binary> for EnvObj {
    #[inline(always)]
    fn from(v: Binary) -> Self {
        v.0
    }
}

impl FixedLengthBinary for Binary {
    #[inline(always)]
    fn put(&mut self, i: u32, v: u8) {
        let v32: u32 = v.into();
        self.0 = self
            .env()
            .binary_put(self.0.to_tagged(), i.into(), v32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn get(&self, i: u32) -> u8 {
        let res32: u32 = self
            .env()
            .binary_get(self.0.to_tagged(), i.into())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    fn len(&self) -> u32 {
        self.env()
            .binary_len(self.0.to_tagged())
            .try_into()
            .unwrap()
    }

    #[inline(always)]
    fn front(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_front(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    fn back(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_back(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }
}

impl VariableLengthBinary for Binary {
    #[inline(always)]
    fn del(&mut self, i: u32) {
        self.0 = self
            .env()
            .binary_del(self.0.to_tagged(), i.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn push(&mut self, x: u8) {
        let x32: u32 = x.into();
        self.0 = self
            .env()
            .binary_push(self.0.to_tagged(), x32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn pop(&mut self) {
        self.0 = self.env().binary_pop(self.0.to_tagged()).in_env(self.env());
    }

    #[inline(always)]
    fn insert(&mut self, i: u32, x: u8) {
        let x32: u32 = x.into();
        self.0 = self
            .env()
            .binary_insert(self.0.to_tagged(), i.into(), x32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    fn append(&mut self, other: &Binary) {
        self.0 = self
            .env()
            .binary_append(self.0.to_tagged(), other.0.to_tagged())
            .in_env(self.env());
    }
}

impl Binary {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Binary {
        let obj = env.binary_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct ArrayBinary<const N: u32>(Binary);

impl<const N: u32> FixedLengthBinary for ArrayBinary<N> {
    #[inline(always)]
    fn put(&mut self, i: u32, v: u8) {
        self.0.put(i, v);
    }

    #[inline(always)]
    fn get(&self, i: u32) -> u8 {
        self.0.get(i)
    }

    #[inline(always)]
    fn len(&self) -> u32 {
        N
    }

    #[inline(always)]
    fn front(&self) -> u8 {
        self.0.front()
    }

    #[inline(always)]
    fn back(&self) -> u8 {
        self.0.back()
    }
}

impl<const N: u32> TryFrom<EnvVal> for ArrayBinary<N> {
    type Error = ();

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<const N: u32> TryFrom<EnvObj> for ArrayBinary<N> {
    type Error = ();

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        let bin: Binary = obj.try_into()?;
        bin.try_into()
    }
}

impl<const N: u32> TryFrom<Binary> for ArrayBinary<N> {
    type Error = ();

    #[inline(always)]
    fn try_from(bin: Binary) -> Result<Self, Self::Error> {
        if bin.len() == N {
            Ok(Self(bin))
        } else {
            Err(())
        }
    }
}

impl<const N: u32> From<ArrayBinary<N>> for RawVal {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<ArrayBinary<N>> for EnvVal {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<ArrayBinary<N>> for EnvObj {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<ArrayBinary<N>> for Binary {
    #[inline(always)]
    fn from(v: ArrayBinary<N>) -> Self {
        v.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bin() {
        let env = Env::default();

        let mut bin = Binary::new(&env);
        assert_eq!(bin.len(), 0);
        bin.push(10);
        assert_eq!(bin.len(), 1);
        bin.push(20);
        assert_eq!(bin.len(), 2);
        bin.push(30);
        assert_eq!(bin.len(), 3);

        let bin_ref = &bin;
        assert_eq!(bin_ref.len(), 3);

        let mut bin_copy = bin.clone();
        assert!(bin == bin_copy);
        assert_eq!(bin_copy.len(), 3);
        bin_copy.push(40);
        assert_eq!(bin_copy.len(), 4);
        assert!(bin != bin_copy);

        assert_eq!(bin.len(), 3);
        assert_eq!(bin_ref.len(), 3);

        bin_copy.pop();
        assert!(bin == bin_copy);

        let bad_fixed: Result<ArrayBinary<4>, ()> = bin.try_into();
        assert!(!bad_fixed.is_ok());
        let _fixed: ArrayBinary<3> = bin_copy.try_into().unwrap();
    }
}
