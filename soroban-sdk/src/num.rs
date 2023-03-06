use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{
        DurationObject, Env as _, EnvBase as _, I256Object, TimepointObject, U256Object,
    },
    ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
use crate::unwrap::UnwrapInfallible;

macro_rules! impl_num_type_wrapping_object {
    ($wrapper:ident, $obj:ty) => {
        #[derive(Clone)]
        pub struct $wrapper {
            env: Env,
            obj: $obj,
        }

        impl Debug for $wrapper {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:?}", self.obj.as_raw())
            }
        }

        impl Eq for $wrapper {}

        impl PartialEq for $wrapper {
            fn eq(&self, other: &Self) -> bool {
                self.partial_cmp(other) == Some(Ordering::Equal)
            }
        }

        impl PartialOrd for $wrapper {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(Ord::cmp(self, other))
            }
        }

        impl Ord for $wrapper {
            fn cmp(&self, other: &Self) -> Ordering {
                self.env.check_same_env(&other.env);
                let v = self
                    .env
                    .obj_cmp(self.obj.into(), other.obj.into())
                    .unwrap_infallible();
                v.cmp(&0)
            }
        }

        impl TryFromVal<Env, $obj> for $wrapper {
            type Error = Infallible;

            fn try_from_val(env: &Env, val: &$obj) -> Result<Self, Self::Error> {
                Ok(unsafe { $wrapper::unchecked_new(env.clone(), *val) })
            }
        }

        impl TryFromVal<Env, RawVal> for $wrapper {
            type Error = ConversionError;

            fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
                Ok(<$obj>::try_from_val(env, val)?
                    .try_into_val(env)
                    .unwrap_infallible())
            }
        }

        impl TryFromVal<Env, $wrapper> for RawVal {
            type Error = ConversionError;

            fn try_from_val(_env: &Env, v: &$wrapper) -> Result<Self, Self::Error> {
                Ok(v.obj.to_raw())
            }
        }

        impl TryFromVal<Env, &$wrapper> for RawVal {
            type Error = ConversionError;

            fn try_from_val(_env: &Env, v: &&$wrapper) -> Result<Self, Self::Error> {
                Ok(v.obj.to_raw())
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl TryFrom<&$wrapper> for ScVal {
            type Error = ConversionError;
            fn try_from(v: &$wrapper) -> Result<Self, Self::Error> {
                ScVal::try_from_val(&v.env, &v.obj.to_raw())
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl TryFrom<$wrapper> for ScVal {
            type Error = ConversionError;
            fn try_from(v: $wrapper) -> Result<Self, Self::Error> {
                (&v).try_into()
            }
        }

        #[cfg(not(target_family = "wasm"))]
        impl TryFromVal<Env, ScVal> for $wrapper {
            type Error = ConversionError;
            fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
                Ok(<$obj>::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                    .try_into_val(env)
                    .unwrap_infallible())
            }
        }

        impl $wrapper {
            #[inline(always)]
            pub(crate) unsafe fn unchecked_new(env: Env, obj: $obj) -> Self {
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

            pub fn as_object(&self) -> &$obj {
                &self.obj
            }

            pub fn to_object(&self) -> $obj {
                self.obj
            }
        }
    };
}

impl_num_type_wrapping_object!(Duration, DurationObject);
impl_num_type_wrapping_object!(Timepoint, TimepointObject);
impl_num_type_wrapping_object!(U256, U256Object);
impl_num_type_wrapping_object!(I256, I256Object);
