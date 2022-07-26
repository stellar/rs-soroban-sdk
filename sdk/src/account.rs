use core::{borrow::Borrow, cmp::Ordering, fmt::Debug};

use crate::{
    env::internal::{Env as _, RawVal, RawValConvertible, TagObject, TaggedVal},
    env::EnvObj,
    Binary, ConversionError, Env, EnvType, EnvVal, FixedBinary,
};

#[derive(Clone)]
pub struct Account(FixedBinary<32>);

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
        let env = self.env();
        let v = env.obj_cmp(self.0.to_raw(), other.0.to_raw());
        let i = i32::try_from(v).unwrap();
        i.cmp(&0)
    }
}

impl Borrow<Binary> for Account {
    fn borrow(&self) -> &Binary {
        self.0.borrow()
    }
}

impl Borrow<Binary> for &Account {
    fn borrow(&self) -> &Binary {
        self.0.borrow()
    }
}

impl Borrow<Binary> for &mut Account {
    fn borrow(&self) -> &Binary {
        self.0.borrow()
    }
}

impl Borrow<FixedBinary<32>> for Account {
    fn borrow(&self) -> &FixedBinary<32> {
        self.0.borrow()
    }
}

impl Borrow<FixedBinary<32>> for &Account {
    fn borrow(&self) -> &FixedBinary<32> {
        self.0.borrow()
    }
}

impl Borrow<FixedBinary<32>> for &mut Account {
    fn borrow(&self) -> &FixedBinary<32> {
        self.0.borrow()
    }
}

impl AsRef<Binary> for Account {
    fn as_ref(&self) -> &Binary {
        self.0.as_ref()
    }
}

impl AsRef<FixedBinary<32>> for Account {
    fn as_ref(&self) -> &FixedBinary<32> {
        &self.0
    }
}

impl From<EnvType<[u8; 32]>> for Account {
    fn from(ev: EnvType<[u8; 32]>) -> Self {
        Account(<_ as From<EnvType<[u8; 32]>>>::from(ev))
    }
}

impl TryFrom<EnvVal> for Account {
    type Error = ConversionError;

    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.clone().try_into()?;
        obj.try_into()
    }
}

impl TryFrom<EnvObj> for Account {
    type Error = ConversionError;

    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        Ok(Account(obj.try_into()?))
    }
}

impl From<Account> for RawVal {
    fn from(a: Account) -> Self {
        a.0.into()
    }
}

impl From<Account> for EnvVal {
    fn from(a: Account) -> Self {
        a.0.into()
    }
}

impl From<Account> for EnvObj {
    fn from(a: Account) -> Self {
        a.0.into()
    }
}

impl Account {
    pub(crate) fn env(&self) -> &Env {
        self.0.env()
    }

    pub(crate) fn as_raw(&self) -> &RawVal {
        self.0.as_raw()
    }

    pub(crate) fn as_tagged(&self) -> &TaggedVal<TagObject> {
        self.0.as_tagged()
    }

    pub(crate) fn to_raw(&self) -> RawVal {
        self.0.to_raw()
    }

    pub(crate) fn to_tagged(&self) -> TaggedVal<TagObject> {
        self.0.to_tagged()
    }

    pub fn from_public_key(public_key: &FixedBinary<32>) -> Result<Account, ()> {
        let acc = Account(public_key.clone());
        // TODO: Fail when account doesn't exist. In the meantime cause a trap
        // at this point by trying to get some information about the account
        // which will trap if the account doesn't exist.
        _ = acc.low_threshold();
        Ok(acc)
    }

    pub fn low_threshold(&self) -> u32 {
        let env = self.env();
        let val = env.account_get_low_threshold(self.to_tagged());
        unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) }
    }

    pub fn medium_threshold(&self) -> u32 {
        let env = self.env();
        let val = env.account_get_medium_threshold(self.to_tagged());
        unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) }
    }

    pub fn high_threshold(&self) -> u32 {
        let env = self.env();
        let val = env.account_get_high_threshold(self.to_tagged());
        unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) }
    }

    pub fn signer_weight(&self, signer: &FixedBinary<32>) -> u32 {
        let env = self.env();
        let val = env.account_get_signer_weight(self.to_tagged(), signer.to_tagged());
        unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) }
    }
}
