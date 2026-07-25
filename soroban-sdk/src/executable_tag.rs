use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{Env as _, ExecutableTagObject},
    ConversionError, Env, String, TryFromVal, TryIntoVal, Val,
};

use crate::unwrap::UnwrapInfallible;
#[cfg(doc)]
use crate::{deploy::Deployer, storage::Storage};

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

/// ExecutableTag is the key identifying an executable reference entry.
///
/// An executable reference entry is a persistent contract data entry, owned by
/// some contract, whose value is the hash of an uploaded Wasm. Other contracts
/// may be created with — or updated to — an executable that points at that
/// entry, so that updating the one entry atomically re-points every contract
/// referring to it. See [Deployer::deploy_external_ref] and
/// [Deployer::update_current_contract_executable_ref].
///
/// A tag may only be created by [ExecutableTag::new], which calls the
/// `create_executable_tag` host function. There is deliberately no other way to
/// construct one: the protocol enforces extra rules on entries keyed by a tag,
/// and those rules are only sound if the key cannot be forged.
///
/// The rules the protocol enforces on an entry keyed by an [ExecutableTag] are:
///
/// - It cannot be deleted. [Storage::remove] on a tag key always panics.
/// - It must be stored with persistent durability. Any other durability panics.
/// - Its value must be the 32-byte hash of an *already uploaded* Wasm. Storing
///   anything else — including the hash of a Wasm that has not been uploaded —
///   panics.
///
/// Together these mean a live executable reference can never dangle.
#[derive(Clone)]
pub struct ExecutableTag {
    env: Env,
    obj: ExecutableTagObject,
}

impl Debug for ExecutableTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ExecutableTag(..)")
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
        // type to exist otherwise, unwrapping.
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

    /// Create the tag identifying the executable reference entry named by
    /// `tag`.
    ///
    /// Creating the tag does not create the entry, and does not require that
    /// the entry already exist. The tag is just the key; write the entry with
    /// [Storage::set] on persistent storage, using this tag as the key and the
    /// hash of an uploaded Wasm as the value.
    ///
    /// ### Examples
    ///
    /// ```
    /// use soroban_sdk::{contract, contractimpl, BytesN, Env, ExecutableTag, String};
    ///
    /// #[contract]
    /// pub struct Manager;
    ///
    /// #[contractimpl]
    /// impl Manager {
    ///     /// Point the executable reference named `name` at `wasm_hash`.
    ///     ///
    ///     /// Every contract whose executable refers to this entry picks up the
    ///     /// new Wasm as soon as this returns — however many of them there are.
    ///     pub fn set(env: Env, name: String, wasm_hash: BytesN<32>) {
    ///         let tag = ExecutableTag::new(&env, &name);
    ///         env.storage().persistent().set(&tag, &wasm_hash);
    ///     }
    /// }
    /// ```
    pub fn new(env: &Env, tag: &String) -> ExecutableTag {
        let obj = env
            .create_executable_tag(tag.to_object())
            .unwrap_infallible();
        unsafe { ExecutableTag::unchecked_new(env.clone(), obj) }
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

    pub fn to_object(&self) -> ExecutableTagObject {
        self.obj
    }
}
