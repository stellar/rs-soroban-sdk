use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use soroban_env_host::U256Object;

use super::{
    env::internal::{Env as _, EnvBase as _},
    ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
use crate::unwrap::UnwrapInfallible;

#[derive(Clone)]
pub struct U256 {
    env: Env,
    obj: U256Object,
}

impl Debug for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "U256(..)")?;
        #[cfg(not(target_family = "wasm"))]
        {
            todo!()
        }
    }
}

impl Eq for U256 {}

impl PartialEq for U256 {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for U256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for U256 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.env.check_same_env(&other.env);
        let v = self
            .env
            .obj_cmp(self.obj.into(), other.obj.into())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl TryFromVal<Env, U256Object> for U256 {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &U256Object) -> Result<Self, Self::Error> {
        Ok(unsafe { U256::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, RawVal> for U256 {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(U256Object::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, U256> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &U256) -> Result<Self, Self::Error> {
        Ok(v.obj.to_raw())
    }
}

impl TryFromVal<Env, &U256> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&U256) -> Result<Self, Self::Error> {
        Ok(v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&U256> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &U256) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.env, &v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<U256> for ScVal {
    type Error = ConversionError;
    fn try_from(v: U256) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for U256 {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            U256Object::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

impl U256 {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: U256Object) -> Self {
        Self { env, obj }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_raw(&self) -> &RawVal {
        self.obj.as_raw()
    }

    pub fn to_raw(&self) -> RawVal {
        self.obj.to_raw()
    }

    pub fn as_object(&self) -> &U256Object {
        &self.obj
    }

    pub fn to_object(&self) -> U256Object {
        self.obj
    }
}
