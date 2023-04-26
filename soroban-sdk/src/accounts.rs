//! Accounts contains types for accessing accounts in the current ledger.
//!
//! See [`Accounts`][Accounts] for examples.

use crate::Env;

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
}

#[cfg(any(test, feature = "testutils"))]
use crate::{testutils, xdr, Address};

#[cfg(any(test, feature = "testutils"))]
extern crate alloc;
#[cfg(any(test, feature = "testutils"))]
use alloc::rc::Rc;

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl testutils::Accounts for Accounts {
    fn create(&self, id: &Address) {
        let env = self.env();
        let Some(id) = id.account_id() else {
            panic!("account can only be created with account address");
        };
        let id = xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(
            id.into(),
        )));
        env.host()
            .with_mut_storage(|storage| {
                let k = Rc::new(xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: id.clone(),
                }));

                if !storage.has(
                    &k,
                    soroban_env_host::budget::AsBudget::as_budget(env.host()),
                )? {
                    let v = Rc::new(xdr::LedgerEntry {
                        data: xdr::LedgerEntryData::Account(self.default_account_ledger_entry(&id)),
                        last_modified_ledger_seq: 0,
                        ext: xdr::LedgerEntryExt::V0,
                    });
                    storage.put(
                        &k,
                        &v,
                        soroban_env_host::budget::AsBudget::as_budget(env.host()),
                    )?
                }
                Ok(())
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
            home_domain: xdr::String32(xdr::StringM::default()),
            inflation_dest: None,
            num_sub_entries: 0,
            seq_num: xdr::SequenceNumber(0),
            thresholds: xdr::Thresholds([1; 4]),
            signers: xdr::VecM::default(),
            ext: xdr::AccountEntryExt::V0,
        }
    }
}
