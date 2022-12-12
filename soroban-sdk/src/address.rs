use core::cmp::Ordering;

use crate::env::internal::xdr::ScObjectType;
use crate::{env::EnvObj, ConversionError, Env, IntoVal, Object, RawVal, TryFromVal, TryIntoVal};

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;

#[cfg(all(feature = "testutils", not(target_family = "wasm")))]
use crate::BytesN;

#[derive(Clone)]
pub struct Address(EnvObj);

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
        self.0.cmp(&other.0)
    }
}

impl TryFromVal<Env, Object> for Address {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: Object) -> Result<Self, Self::Error> {
        if val.is_obj_type(ScObjectType::Address) {
            Ok(Address(val.in_env(env)))
        } else {
            Err(ConversionError {})
        }
    }
}

impl TryIntoVal<Env, Address> for Object {
    type Error = <Address as TryFromVal<Env, Object>>::Error;

    fn try_into_val(self, env: &Env) -> Result<Address, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, self)
    }
}

impl TryFromVal<Env, RawVal> for Address {
    type Error = <Address as TryFromVal<Env, Object>>::Error;

    fn try_from_val(env: &Env, val: RawVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(env, val.try_into()?)
    }
}

impl TryIntoVal<Env, Address> for RawVal {
    type Error = <Address as TryFromVal<Env, Object>>::Error;

    fn try_into_val(self, env: &Env) -> Result<Address, Self::Error> {
        <_ as TryFromVal<_, RawVal>>::try_from_val(env, self)
    }
}

impl IntoVal<Env, Object> for Address {
    fn into_val(self, _env: &Env) -> Object {
        self.to_object()
    }
}

impl IntoVal<Env, Object> for &Address {
    fn into_val(self, _env: &Env) -> Object {
        self.to_object()
    }
}

impl IntoVal<Env, RawVal> for Address {
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
    }
}

impl IntoVal<Env, RawVal> for &Address {
    fn into_val(self, _env: &Env) -> RawVal {
        self.to_raw()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Address> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Address) -> Result<Self, Self::Error> {
        ScVal::try_from_val(&v.0.env, v.0.val.to_raw())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Address> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Address) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for Address {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: ScVal) -> Result<Self, Self::Error> {
        <_ as TryFromVal<_, Object>>::try_from_val(
            env,
            val.try_into_val(env).map_err(|_| ConversionError)?,
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryIntoVal<Env, Address> for ScVal {
    type Error = ConversionError;
    fn try_into_val(self, env: &Env) -> Result<Address, Self::Error> {
        Address::try_from_val(env, self)
    }
}

impl Address {
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

    #[cfg(all(feature = "testutils", not(target_family = "wasm")))]
    pub fn from_contract_id(env: &Env, contract_id: &BytesN<32>) -> Self {
        use crate::env::xdr::{Hash, ScAddress, ScObject};

        let sc_addr = ScVal::Object(Some(ScObject::Address(ScAddress::Contract(Hash(
            contract_id.into_val(env),
        )))));
        Self::try_from_val(env, sc_addr).unwrap()
    }
}
