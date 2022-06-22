use core::cmp::Ordering;

use super::{env::internal::Env as _, xdr::ScObjectType, Env, EnvObj, EnvVal, RawVal};

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
        if obj.as_tagged().is_obj_type(ScObjectType::Vec) {
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

    #[inline(always)]
    pub fn put(&mut self, i: u32, v: u8) {
        let v32: u32 = v.into();
        self.0 = self
            .env()
            .binary_put(self.0.to_tagged(), i.into(), v32.into())
            .in_env(self.env());
    }

    #[inline(always)]
    pub fn get(&self, i: u32) -> u8 {
        let res32: u32 = self
            .env()
            .binary_get(self.0.to_tagged(), i.into())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    #[inline(always)]
    pub fn del(&mut self, i: u32) {
        self.0 = self
            .env()
            .binary_del(self.0.to_tagged(), i.into())
            .in_env(self.env());
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env()
            .binary_len(self.0.to_tagged())
            .try_into()
            .unwrap()
    }

    pub fn push(&mut self, x: u8) {
        let x32: u32 = x.into();
        self.0 = self
            .env()
            .binary_push(self.0.to_tagged(), x32.into())
            .in_env(self.env());
    }

    pub fn pop(&mut self) {
        self.0 = self.env().binary_pop(self.0.to_tagged()).in_env(self.env());
    }

    pub fn front(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_front(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    pub fn back(&self) -> u8 {
        let res32: u32 = self
            .env()
            .binary_back(self.0.to_tagged())
            .try_into()
            .unwrap();
        res32.try_into().unwrap()
    }

    pub fn insert(&mut self, i: u32, x: u8) {
        let x32: u32 = x.into();
        self.0 = self
            .env()
            .binary_insert(self.0.to_tagged(), i.into(), x32.into())
            .in_env(self.env());
    }

    pub fn append(&mut self, other: &Binary) {
        self.0 = self
            .env()
            .binary_append(self.0.to_tagged(), other.0.to_tagged())
            .in_env(self.env());
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct FixedLengthBinary<const N: u32>(Binary);

impl<const N: u32> TryFrom<EnvVal> for FixedLengthBinary<N> {
    type Error = ();

    #[inline(always)]
    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.try_into()?;
        obj.try_into()
    }
}

impl<const N: u32> TryFrom<EnvObj> for FixedLengthBinary<N> {
    type Error = ();

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        let bin: Binary = obj.try_into()?;
        bin.try_into()
    }
}

impl<const N: u32> TryFrom<Binary> for FixedLengthBinary<N> {
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

impl<const N: u32> AsRef<Binary> for FixedLengthBinary<N> {
    #[inline(always)]
    fn as_ref(&self) -> &Binary {
        &self.0
    }
}

impl<const N: u32> From<FixedLengthBinary<N>> for RawVal {
    #[inline(always)]
    fn from(v: FixedLengthBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<FixedLengthBinary<N>> for EnvVal {
    #[inline(always)]
    fn from(v: FixedLengthBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<FixedLengthBinary<N>> for EnvObj {
    #[inline(always)]
    fn from(v: FixedLengthBinary<N>) -> Self {
        v.0.into()
    }
}

impl<const N: u32> From<FixedLengthBinary<N>> for Binary {
    #[inline(always)]
    fn from(v: FixedLengthBinary<N>) -> Self {
        v.0
    }
}
