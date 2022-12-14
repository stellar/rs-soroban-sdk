//! Accounts contains types for accessing accounts in the current ledger.
//!
//! See [`Accounts`][Accounts] for examples.
use core::{cmp::Ordering, fmt::Debug};

use crate::{
    env::internal::xdr,
    env::internal::{Env as _, EnvBase as _, RawVal, RawValConvertible},
    BytesN, ConversionError, Env, IntoVal, Object, TryFromVal, TryIntoVal,
};

/// Accounts retrieves information about accounts that exist in the current
/// ledger.
///
/// ### Examples
///
/// ```
/// use soroban_sdk::Env;
///
/// # use soroban_sdk::{contractimpl, AccountId, BytesN};
/// #
/// # pub struct Contract;
/// #
/// # #[contractimpl]
/// # impl Contract {
/// #     pub fn f(env: Env, account_id: AccountId) {
/// let account = env.accounts().get(&account_id).expect("account does not exist");
/// assert_eq!(account.medium_threshold(), 1);
/// #     }
/// # }
/// #
/// # #[cfg(feature = "testutils")]
/// # fn main() {
/// #     use soroban_sdk::testutils::Accounts;
/// #     let env = Env::default();
/// #     let contract_id = BytesN::from_array(&env, &[0; 32]);
/// #     env.register_contract(&contract_id, Contract);
/// #     let account_id = env.accounts().generate();
/// #     env.accounts().create(&account_id);
/// #     ContractClient::new(&env, &contract_id).f(&account_id);
/// # }
/// # #[cfg(not(feature = "testutils"))]
/// # fn main() { }
/// ```
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

/// Account ID is an identifier for an account.
///
/// The ID is opaque and does not expose the identifier to the contract, but the
/// value can be used as a key in maps, or compared with other account
/// identifiers.
///
/// In tests account identifiers can be generated using [`Accounts`].
#[derive(Clone)]
pub struct AccountId {
    env: Env,
    obj: Object,
}

impl Debug for AccountId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "AccountId(..)")?;
        #[cfg(not(target_family = "wasm"))]
        {
            use stellar_strkey::StrkeyPublicKeyEd25519;
            let account_id: xdr::AccountId = self.try_into().map_err(|_| core::fmt::Error)?;
            let xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(ed25519))) =
                account_id;
            let strkey = StrkeyPublicKeyEd25519(ed25519);
            write!(f, "AccountId({})", strkey.to_string())?;
        }
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
        self.env.check_same_env(&other.env);
        let v = self.env.obj_cmp(self.obj.to_raw(), other.obj.to_raw());
        v.cmp(&0)
    }
}

impl TryFromVal<Env, Object> for AccountId {
    type Error = ConversionError;

    fn try_from_val(env: &Env, obj: Object) -> Result<Self, Self::Error> {
        if obj.is_obj_type(xdr::ScObjectType::AccountId) {
            Ok(AccountId {
                env: env.clone(),
                obj,
            })
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
        ScVal::try_from_val(&v.env, v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&AccountId> for super::xdr::AccountId {
    type Error = ConversionError;
    fn try_from(v: &AccountId) -> Result<Self, Self::Error> {
        let id: ScVal = v.to_raw().try_into_val(v.env()).unwrap();
        id.try_into().map_err(|_| ConversionError)
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
impl TryFrom<AccountId> for super::xdr::AccountId {
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
impl TryFromVal<Env, super::xdr::AccountId> for AccountId {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: super::xdr::AccountId) -> Result<Self, Self::Error> {
        let val: ScVal = val.try_into()?;
        val.try_into_val(env).map_err(|_| ConversionError)
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, AccountId> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<AccountId, Self::Error> {
        AccountId::try_from_val(env, self)
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, AccountId> for super::xdr::AccountId {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<AccountId, Self::Error> {
        AccountId::try_from_val(env, self)
    }
}

impl AccountId {
    pub(crate) unsafe fn unchecked_new(env: Env, obj: Object) -> Self {
        Self { env, obj }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_raw(&self) -> &RawVal {
        self.obj.as_raw()
    }

    pub fn as_object(&self) -> &Object {
        &self.obj
    }

    pub fn to_raw(&self) -> RawVal {
        self.obj.to_raw()
    }

    pub fn to_object(&self) -> Object {
        self.obj
    }
}

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl crate::testutils::AccountId for AccountId {
    fn random(env: &Env) -> AccountId {
        xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(
            crate::testutils::random(),
        )))
        .try_into_val(env)
        .unwrap()
    }
}

/// Account provides access to information about an accounts thresholds and
/// signers.
///
/// In tests accounts can be generated using [`Accounts`].
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
use crate::testutils::{self, AccountId as _};

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Accounts for Accounts {
    fn generate_and_create(&self) -> AccountId {
        let id = self.generate();
        self.create(&id);
        id
    }

    fn generate(&self) -> AccountId {
        AccountId::random(self.env())
    }

    fn create(&self, id: &AccountId) {
        let env = self.env();
        let id: xdr::AccountId = id.try_into().unwrap();
        env.host()
            .with_mut_storage(|storage| {
                let k = xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: id.clone(),
                });
                let v = xdr::LedgerEntry {
                    data: xdr::LedgerEntryData::Account(self.default_account_ledger_entry(&id)),
                    last_modified_ledger_seq: 0,
                    ext: xdr::LedgerEntryExt::V0,
                };
                storage.put(
                    &k,
                    &v,
                    soroban_env_host::budget::AsBudget::as_budget(env.host()),
                )
            })
            .unwrap();
    }

    fn set_thresholds(&self, id: &AccountId, low: u8, medium: u8, high: u8) {
        let id: xdr::AccountId = id.try_into().unwrap();
        self.update_account_ledger_entry(&id, |a| {
            a.thresholds.0[1] = low;
            a.thresholds.0[2] = medium;
            a.thresholds.0[3] = high;
        });
    }

    fn set_signer_weight(&self, id: &AccountId, signer: &BytesN<32>, weight: u8) {
        let id: xdr::AccountId = id.try_into().unwrap();
        self.update_account_ledger_entry(&id, |a| {
            let xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(account_id_ed25519)) =
                a.account_id.0;
            if signer == &account_id_ed25519 {
                // Master key.
                a.thresholds.0[0] = weight;
            } else {
                // Additional signer.
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
            }
        });
    }

    fn remove(&self, id: &AccountId) {
        let env = self.env();
        let id: xdr::AccountId = id.try_into().unwrap();
        env.host()
            .with_mut_storage(|storage| {
                let k = xdr::LedgerKey::Account(xdr::LedgerKeyAccount { account_id: id });
                storage.del(
                    &k,
                    soroban_env_host::budget::AsBudget::as_budget(env.host()),
                )
            })
            .unwrap();
    }
}

#[cfg(any(test, feature = "testutils"))]
impl Accounts {
    fn default_account_ledger_entry(&self, id: &xdr::AccountId) -> xdr::AccountEntry {
        xdr::AccountEntry {
            account_id: id.clone(),
            balance: 0,
            flags: 0,
            home_domain: xdr::StringM::default(),
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
                    .get(
                        &k,
                        soroban_env_host::budget::AsBudget::as_budget(env.host()),
                    )
                    .ok()
                    .and_then(|v| {
                        if let xdr::LedgerEntryData::Account(_) = v.data {
                            Some(v)
                        } else {
                            None
                        }
                    })
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
                storage.put(
                    &k,
                    &v,
                    soroban_env_host::budget::AsBudget::as_budget(env.host()),
                )
            })
            .unwrap();
    }
}
