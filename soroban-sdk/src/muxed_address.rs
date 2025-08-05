use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{AddressObject, Env as _, MuxedAddressObject, Tag},
    ConversionError, Env, TryFromVal, TryIntoVal, Val,
};
use crate::{env::internal, unwrap::UnwrapInfallible, Address};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::{ScAddress, ScVal};

#[derive(Clone)]
enum AddressObjectWrapper {
    Address(AddressObject),
    MuxedAddress(MuxedAddressObject),
}

/// MuxedAddress is a union type that represents either the regular `Address`,
/// or a 'multiplexed' address that consists of a regular address and a u64 id
/// and can be used for representing the 'virtual' accounts that allows for
/// managing multiple balances off-chain with only a single on-chain balance
/// entry. The address part can be used as a regular `Address`, and the id
/// part should be used only in the events for the off-chain processing.
///
/// This type is only necessary in a few special cases, such as token transfers
/// that support non-custodial accounts (e.g. for the exchange support). Prefer
/// using the regular `Address` type unless multiplexing support is necessary.
///
/// This type is compatible with `Address` at the contract interface level, i.e.
/// if a contract accepts `MuxedAddress` as an input, then its callers may still
/// pass `Address` into the call successfully. This means that if a
/// contract has upgraded its interface to switch from `Address` argument to
/// `MuxedAddress` argument, it won't break any of its existing clients.
///
/// Currently only the regular Stellar accounts can be multiplexed, i.e.
/// multiplexed contract addresses don't exist.
///
/// Note, that multiplexed addresses can not be used directly as a storage key.
/// This is a precaution to prevent accidental unexpected fragmentation of
/// the key space (like creating an arbitrary number of balances for the same
/// actual `Address`).
#[derive(Clone)]
pub struct MuxedAddress {
    env: Env,
    obj: AddressObjectWrapper,
}

impl Debug for MuxedAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        match &self.obj {
            AddressObjectWrapper::Address(_) => write!(f, "Address(..)"),
            AddressObjectWrapper::MuxedAddress(_) => write!(f, "MuxedAddress(..)"),
        }
        #[cfg(not(target_family = "wasm"))]
        {
            use crate::env::internal::xdr;
            use stellar_strkey::Strkey;
            match &self.obj {
                AddressObjectWrapper::Address(address_object) => {
                    Address::try_from_val(self.env(), address_object)
                        .map_err(|_| core::fmt::Error)?
                        .fmt(f)
                }
                AddressObjectWrapper::MuxedAddress(muxed_address_object) => {
                    let sc_val = ScVal::try_from_val(self.env(), &muxed_address_object.to_val())
                        .map_err(|_| core::fmt::Error)?;
                    if let ScVal::Address(addr) = sc_val {
                        match addr {
                            xdr::ScAddress::MuxedAccount(muxed_account) => {
                                let strkey = Strkey::MuxedAccountEd25519(
                                    stellar_strkey::ed25519::MuxedAccount {
                                        ed25519: muxed_account.ed25519.0,
                                        id: muxed_account.id,
                                    },
                                );
                                write!(f, "MuxedAccount({})", strkey.to_string())
                            }
                            _ => Err(core::fmt::Error),
                        }
                    } else {
                        Err(core::fmt::Error)
                    }
                }
            }
        }
    }
}

impl Eq for MuxedAddress {}

impl PartialEq for MuxedAddress {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for MuxedAddress {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for MuxedAddress {
    fn cmp(&self, other: &Self) -> Ordering {
        let v = self
            .env
            .obj_cmp(self.to_val(), other.to_val())
            .unwrap_infallible();
        v.cmp(&0)
    }
}

impl TryFromVal<Env, MuxedAddressObject> for MuxedAddress {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &MuxedAddressObject) -> Result<Self, Self::Error> {
        Ok(unsafe { MuxedAddress::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, AddressObject> for MuxedAddress {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &AddressObject) -> Result<Self, Self::Error> {
        Ok(unsafe { MuxedAddress::unchecked_new_from_address(env.clone(), *val) })
    }
}

impl TryFromVal<Env, Val> for MuxedAddress {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        if val.get_tag() == Tag::AddressObject {
            Ok(AddressObject::try_from_val(env, val)?
                .try_into_val(env)
                .unwrap_infallible())
        } else {
            Ok(MuxedAddressObject::try_from_val(env, val)?
                .try_into_val(env)
                .unwrap_infallible())
        }
    }
}

impl TryFromVal<Env, MuxedAddress> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &MuxedAddress) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl TryFromVal<Env, &MuxedAddress> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&MuxedAddress) -> Result<Self, Self::Error> {
        Ok(v.to_val())
    }
}

impl From<&MuxedAddress> for MuxedAddress {
    fn from(address: &MuxedAddress) -> Self {
        address.clone()
    }
}

impl From<Address> for MuxedAddress {
    fn from(address: Address) -> Self {
        (&address).into()
    }
}

