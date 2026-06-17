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
    ///
    /// ### Examples
    ///
    /// See [`CustomAccount::delegate_auth`] for a complete, runnable example of
    /// a modular custom account that retrieves its delegated signers with this
    /// function, validates them, and delegates authentication to them.
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
    ///
    /// ### Examples
    ///
    /// A "modular" custom account that performs no authentication itself, and
    /// instead delegates it to a set of registered signer addresses. The user
    /// chooses which of the registered signers to authenticate with, by
    /// attaching them to the transaction as delegated signers.
    ///
    /// ```
    /// use soroban_sdk::{
    ///     auth::{Context, CustomAccountInterface},
    ///     contract, contracterror, contractimpl, contracttype,
    ///     crypto::Hash, vec, Address, Env, Vec,
    /// };
    ///
    /// #[contracterror]
    /// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    /// #[repr(u32)]
    /// pub enum Error {
    ///     UnknownDelegate = 1,
    /// }
    ///
    /// #[contracttype]
    /// pub enum DataKey {
    ///     // Marks an address as a signer allowed to authenticate for this
    ///     // account.
    ///     Signer(Address),
    ///     ApprovedContexts,
    /// }
    ///
    /// #[contract]
    /// pub struct ModularAccount;
    ///
    /// #[contractimpl]
    /// impl ModularAccount {
    ///     // Registers the addresses allowed to authenticate for this account.
    ///     pub fn __constructor(env: Env, signers: Vec<Address>) {
    ///         for signer in signers.iter() {
    ///             env.storage().instance().set(&DataKey::Signer(signer), &());
    ///         }
    ///     }
    /// }
    ///
    /// #[contractimpl]
    /// impl CustomAccountInterface for ModularAccount {
    ///     // The account verifies no signature of its own, so it carries no
    ///     // signature to check.
    ///     type Signature = ();
    ///     type Error = Error;
    ///
    ///     fn __check_auth(
    ///         env: Env,
    ///         _signature_payload: Hash<32>,
    ///         _signatures: (),
    ///         _auth_contexts: Vec<Context>,
    ///     ) -> Result<(), Error> {
    ///         // The signers the user attached to the auth entry for this
    ///         // account's authorization.
    ///         let delegates = env.custom_account().get_delegated_signers();
    ///
    ///         // The host does not validate the delegates, so the account must
    ///         // check that each one is actually a registered signer before
    ///         // trusting it.
    ///         for delegate in delegates.iter() {
    ///             if !env.storage().instance().has(&DataKey::Signer(delegate.clone())) {
    ///                 return Err(Error::UnknownDelegate);
    ///             }
    ///             // Forward the current authorization to the delegate. Its own
    ///             // authentication is checked as if it had required auth
    ///             // directly, but without needing a separate auth entry.
    ///             env.custom_account().delegate_auth(&delegate);
    ///         }
    ///         Ok(())
    ///     }
    /// }
    ///
    /// // A delegate account. Stands in for any address (a Stellar account or
    /// // another contract) that knows how to authenticate itself. Here it
    /// // records the context it approved so the test can verify the delegation
    /// // reached it.
    /// #[contract]
    /// pub struct DelegateAccount;
    ///
    /// #[contractimpl]
    /// impl CustomAccountInterface for DelegateAccount {
    ///     type Signature = ();
    ///     type Error = Error;
    ///     fn __check_auth(
    ///         env: Env,
    ///         _signature_payload: Hash<32>,
    ///         _signatures: (),
    ///         auth_contexts: Vec<Context>,
    ///     ) -> Result<(), Error> {
    ///         env.storage()
    ///             .instance()
    ///             .set(&DataKey::ApprovedContexts, &auth_contexts);
    ///         // Returning `Ok(())` approves the delegated authentication;
    ///         // returning an error would reject it.
    ///         Ok(())
    ///     }
    /// }
    ///
    /// // A contract with an operation that requires the account's authorization.
    /// #[contract]
    /// pub struct Protected;
    ///
    /// #[contractimpl]
    /// impl Protected {
    ///     pub fn protected(account: Address) {
    ///         account.require_auth();
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     use soroban_sdk::xdr::{
    ///         InvokeContractArgs, ScAddress, ScVal, SorobanAddressCredentials,
    ///         SorobanAddressCredentialsWithDelegates, SorobanAuthorizationEntry,
    ///         SorobanAuthorizedFunction, SorobanAuthorizedInvocation,
    ///         SorobanCredentials, SorobanDelegateSignature, StringM, VecM,
    ///     };
    ///
    ///     let env = Env::default();
    ///     let delegate = env.register(DelegateAccount, ());
    ///     // Register the modular account with `delegate` as an allowed signer.
    ///     let account = env.register(ModularAccount, (vec![&env, delegate.clone()],));
    ///     let protected = env.register(Protected, ());
    ///
    ///     let account_addr: ScAddress = account.clone().try_into().unwrap();
    ///     let delegate_addr: ScAddress = delegate.clone().try_into().unwrap();
    ///
    ///     // This authorization entry is normally built by the user's
    ///     // wallet/tooling and attached to the transaction. It authorizes
    ///     // `protected` on behalf of the account, and attaches `delegate` as a
    ///     // delegated signer. Delegates must be sorted by address with no
    ///     // duplicates.
    ///     env.set_auths(&[SorobanAuthorizationEntry {
    ///         credentials: SorobanCredentials::AddressWithDelegates(
    ///             SorobanAddressCredentialsWithDelegates {
    ///                 address_credentials: SorobanAddressCredentials {
    ///                     address: account_addr.clone(),
    ///                     nonce: 1,
    ///                     signature_expiration_ledger: 100,
    ///                     // The account verifies no signature of its own.
    ///                     signature: ScVal::Void,
    ///                 },
    ///                 delegates: std::vec![SorobanDelegateSignature {
    ///                     address: delegate_addr,
    ///                     signature: ScVal::Void,
    ///                     nested_delegates: VecM::default(),
    ///                 }]
    ///                 .try_into()
    ///                 .unwrap(),
    ///             },
    ///         ),
    ///         root_invocation: SorobanAuthorizedInvocation {
    ///             function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
    ///                 contract_address: protected.clone().try_into().unwrap(),
    ///                 function_name: StringM::try_from("protected").unwrap().into(),
    ///                 args: std::vec![ScVal::Address(account_addr)].try_into().unwrap(),
    ///             }),
    ///             sub_invocations: VecM::default(),
    ///         },
    ///     }]);
    ///
    ///     // The call succeeds: the account delegates its authentication to
    ///     // `delegate`, which approves it.
    ///     ProtectedClient::new(&env, &protected).protected(&account);
    ///
    ///     // The account authorized the `protected` call. Delegating to
    ///     // `delegate` is not recorded as a separate authorization.
    ///     use soroban_sdk::{
    ///         auth::ContractContext,
    ///         testutils::{AuthorizedFunction, AuthorizedInvocation},
    ///         IntoVal, Symbol,
    ///     };
    ///     assert_eq!(
    ///         env.auths(),
    ///         std::vec![(
    ///             account.clone(),
    ///             AuthorizedInvocation {
    ///                 function: AuthorizedFunction::Contract((
    ///                     protected.clone(),
    ///                     Symbol::new(&env, "protected"),
    ///                     (account.clone(),).into_val(&env),
    ///                 )),
    ///                 sub_invocations: std::vec![],
    ///             }
    ///         )]
    ///     );
    ///
    ///     // The delegation actually reached `delegate`, which approved the
    ///     // same invocation that was authorized above.
    ///     let approved: Vec<Context> = env.as_contract(&delegate, || {
    ///         env.storage().instance().get(&DataKey::ApprovedContexts).unwrap()
    ///     });
    ///     assert_eq!(
    ///         approved,
    ///         vec![
    ///             &env,
    ///             Context::Contract(ContractContext {
    ///                 contract: protected.clone(),
    ///                 fn_name: Symbol::new(&env, "protected"),
    ///                 args: (account.clone(),).into_val(&env),
    ///             }),
    ///         ],
    ///     );
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn delegate_auth(&self, address: &Address) {
        Env::delegate_account_auth(&self.env, address.to_object()).unwrap_infallible();
    }
}
