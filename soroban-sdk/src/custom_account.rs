//! CustomAccount provides access to helper functions for custom account
//! contracts.
//!
//! The accessor can be created using [Env::custom_account].

use crate::{env::internal::Env as _, unwrap::UnwrapInfallible, Address, Env, IntoVal, Vec};

/// Provides access to functions used for custom account implementation.
///
/// The accessor's methods may only be called within `__check_auth` contract
/// function.
pub struct CustomAccount {
    env: Env,
}

impl CustomAccount {
    pub(crate) fn new(env: &Env) -> CustomAccount {
        CustomAccount { env: env.clone() }
    }

    /// Get the addresses of delegated signers passed to the transaction for
    /// performing authorization checks on behalf of the current custom
    /// account contract.
    ///
    /// This may only be called within `__check_auth` contract function.
    ///
    /// This returns the addresses provided by the user and these are not
    /// sanitized in any way. It is the responsibility of the contract to check
    /// that the addresses are actually valid delegates for the account.
    ///
    /// Typical usage flow is to use `CustomAccount::get_delegated_signers` to
    /// get all the delegated signers passed by the user inside the matching
    /// auth entry, then verify that these delegates actually belong to the
    /// account and are valid for this transaction, and then call
    /// `CustomAccount::delegate_auth` for each of these delegates.
    ///
    /// ### Panics
    ///
    /// If called outside of `__check_auth`.
    pub fn get_delegated_signers(&self) -> Vec<Address> {
        let addresses =
            Env::get_delegated_signers_for_current_auth_check(&self.env).unwrap_infallible();
        addresses.into_val(&self.env)
    }

    /// Delegates the current `__check_auth` authorization to this address.
    ///
    /// This may only be called within `__check_auth` contract function.
    ///
    /// When this is called, the host forwards the current authorization
    /// context provided to `__check_auth` to the `address` and checks if the
    /// `address` has authorized the invocation. This is similar in effect to
    /// calling `require_auth` on the `address`, but unlike `require_auth` this
    /// is not interpreted as a new contract invocation and thus does not
    /// require a separate authorization entry for the `address` in
    /// transaction.
    ///
    /// Use this in conjunction with `CustomAccount::get_delegated_signers`
    /// instead of `require_auth` when implementing modular custom accounts.
    /// These accounts don't have to perform the authentication themselves, but
    /// instead can delegate the authentication logic to a different address
    /// (G- or C-) that performs the actual authentication logic. The delegate
    /// may rely on its own delegates as well, i.e. delegation can be nested
    /// recursively if necessary.
    ///
    /// Typical usage flow is to use `CustomAccount::get_delegated_signers` to
    /// get all the delegated signers passed by the user inside the matching
    /// auth entry, then verify that these delegates actually belong to the
    /// account and are valid for this transaction, and then call
    /// `CustomAccount::delegate_auth` for each of these delegates.
    ///
    /// ### Panics
    ///
    /// If called outside of `__check_auth`, or if this address has not
    /// authorized the invocation.
    pub fn delegate_auth(&self, address: &Address) {
        Env::delegate_account_auth(&self.env, address.to_object()).unwrap_infallible();
    }
}
