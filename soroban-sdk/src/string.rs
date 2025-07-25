use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{Env as _, EnvBase as _, StringObject},
    ConversionError, Env, TryFromVal, TryIntoVal, Val,
};

use crate::unwrap::{UnwrapInfallible, UnwrapOptimized};
#[cfg(doc)]
use crate::{storage::Storage, Map, Vec};

#[cfg(not(target_family = "wasm"))]
use super::xdr::{ScString, ScVal};

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
/// let s = String::from_str(&env, msg);
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
        #[cfg(target_family = "wasm")]
        write!(f, "String(..)")?;
        #[cfg(not(target_family = "wasm"))]
        write!(f, "String({})", self.to_string())?;
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
        #[cfg(not(target_family = "wasm"))]
        if !self.env.is_same_env(&other.env) {
            return ScVal::from(self).cmp(&ScVal::from(other));
        }
        let v = self
            .env
            .obj_cmp(self.obj.to_val(), other.obj.to_val())
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

impl TryFromVal<Env, Val> for String {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(StringObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, String> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &String) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl TryFromVal<Env, &String> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&String) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl From<String> for Val {
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
impl From<&String> for ScVal {
    fn from(v: &String) -> Self {
        // This conversion occurs only in test utilities, and theoretically all
        // values should convert to an ScVal because the Env won't let the host
        // type to exist otherwise, unwrapping. Even if there are edge cases
        // that don't, this is a trade off for a better test developer
        // experience.
        ScVal::try_from_val(&v.env, &v.obj.to_val()).unwrap()
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<String> for ScVal {
    fn from(v: String) -> Self {
        (&v).into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for String {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            StringObject::try_from_val(env, &Val::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

impl TryFromVal<Env, &str> for String {
    type Error = ConversionError;

    fn try_from_val(env: &Env, v: &&str) -> Result<Self, Self::Error> {
        Ok(String::from_str(env, v))
    }
}

#[cfg(not(target_family = "wasm"))]
impl ToString for String {
    fn to_string(&self) -> std::string::String {
        let sc_val: ScVal = self.try_into().unwrap();
        if let ScVal::String(ScString(s)) = sc_val {
            s.to_utf8_string().unwrap()
        } else {
            panic!("value is not a string");
        }
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

    pub fn as_val(&self) -> &Val {
        self.obj.as_val()
    }

    pub fn to_val(&self) -> Val {
        self.obj.to_val()
    }

    pub fn as_object(&self) -> &StringObject {
        &self.obj
    }

    pub fn to_object(&self) -> StringObject {
        self.obj
    }

    #[inline(always)]
    #[doc(hidden)]
    #[deprecated(note = "use from_str")]
    pub fn from_slice(env: &Env, slice: &str) -> String {
        Self::from_str(env, slice)
    }

    #[inline(always)]
    pub fn from_bytes(env: &Env, b: &[u8]) -> String {
        String {
            env: env.clone(),
            obj: env.string_new_from_slice(b).unwrap_optimized(),
        }
    }

    #[inline(always)]
    pub fn from_str(env: &Env, s: &str) -> String {
        String {
            env: env.clone(),
            obj: env.string_new_from_slice(s.as_bytes()).unwrap_optimized(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.env().string_len(self.obj).unwrap_infallible().into()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copy the bytes in [String] into the given slice.
    ///
    /// ### Panics
    ///
    /// If the output slice and string are of different lengths.
    #[inline(always)]
    pub fn copy_into_slice(&self, slice: &mut [u8]) {
        let env = self.env();
        if self.len() as usize != slice.len() {
            sdk_panic!("String::copy_into_slice with mismatched slice length")
        }
        env.string_copy_to_slice(self.to_object(), Val::U32_ZERO, slice)
            .unwrap_optimized();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::IntoVal;

    #[test]
    fn string_from_and_to_slices() {
        let env = Env::default();

        let msg = "a message";
        let s = String::from_str(&env, msg);
        let mut out = [0u8; 9];
        s.copy_into_slice(&mut out);
        assert_eq!(msg.as_bytes(), out)
    }

    #[test]
    fn string_from_and_to_bytes() {
        let env = Env::default();

        let msg = b"a message";
        let s = String::from_bytes(&env, msg);
        let mut out = [0u8; 9];
        s.copy_into_slice(&mut out);
        assert_eq!(msg, &out)
    }

    #[test]
    #[should_panic]
    fn string_to_short_slice() {
        let env = Env::default();
        let msg = "a message";
        let s = String::from_str(&env, msg);
        let mut out = [0u8; 8];
        s.copy_into_slice(&mut out);
    }

    #[test]
    #[should_panic]
    fn string_to_long_slice() {
        let env = Env::default();
        let msg = "a message";
        let s = String::from_str(&env, msg);
        let mut out = [0u8; 10];
        s.copy_into_slice(&mut out);
    }

    #[test]
    fn string_to_val() {
        let env = Env::default();

        let s = String::from_str(&env, "abcdef");
        let val: Val = s.clone().into_val(&env);
        let rt: String = val.into_val(&env);

        assert_eq!(s, rt);
    }

    #[test]
    fn ref_string_to_val() {
        let env = Env::default();

        let s = String::from_str(&env, "abcdef");
        let val: Val = (&s).into_val(&env);
        let rt: String = val.into_val(&env);

        assert_eq!(s, rt);
    }

    #[test]
    fn double_ref_string_to_val() {
        let env = Env::default();

        let s = String::from_str(&env, "abcdef");
        let val: Val = (&&s).into_val(&env);
        let rt: String = val.into_val(&env);

        assert_eq!(s, rt);
    }
}
