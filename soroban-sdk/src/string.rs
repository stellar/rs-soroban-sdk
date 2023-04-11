use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{Env as _, EnvBase as _, StringObject},
    ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

use crate::unwrap::{UnwrapInfallible, UnwrapOptimized};
#[cfg(doc)]
use crate::{storage::Storage, Map, Vec};

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

/// String is a contiguous growable array type containing `u8`s.
///
/// The array is stored in the Host and available to the Guest through the
/// functions defined on String.
///
/// String values can be stored as [Storage], or in other types like [Vec],
/// [Map], etc.
///
/// ### Examples
///
/// String values can be created from slices:
/// ```
/// use soroban_sdk::{String, Env};
///
/// let env = Env::default();
/// let msg = "a message";
/// let s = String::from_slice(&env, msg);
/// let mut out = [0u8; 9];
/// s.copy_into_slice(&mut out);
/// assert_eq!(msg.as_bytes(), out)
/// ```
#[derive(Clone)]
pub struct String {
    env: Env,
    obj: StringObject,
}

impl Debug for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "String()")?;
        Ok(())
    }
}

impl Eq for String {}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for String {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.env.check_same_env(&other.env);
        let v = self
            .env
            .obj_cmp(self.obj.to_raw(), other.obj.to_raw())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl TryFromVal<Env, String> for String {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &String) -> Result<Self, Self::Error> {
        Ok(v.clone())
    }
}

impl TryFromVal<Env, StringObject> for String {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &StringObject) -> Result<Self, Self::Error> {
        Ok(unsafe { String::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, RawVal> for String {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(StringObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, String> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &String) -> Result<Self, Self::Error> {
        Ok(v.to_raw())
    }
}

impl From<String> for RawVal {
    #[inline(always)]
    fn from(v: String) -> Self {
        v.obj.into()
    }
}

impl From<String> for StringObject {
    #[inline(always)]
    fn from(v: String) -> Self {
        v.obj
    }
}

impl From<&String> for StringObject {
    #[inline(always)]
    fn from(v: &String) -> Self {
        v.obj
    }
}

impl From<&String> for String {
    #[inline(always)]
    fn from(v: &String) -> Self {
        v.clone()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&String> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &String) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.env, &v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<String> for ScVal {
    type Error = ConversionError;
    fn try_from(v: String) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for String {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            StringObject::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

impl TryFromVal<Env, &str> for String {
    type Error = ConversionError;

    fn try_from_val(env: &Env, v: &&str) -> Result<Self, Self::Error> {
        Ok(String::from_slice(env, v))
    }
}

impl String {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: StringObject) -> Self {
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

    pub(crate) fn as_object(&self) -> &StringObject {
        &self.obj
    }

    pub(crate) fn to_object(&self) -> StringObject {
        self.obj
    }

    #[inline(always)]
    pub fn from_slice(env: &Env, slice: &str) -> String {
        String {
            env: env.clone(),
            obj: env.string_new_from_slice(slice).unwrap_optimized(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env().string_len(self.obj).unwrap_infallible().into()
    }

    /// Copy the bytes in [String] into the given slice.
    ///
    /// The minimum number of bytes are copied to either exhaust [String] or fill
    /// slice.
    #[inline(always)]
    pub fn copy_into_slice(&self, slice: &mut [u8]) {
        let env = self.env();
        env.string_copy_to_slice(self.to_object(), RawVal::U32_ZERO, slice)
            .unwrap_optimized();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_from_and_to_slices() {
        let env = Env::default();

        let msg = "a message";
        let s = String::from_slice(&env, msg);
        let mut out = [0u8; 9];
        s.copy_into_slice(&mut out);
        assert_eq!(msg.as_bytes(), out)
    }
}
