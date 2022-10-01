use core::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use crate::{
    env::internal::{Env as _, RawVal, RawValConvertible},
    env::EnvObj,
    xdr::ScObjectType,
    BytesN, ConversionError, Env, IntoVal, Object, TryFromVal, TryIntoVal,
};

/// Account ID is a Stellar account ID.
///
/// The ID is opaque and does not expose the identifier to the contract, but the
/// value is unique and can be used as a key in maps, or compared with other
/// account identifiers.
#[derive(Clone)]
pub struct AccountId(EnvObj);

impl Debug for AccountId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccountId(..)")?;
        Ok(())
    }
}

impl Eq for AccountId {}

impl PartialEq for AccountId {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for AccountId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for AccountId {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl TryFromVal<Env, Object> for AccountId {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: Object) -> Result<Self, Self::Error> {
        if val.is_obj_type(ScObjectType::AccountId) {
            Ok(AccountId(val.in_env(env)))
        } else {
            Err(ConversionError {})
        }
    }
}

impl TryIntoVal<Env, AccountId> for Object {
    type Error = <AccountId as TryFromVal<Env, Object>>::Error;

    fn try_into_val(self, env: &Env) -> Result<AccountId, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, self)
    }
}

impl TryFromVal<Env, RawVal> for AccountId {
    type Error = <AccountId as TryFromVal<Env, Object>>::Error;

    fn try_from_val(env: &Env, val: RawVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, val.try_into()?)
    }
}

impl TryIntoVal<Env, AccountId> for RawVal {
    type Error = <AccountId as TryFromVal<Env, Object>>::Error;

    fn try_into_val(self, env: &Env) -> Result<AccountId, Self::Error> {
        <_ as TryFromVal<_, RawVal>>::try_from_val(env, self)
    }
}

impl IntoVal<Env, Object> for AccountId {
    fn into_val(self, _env: &Env) -> Object {
        self.to_object()
    }
}

impl IntoVal<Env, Object> for &AccountId {
    fn into_val(self, _env: &Env) -> Object {
        self.to_object()
    }
}

impl IntoVal<Env, RawVal> for AccountId {
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
    }
}

impl IntoVal<Env, RawVal> for &AccountId {
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
    }
}

#[cfg(not(target_family = "wasm"))]
use super::xdr::ScVal;

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&AccountId> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &AccountId) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.0.env, v.0.val.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<AccountId> for ScVal {
    type Error = ConversionError;
    fn try_from(v: AccountId) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for AccountId {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(
            env,
            val.try_into_val(env).map_err(|_| ConversionError)?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, AccountId> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<AccountId, Self::Error> {
        AccountId::try_from_val(env, self)
    }
}

impl AccountId {
    pub(crate) unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj)
    }

    pub fn env(&self) -> &Env {
        self.0.env()
    }

    pub fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub fn as_object(&self) -> &Object {
        self.0.as_object()
    }

    pub fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub fn to_object(&self) -> Object {
        self.0.to_object()
    }
}

/// Account references a Stellar account and provides access to information
/// about the account, such as its thresholds and signers.
#[derive(Clone)]
pub struct Account(AccountId);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AccountError {
    DoesNotExist,
}

impl Display for AccountError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AccountError::DoesNotExist => write!(f, "account does not exist"),
        }
    }
}

impl Debug for Account {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Account(")?;
        Debug::fmt(&self.0, f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl Eq for Account {}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Account {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl AsRef<AccountId> for Account {
    fn as_ref(&self) -> &AccountId {
        &self.0
    }
}

impl Account {
    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub(crate) fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub(crate) fn as_object(&self) -> &Object {
        self.0.as_object()
    }

    pub(crate) fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub(crate) fn to_object(&self) -> Object {
        self.0.to_object()
    }

    /// Creates an account from a public key.
    pub fn from_id(id: &AccountId) -> Result<Account, AccountError> {
        let env = id.env();
        if env.account_exists(id.to_object()).is_false() {
            return Err(AccountError::DoesNotExist);
        }
        Ok(Account(id.clone()))
    }

    pub fn to_id(&self) -> AccountId {
        self.0.clone()
    }

    /// Returns if the account exists.
    pub fn exists(id: &AccountId) -> bool {
        let env = id.env();
        env.account_exists(id.to_object()).is_true()
    }

    /// Returns the low threshold for the Stellar account.
    pub fn low_threshold(&self) -> u8 {
        let env = self.env();
        let val = env.account_get_low_threshold(self.to_object());
        let threshold_u32 = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) };
        threshold_u32 as u8
    }

    /// Returns the medium threshold for the Stellar account.
    pub fn medium_threshold(&self) -> u8 {
        let env = self.env();
        let val = env.account_get_medium_threshold(self.to_object());
        let threshold_u32 = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) };
        threshold_u32 as u8
    }

    /// Returns the high threshold for the Stellar account.
    pub fn high_threshold(&self) -> u8 {
        let env = self.env();
        let val = env.account_get_high_threshold(self.to_object());
        let threshold_u32 = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) };
        threshold_u32 as u8
    }

    /// Returns the signer weight for the signer for this Stellar account. If
    /// the signer does not exist for the account, returns zero (`0`).
    pub fn signer_weight(&self, signer: &BytesN<32>) -> u8 {
        let env = self.env();
        let val = env.account_get_signer_weight(self.to_object(), signer.to_object());
        let weight_u32 = unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) };
        weight_u32 as u8
    }
}
