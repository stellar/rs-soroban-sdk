use core::{cmp::Ordering, fmt::Debug};

use crate::{
    env::internal::{Env as _, RawVal, RawValConvertible},
    env::EnvObj,
    xdr::ScObjectType,
    BytesN, ConversionError, Env, IntoVal, Object, TryFromVal, TryIntoVal,
};

/// Accouns retrieves information about accounts that exist in the current
/// ledger.
///
/// ### Examples
///
/// TODO
#[derive(Clone)]
pub struct Accounts(Env);

impl Accounts {
    #[inline(always)]
    pub(crate) fn env(&self) -> &Env {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn new(env: &Env) -> Accounts {
        Accounts(env.clone())
    }

    /// Gets the account for the account ID.
    pub fn get(&self, id: &AccountId) -> Option<Account> {
        let env = id.env();
        if env.account_exists(id.to_object()).is_true() {
            Some(Account(id.clone()))
        } else {
            None
        }
    }
}

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

#[cfg(any(test, feature = "testutils"))]
use crate::{env::internal::xdr, testutils};

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Accounts for Accounts {
    fn generate(&self) -> xdr::AccountId {
        use rand::RngCore;
        let mut bytes: [u8; 32] = Default::default();
        rand::thread_rng().fill_bytes(&mut bytes);
        xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(bytes)))
    }

    fn create(&self, id: &xdr::AccountId) {
        let env = self.env();
        env.host()
            .with_mut_storage(|storage| {
                let k = xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: id.clone(),
                });
                let v = xdr::LedgerEntry {
                    data: xdr::LedgerEntryData::Account(self.default_account_ledger_entry(id)),
                    last_modified_ledger_seq: 0,
                    ext: xdr::LedgerEntryExt::V0,
                };
                storage.put(&k, &v)
            })
            .unwrap();
    }

    fn set_thresholds(&self, id: &xdr::AccountId, low: u8, medium: u8, high: u8) {
        self.update_account_ledger_entry(id, |a| {
            a.thresholds.0[1] = low;
            a.thresholds.0[2] = medium;
            a.thresholds.0[3] = high;
        });
    }

    fn set_signer_weight(&self, id: &crate::xdr::AccountId, signer: &BytesN<32>, weight: u8) {
        self.update_account_ledger_entry(id, |a| {
            let mut signers = a.signers.to_vec();
            let mut found = false;
            for s in signers.iter_mut() {
                if let xdr::SignerKey::Ed25519(ed25519) = &s.key {
                    if signer == &ed25519.0 {
                        s.weight = weight.into();
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                signers.push(xdr::Signer {
                    key: xdr::SignerKey::Ed25519(xdr::Uint256(signer.into_val(self.env()))),
                    weight: weight.into(),
                })
            }
            a.signers = signers.try_into().unwrap();
        });
    }

    fn remove(&self, id: &xdr::AccountId) {
        let env = self.env();
        env.host()
            .with_mut_storage(|storage| {
                let k = xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: id.clone(),
                });
                storage.del(&k)
            })
            .unwrap();
    }
}

#[cfg(any(test, feature = "testutils"))]
impl Accounts {
    #[cfg(any(test, feature = "testutils"))]
    fn default_account_ledger_entry(&self, id: &xdr::AccountId) -> xdr::AccountEntry {
        xdr::AccountEntry {
            account_id: id.clone(),
            balance: 0,
            flags: 0,
            home_domain: xdr::VecM::default(),
            inflation_dest: None,
            num_sub_entries: 0,
            seq_num: xdr::SequenceNumber(0),
            thresholds: xdr::Thresholds([1; 4]),
            signers: xdr::VecM::default(),
            ext: xdr::AccountEntryExt::V0,
        }
    }

    fn update_account_ledger_entry<F>(&self, id: &xdr::AccountId, f: F)
    where
        F: FnOnce(&mut xdr::AccountEntry),
    {
        let env = self.env();
        env.host()
            .with_mut_storage(|storage| {
                let k = xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: id.clone(),
                });
                let mut v = storage
                    .get(&k)
                    .ok()
                    .map(|v| {
                        if let xdr::LedgerEntryData::Account(_) = v.data {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .unwrap_or_else(|| xdr::LedgerEntry {
                        data: xdr::LedgerEntryData::Account(self.default_account_ledger_entry(id)),
                        last_modified_ledger_seq: 0,
                        ext: xdr::LedgerEntryExt::V0,
                    });
                if let xdr::LedgerEntryData::Account(ref mut a) = &mut v.data {
                    f(a);
                } else {
                    panic!("ledger entry is not an account");
                }
                storage.put(&k, &v)
            })
            .unwrap();
    }
}
