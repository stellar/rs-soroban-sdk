use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{AddressObject, Env as _, EnvBase as _},
    BytesN, ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
#[cfg(any(test, feature = "testutils", not(target_family = "wasm")))]
use crate::env::xdr::ScAddress;
use crate::{
    unwrap::{UnwrapInfallible, UnwrapOptimized},
    Vec,
};

/// Address is a universal opaque identifier to use in contracts.
///
/// Address can be used as an input argument (for example, to identify the
/// payment recipient), as a data key (for example, to store the balance), as
/// the authentication & authorization source (for example, to authorize the
/// token transfer) etc.
///
/// See `require_auth` documentation for more details on using Address for
/// authorization.
///
/// Internally, Address may represent a Stellar account or a contract. Contract
/// address may be used to identify the account contracts - special contracts
/// that allow customizing authentication logic and adding custom authorization
/// rules.
///
/// In tests Addresses should be generated via `Address::random()`.
#[derive(Clone)]
pub struct Address {
    env: Env,
    obj: AddressObject,
}

impl Debug for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "Address(..)")?;
        #[cfg(not(target_family = "wasm"))]
        {
            use crate::env::internal::xdr;
            use stellar_strkey::{ed25519, Contract, Strkey};
            let sc_val = ScVal::try_from(self).map_err(|_| core::fmt::Error)?;
            if let ScVal::Address(addr) = sc_val {
                match addr {
                    xdr::ScAddress::Account(account_id) => {
                        let xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(
                            ed25519,
                        ))) = account_id;
                        let strkey = Strkey::PublicKeyEd25519(ed25519::PublicKey(ed25519));
                        write!(f, "AccountId({})", strkey.to_string())?;
                    }
                    xdr::ScAddress::Contract(contract_id) => {
                        let strkey = Strkey::Contract(Contract(contract_id.0));
                        write!(f, "Contract({})", strkey.to_string())?;
                    }
                }
            } else {
                return Err(core::fmt::Error);
            }
        }
        Ok(())
    }
}

impl Eq for Address {}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Address {
    fn cmp(&self, other: &Self) -> Ordering {
        self.env.check_same_env(&other.env);
        let v = self
            .env
            .obj_cmp(self.obj.to_raw(), other.obj.to_raw())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl TryFromVal<Env, AddressObject> for Address {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &AddressObject) -> Result<Self, Self::Error> {
        Ok(unsafe { Address::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, RawVal> for Address {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(AddressObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, Address> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Address) -> Result<Self, Self::Error> {
        Ok(v.to_raw())
    }
}

impl TryFromVal<Env, &Address> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&Address) -> Result<Self, Self::Error> {
        Ok(v.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Address> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Address) -> Result<Self, ConversionError> {
        ScVal::try_from_val(&v.env, &v.obj.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Address> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Address) -> Result<Self, ConversionError> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for Address {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            AddressObject::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Address> for ScAddress {
    type Error = ConversionError;
    fn try_from(v: &Address) -> Result<Self, Self::Error> {
        match ScVal::try_from_val(&v.env, &v.obj.to_raw())? {
            ScVal::Address(a) => Ok(a),
            _ => Err(ConversionError),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Address> for ScAddress {
    type Error = ConversionError;
    fn try_from(v: Address) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScAddress> for Address {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScAddress) -> Result<Self, Self::Error> {
        Ok(AddressObject::try_from_val(
            env,
            &RawVal::try_from_val(env, &ScVal::Address(val.clone()))?,
        )?
        .try_into_val(env)
        .unwrap_infallible())
    }
}

impl Address {
    /// Ensures that this Address has authorized invocation of the current
    /// contract with the provided arguments.
    ///
    /// During the on-chain execution the Soroban host will perform the needed
    /// authentication (verify the signatures) and ensure the replay prevention.
    /// The contracts don't need to perform this tasks.
    ///
    /// The arguments don't have to match the arguments of the contract
    /// invocation. However, it's considered the best practice to have a
    /// well-defined, deterministic and ledger-state-independent mapping between
    /// the contract invocation arguments and `require_auth` arguments. This
    /// will allow the contract callers to easily build the required signature
    /// payloads and prevent potential authorization failures.
    ///
    /// ### Panics
    ///
    /// If the invocation is not authorized.
    pub fn require_auth_for_args(&self, args: Vec<RawVal>) {
        self.env.require_auth_for_args(&self, args);
    }

    /// Ensures that this Address has authorized invocation of the current
    /// contract with all the invocation arguments
    ///
    /// This works exactly in the same fashion as `require_auth_for_args`, but
    /// arguments are automatically inferred from the current contract
    /// invocation.
    ///
    /// This is useful when there is only a single Address that needs to
    /// authorize the contract invocation and there are no dynamic arguments
    /// that don't need authorization.
    ///
    /// ### Panics
    ///
    /// If the invocation is not authorized.    
    pub fn require_auth(&self) {
        self.env.require_auth(&self);
    }

    /// Creates an `Address` corresponding to the provided contract identifier.
    ///
    /// Prefer using the `Address` directly as input or output argument. Only
    /// use this in special cases, for example to get an Address of a freshly
    /// deployed contract.
    ///
    /// TODO: Replace this function in its pub form with a function that accepts
    /// a strkey instead. Dependent on https://github.com/stellar/rs-stellar-strkey/issues/56.
    pub fn from_contract_id(contract_id: &BytesN<32>) -> Self {
        let env = contract_id.env();
        unsafe {
            Self::unchecked_new(
                env.clone(),
                env.contract_id_to_address(contract_id.to_object())
                    .unwrap_optimized(),
            )
        }
    }

    /// Returns 32-byte contract identifier corresponding to this `Address`.
    ///
    /// Returns None if the Address is not a contract.
    pub(crate) fn try_contract_id(&self) -> Option<BytesN<32>> {
        let rv = self.env.address_to_contract_id(self.obj).unwrap_optimized();
        if let Ok(()) = rv.try_into_val(&self.env) {
            None
        } else {
            Some(rv.try_into_val(&self.env).unwrap_optimized())
        }
    }

    /// Returns 32-byte contract identifier corresponding to this `Address`.
    ///
    /// ### Panics
    ///
    /// If Address is not a contract ID.
    pub(crate) fn contract_id(&self) -> BytesN<32> {
        match self.try_contract_id() {
            Some(contract_id) => contract_id,
            None => panic!("Address is not a Contract ID"),
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: AddressObject) -> Self {
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

    pub fn as_object(&self) -> &AddressObject {
        &self.obj
    }

    pub fn to_object(&self) -> AddressObject {
        self.obj
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::env::xdr::Hash;
#[cfg(any(test, feature = "testutils"))]
use crate::testutils::random;
#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl crate::testutils::Address for Address {
    fn random(env: &Env) -> Self {
        let sc_addr = ScVal::Address(ScAddress::Contract(Hash(random())));
        Self::try_from_val(env, &sc_addr).unwrap()
    }

    fn contract_id(&self) -> crate::BytesN<32> {
        self.contract_id()
    }
}
