use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use soroban_env_host::TimepointObject;

use super::{
    env::internal::{Env as _, EnvBase as _},
    ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
use crate::unwrap::UnwrapInfallible;

#[derive(Clone)]
pub struct Timepoint {
    env: Env,
    obj: TimepointObject,
}

impl Debug for Timepoint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "Timepoint(..)")?;
        #[cfg(not(target_family = "wasm"))]
        {
            todo!()
        }
    }
}

impl Eq for Timepoint {}

impl PartialEq for Timepoint {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Timepoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Timepoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.env.check_same_env(&other.env);
        let v = self
            .env
            .obj_cmp(self.obj.into(), other.obj.into())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl TryFromVal<Env, TimepointObject> for Timepoint {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &TimepointObject) -> Result<Self, Self::Error> {
        Ok(unsafe { Timepoint::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, RawVal> for Timepoint {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(TimepointObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, Timepoint> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Timepoint) -> Result<Self, Self::Error> {
        Ok(v.obj.to_raw())
    }
}

impl TryFromVal<Env, &Timepoint> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&Timepoint) -> Result<Self, Self::Error> {
        Ok(v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Timepoint> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Timepoint) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.env, &v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Timepoint> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Timepoint) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for Timepoint {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            TimepointObject::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

impl Timepoint {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: TimepointObject) -> Self {
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

    pub fn as_object(&self) -> &TimepointObject {
        &self.obj
    }

    pub fn to_object(&self) -> TimepointObject {
        self.obj
    }
}
