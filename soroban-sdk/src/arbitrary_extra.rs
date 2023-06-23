//! Extra trait impls required by the bounds to `SorobanArbitrary`.
//!
//! These are in their own module so that they are defined even when "testutils"
//! is not configured, making type inference consistent between configurations.

use crate::ConversionError;
use crate::Error;
use crate::{Env, TryFromVal};

impl TryFromVal<Env, u32> for u32 {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &u32) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, i32> for i32 {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &i32) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, u64> for u64 {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &u64) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, i64> for i64 {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &i64) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, u128> for u128 {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &u128) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, i128> for i128 {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &i128) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, bool> for bool {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &bool) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, ()> for () {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &()) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}

impl TryFromVal<Env, Error> for Error {
    type Error = ConversionError;
    fn try_from_val(_env: &Env, v: &Error) -> Result<Self, Self::Error> {
        Ok(*v)
    }
}