impl From<&Address> for MuxedAddress {
    fn from(address: &Address) -> Self {
        address
            .as_object()
            .try_into_val(address.env())
            .unwrap_infallible()
    }
}

impl MuxedAddress {
    /// Returns the `Address` part of this multiplexed address.
    ///
    /// The address part is necessary to perform most of the operations, such
    /// as authorization or storage.
    pub fn address(&self) -> Address {
        match &self.obj {
            AddressObjectWrapper::Address(address_object) => {
                Address::try_from_val(&self.env, address_object).unwrap_infallible()
            }
            AddressObjectWrapper::MuxedAddress(muxed_address_object) => Address::try_from_val(
                &self.env,
                &internal::Env::get_address_from_muxed_address(&self.env, *muxed_address_object)
                    .unwrap_infallible(),
            )
            .unwrap_infallible(),
        }
    }

    /// Returns the multiplexing identifier part of this multiplexed address,
    /// if any.
    ///
    /// Returns `None` for the regular (non-multiplexed) addresses.
    ///
    /// This identifier should normally be used in the events in order to allow
    /// for tracking the virtual balances associated with this address off-chain.
    pub fn id(&self) -> Option<u64> {
        match &self.obj {
            AddressObjectWrapper::Address(_) => None,
            AddressObjectWrapper::MuxedAddress(muxed_address_object) => Some(
                u64::try_from_val(
                    &self.env,
                    &internal::Env::get_id_from_muxed_address(&self.env, *muxed_address_object)
                        .unwrap_infallible(),
                )
                .unwrap(),
            ),
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn unchecked_new_from_address(env: Env, obj: AddressObject) -> Self {
        Self {
            env,
            obj: AddressObjectWrapper::Address(obj),
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, obj: MuxedAddressObject) -> Self {
        Self {
            env,
            obj: AddressObjectWrapper::MuxedAddress(obj),
        }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_val(&self) -> &Val {
        match &self.obj {
            AddressObjectWrapper::Address(o) => o.as_val(),
            AddressObjectWrapper::MuxedAddress(o) => o.as_val(),
        }
    }

    pub fn to_val(&self) -> Val {
        match self.obj {
            AddressObjectWrapper::Address(o) => o.to_val(),
            AddressObjectWrapper::MuxedAddress(o) => o.to_val(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for MuxedAddress {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        let v = Val::try_from_val(env, val)?;
        match val {
            ScVal::Address(sc_address) => match sc_address {
                ScAddress::Account(_) | ScAddress::Contract(_) => {
                    Ok(AddressObject::try_from_val(env, &v)?
                        .try_into_val(env)
                        .unwrap_infallible())
                }
                ScAddress::MuxedAccount(_) => Ok(MuxedAddressObject::try_from_val(env, &v)?
                    .try_into_val(env)
                    .unwrap_infallible()),
                ScAddress::ClaimableBalance(_) | ScAddress::LiquidityPool(_) => {
                    panic!("unsupported ScAddress type")
                }
            },
            _ => panic!("incorrect scval type"),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScAddress> for MuxedAddress {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScAddress) -> Result<Self, Self::Error> {
        ScVal::Address(val.clone()).try_into_val(env)
    }
}

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl crate::testutils::MuxedAddress for MuxedAddress {
    fn generate(env: &Env) -> crate::MuxedAddress {
        let sc_val = ScVal::Address(crate::env::internal::xdr::ScAddress::MuxedAccount(
            crate::env::internal::xdr::MuxedEd25519Account {
                ed25519: crate::env::internal::xdr::Uint256(
                    env.with_generator(|mut g| g.address()),
                ),
                id: env.with_generator(|mut g| g.mux_id()),
            },
        ));
        sc_val.try_into_val(env).unwrap()
    }

    fn new<T: Into<MuxedAddress>>(address: T, id: u64) -> crate::MuxedAddress {
        let address: MuxedAddress = address.into();
        let sc_val = ScVal::try_from_val(&address.env, address.as_val()).unwrap();
        let account_id = match sc_val {
            ScVal::Address(address) => match address {
                ScAddress::MuxedAccount(muxed_account) => muxed_account.ed25519,
                ScAddress::Account(crate::env::internal::xdr::AccountId(
                    crate::env::internal::xdr::PublicKey::PublicKeyTypeEd25519(account_id),
                )) => account_id,
                ScAddress::Contract(_) => panic!("contract addresses can not be multiplexed"),
                ScAddress::ClaimableBalance(_) | ScAddress::LiquidityPool(_) => unreachable!(),
            },
            _ => unreachable!(),
        };
        let result_sc_val = ScVal::Address(ScAddress::MuxedAccount(
            crate::env::internal::xdr::MuxedEd25519Account {
                id,
                ed25519: account_id,
            },
        ));
        result_sc_val.try_into_val(&address.env).unwrap()
    }
}
