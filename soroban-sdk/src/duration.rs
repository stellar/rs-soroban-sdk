use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use soroban_env_host::DurationObject;

use super::{
    env::internal::{Env as _, EnvBase as _},
    ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
use crate::unwrap::UnwrapInfallible;

#[derive(Clone)]
pub struct Duration {
    env: Env,
    obj: DurationObject,
}

impl Debug for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "Duration(..)")?;
        #[cfg(not(target_family = "wasm"))]
        {
            todo!()
        }
    }
}

impl Eq for Duration {}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.env.check_same_env(&other.env);
        let v = self
            .env
            .obj_cmp(self.obj.into(), other.obj.into())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl TryFromVal<Env, DurationObject> for Duration {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &DurationObject) -> Result<Self, Self::Error> {
        Ok(unsafe { Duration::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, RawVal> for Duration {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(DurationObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, Duration> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Duration) -> Result<Self, Self::Error> {
        Ok(v.obj.to_raw())
    }
}

impl TryFromVal<Env, &Duration> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&Duration) -> Result<Self, Self::Error> {
        Ok(v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Duration> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Duration) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.env, &v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Duration> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Duration) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for Duration {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            DurationObject::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

impl Duration {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: DurationObject) -> Self {
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

    pub fn as_object(&self) -> &DurationObject {
        &self.obj
    }

    pub fn to_object(&self) -> DurationObject {
        self.obj
    }
}
