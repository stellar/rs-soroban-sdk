use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{AddressObject, Env as _},
    ConversionError, Env, String, TryFromVal, TryIntoVal, Val,
};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;
#[cfg(any(test, feature = "testutils", not(target_family = "wasm")))]
use crate::env::xdr::ScAddress;
use crate::{unwrap::UnwrapInfallible, Bytes, Vec};

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
/// In tests Addresses should be generated via `Address::generate()`.
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

impl TryFromVal<Env, AddressObject> for Address {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &AddressObject) -> Result<Self, Self::Error> {
        Ok(unsafe { Address::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, Val> for Address {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(AddressObject::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, Address> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Address) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl TryFromVal<Env, &Address> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&Address) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<&Address> for ScVal {
    fn from(v: &Address) -> Self {
        // This conversion occurs only in test utilities, and theoretically all
        // values should convert to an ScVal because the Env won't let the host
        // type to exist otherwise, unwrapping. Even if there are edge cases
        // that don't, this is a trade off for a better test developer
        // experience.
        ScVal::try_from_val(&v.env, &v.obj.to_val()).unwrap()
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<Address> for ScVal {
    fn from(v: Address) -> Self {
        (&v).into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for Address {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            AddressObject::try_from_val(env, &Val::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<&Address> for ScAddress {
    fn from(v: &Address) -> Self {
        match ScVal::try_from_val(&v.env, &v.obj.to_val()).unwrap() {
            ScVal::Address(a) => a,
            _ => panic!("expected ScVal::Address"),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<Address> for ScAddress {
    fn from(v: Address) -> Self {
        (&v).into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScAddress> for Address {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScAddress) -> Result<Self, Self::Error> {
        Ok(AddressObject::try_from_val(
            env,
            &Val::try_from_val(env, &ScVal::Address(val.clone()))?,
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
    pub fn require_auth_for_args(&self, args: Vec<Val>) {
        self.env.require_auth_for_args(self, args);
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
        self.env.require_auth(self);
    }

    /// Creates an `Address` corresponding to the provided Stellar strkey.
    ///
    /// The only supported strkey types are account keys (`G...`) and contract keys (`C...`). Any
    /// other valid or invalid strkey will cause this to panic.
    ///
    /// Prefer using the `Address` directly as input or output argument. Only
    /// use this in special cases when addresses need to be shared between
    /// different environments (e.g. different chains).
    pub fn from_str(env: &Env, strkey: &str) -> Address {
        Address::from_string(&String::from_str(env, strkey))
    }

    /// Creates an `Address` corresponding to the provided Stellar strkey.
    ///
    /// The only supported strkey types are account keys (`G...`) and contract keys (`C...`). Any
    /// other valid or invalid strkey will cause this to panic.
    ///
    /// Prefer using the `Address` directly as input or output argument. Only
    /// use this in special cases when addresses need to be shared between
    /// different environments (e.g. different chains).
    pub fn from_string(strkey: &String) -> Self {
        let env = strkey.env();
        unsafe {
            Self::unchecked_new(
                env.clone(),
                env.strkey_to_address(strkey.to_object().to_val())
                    .unwrap_infallible(),
            )
        }
    }

    /// Creates an `Address` corresponding to the provided Stellar strkey bytes.
    ///
    /// This behaves exactly in the same fashion as `from_strkey`, i.e. the bytes should contain
    /// exactly the same contents as `String` would (i.e. base-32 ASCII string).
    ///
    /// The only supported strkey types are account keys (`G...`) and contract keys (`C...`). Any
    /// other valid or invalid strkey will cause this to panic.
    ///
    /// Prefer using the `Address` directly as input or output argument. Only
    /// use this in special cases when addresses need to be shared between
    /// different environments (e.g. different chains).
    pub fn from_string_bytes(strkey: &Bytes) -> Self {
        let env = strkey.env();
        unsafe {
            Self::unchecked_new(
                env.clone(),
                env.strkey_to_address(strkey.to_object().to_val())
                    .unwrap_infallible(),
            )
        }
    }

    pub fn to_string(&self) -> String {
        String::try_from_val(
            &self.env,
            &self.env.address_to_strkey(self.obj).unwrap_infallible(),
        )
        .unwrap_optimized()
    }

    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: AddressObject) -> Self {
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

    pub fn as_object(&self) -> &AddressObject {
        &self.obj
    }

    pub fn to_object(&self) -> AddressObject {
        self.obj
    }
}

#[cfg(any(not(target_family = "wasm"), test, feature = "testutils"))]
use crate::env::xdr::Hash;
use crate::unwrap::UnwrapOptimized;

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl crate::testutils::Address for Address {
    fn generate(env: &Env) -> Self {
        Self::try_from_val(
            env,
            &ScAddress::Contract(Hash(env.with_generator(|mut g| g.address()))),
        )
        .unwrap()
    }
}

#[cfg(not(target_family = "wasm"))]
impl Address {
    pub(crate) fn contract_id(&self) -> Hash {
        let sc_address: ScAddress = self.try_into().unwrap();
        if let ScAddress::Contract(c) = sc_address {
            c
        } else {
            panic!("address is not a contract {:?}", self);
        }
    }

    pub(crate) fn from_contract_id(env: &Env, contract_id: [u8; 32]) -> Self {
        Self::try_from_val(env, &ScAddress::Contract(Hash(contract_id))).unwrap()
    }
}
