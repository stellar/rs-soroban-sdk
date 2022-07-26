use crate::{
    env::internal::{Env as _, RawVal, RawValConvertible, TagObject, TaggedVal},
    Binary, Env, FixedBinary,
};

pub struct Account(Binary);

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

    pub fn from_public_key(public_key: &Binary) -> Result<Account, ()> {
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

    pub fn signer_weight(&self, signer: FixedBinary<32>) -> u32 {
        let env = self.env();
        let val = env.account_get_signer_weight(self.to_tagged(), signer.to_tagged());
        unsafe { <u32 as RawValConvertible>::unchecked_from_val(val) }
    }
}
