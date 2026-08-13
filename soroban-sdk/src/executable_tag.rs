use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{Env as _, ExecutableTagObject},
    ConversionError, Env, String, TryFromVal, TryIntoVal, Val,
};

use crate::unwrap::UnwrapInfallible;
#[cfg(doc)]
use crate::{deploy::Deployer, deploy::DeployerWithAddress, storage::Persistent};

#[cfg(not(target_family = "wasm"))]
use super::xdr::{ScString, ScVal};

/// ExecutableTag is the key of an executable reference entry.
///
/// An executable reference entry is a persistent contract data entry keyed by
/// an `ExecutableTag` whose value is a Wasm hash. Contracts can use another
/// contract's executable reference entry as their own executable: their code
/// is then the Wasm the entry currently points at. Updating the entry to a
/// new Wasm hash causes the new Wasm to be used by all contracts that use the
/// entry as their executable. See [DeployerWithAddress::deploy_executable_ref] and
/// [Deployer::update_current_contract_executable_ref].
///
/// The protocol enforces additional checks on an entry keyed by an [ExecutableTag]:
///
/// - The entry must be stored with persistent durability. Any other
///   durability panics.
/// - The value must be the 32-byte hash of Wasm that has already been
///   uploaded. Storing any other value panics.
/// - The entry cannot be removed. [Persistent::remove] with a tag key panics.
#[derive(Clone)]
pub struct ExecutableTag {
    env: Env,
    obj: ExecutableTagObject,
}

impl Debug for ExecutableTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "ExecutableTag(..)")?;
        #[cfg(not(target_family = "wasm"))]
        write!(f, "ExecutableTag({self})")?;
        Ok(())
    }
}

#[cfg(not(target_family = "wasm"))]
impl core::fmt::Display for ExecutableTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let sc_val: ScVal = self.into();
        if let ScVal::ExecutableTag(ScString(s)) = sc_val {
            let utf8_s = s.to_utf8_string().unwrap();
            write!(f, "{utf8_s}")?;
        } else {
            panic!("value is not an executable tag");
        }
        Ok(())
    }
}

impl Eq for ExecutableTag {}

impl PartialEq for ExecutableTag {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for ExecutableTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for ExecutableTag {
    fn cmp(&self, other: &Self) -> Ordering {
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

impl TryFromVal<Env, ExecutableTag> for ExecutableTag {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &ExecutableTag) -> Result<Self, Self::Error> {
        Ok(v.clone())
    }
}

impl TryFromVal<Env, ExecutableTagObject> for ExecutableTag {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &ExecutableTagObject) -> Result<Self, Self::Error> {
        Ok(unsafe { ExecutableTag::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, Val> for ExecutableTag {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(ExecutableTagObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, ExecutableTag> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &ExecutableTag) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl TryFromVal<Env, &ExecutableTag> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&ExecutableTag) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl From<ExecutableTag> for Val {
    #[inline(always)]
    fn from(v: ExecutableTag) -> Self {
        v.obj.into()
    }
}

impl From<ExecutableTag> for ExecutableTagObject {
    #[inline(always)]
    fn from(v: ExecutableTag) -> Self {
        v.obj
    }
}

impl From<&ExecutableTag> for ExecutableTagObject {
    #[inline(always)]
    fn from(v: &ExecutableTag) -> Self {
        v.obj
    }
}

impl From<&ExecutableTag> for ExecutableTag {
    #[inline(always)]
    fn from(v: &ExecutableTag) -> Self {
        v.clone()
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<&ExecutableTag> for ScVal {
    fn from(v: &ExecutableTag) -> Self {
        // This conversion occurs only in test utilities, and theoretically all
        // values should convert to an ScVal because the Env won't let the host
        // type to exist otherwise, unwrapping. Even if there are edge cases
        // that don't, this is a trade off for a better test developer
        // experience.
        ScVal::try_from_val(&v.env, &v.obj.to_val()).unwrap()
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<ExecutableTag> for ScVal {
    fn from(v: ExecutableTag) -> Self {
        (&v).into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for ExecutableTag {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            ExecutableTagObject::try_from_val(env, &Val::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

impl ExecutableTag {
    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: ExecutableTagObject) -> Self {
        Self { env, obj }
    }

    /// Create the tag that keys the executable reference entry named `tag`.
    pub fn new(env: &Env, tag: &String) -> ExecutableTag {
        let obj = env
            .create_executable_tag(tag.to_object())
            .unwrap_infallible();
        unsafe { ExecutableTag::unchecked_new(env.clone(), obj) }
    }

    /// Create the tag that keys the executable reference entry named `tag`.
    pub fn from_str(env: &Env, tag: &str) -> ExecutableTag {
        Self::new(env, &String::from_str(env, tag))
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

    pub fn as_object(&self) -> &ExecutableTagObject {
        &self.obj
    }

    pub fn to_object(&self) -> ExecutableTagObject {
        self.obj
    }
}
