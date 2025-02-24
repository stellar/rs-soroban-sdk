use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{AddressObject, Env as _, MuxedAddressObject, Tag},
    ConversionError, Env, TryFromVal, TryIntoVal, Val,
};
use crate::{unwrap::UnwrapInfallible, Address};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::{ScAddress, ScVal};

#[derive(Clone)]
enum AddressObjectWrapper {
    Address(AddressObject),
    MuxedAddress(MuxedAddressObject),
}

#[derive(Clone)]
pub struct MuxedAddress {
    env: Env,
    obj: AddressObjectWrapper,
}

impl Debug for MuxedAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MuxedAddress(..)")?;
        Ok(())
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

impl From<Address> for MuxedAddress {
    fn from(address: Address) -> Self {
        address
            .as_object()
            .try_into_val(address.env())
            .unwrap_infallible()
    }
}

impl MuxedAddress {
    pub fn to_address(&self) -> Address {
        match &self.obj {
            AddressObjectWrapper::Address(address_object) => {
                Address::try_from_val(&self.env, address_object).unwrap_infallible()
            }
            AddressObjectWrapper::MuxedAddress(muxed_address_object) => {
                self.env.muxed_address_to_address(*muxed_address_object)
            }
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
    fn from_account_id(env: &Env, account_key: &[u8; 32], id: u64) -> crate::MuxedAddress {
        let sc_val = ScVal::Address(crate::env::internal::xdr::ScAddress::MuxedAccount(
            crate::env::internal::xdr::MuxedAccount::MuxedEd25519(
                crate::env::internal::xdr::MuxedAccountMed25519 {
                    id,
                    ed25519: crate::env::internal::xdr::Uint256(account_key.clone()),
                },
            ),
        ));
        sc_val.try_into_val(env).unwrap()
    }
}
