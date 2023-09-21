use core::convert::Infallible;

#[cfg(target_family = "wasm")]
pub mod internal {
    use core::convert::Infallible;

    pub use soroban_env_guest::*;
    pub type EnvImpl = Guest;
    pub type MaybeEnvImpl = Guest;

    // In the Guest case, Env::Error is already Infallible so there is no work
    // to do to "reject an error": if an error occurs in the environment, the
    // host will trap our VM and we'll never get here at all.
    pub(crate) fn reject_err<T>(_env: &Guest, r: Result<T, Infallible>) -> Result<T, Infallible> {
        r
    }
}

#[cfg(not(target_family = "wasm"))]
pub mod internal {
    use core::convert::Infallible;

    pub use soroban_env_host::*;
    pub type EnvImpl = Host;
    pub type MaybeEnvImpl = Option<Host>;

    // When we have `feature="testutils"` (or are in cfg(test)) we enable feature
    // `soroban-env-{common,host}/testutils` which in turn adds the helper method
    // `Env::escalate_error_to_panic` to the Env trait.
    //
    // When this is available we want to use it, because it works in concert
    // with a _different_ part of the host that's also `testutils`-gated: the
    // mechanism for emulating the WASM VM error-handling semantics with native
    // contracts. In particular when a WASM contract calls a host function that
    // fails with some error E, the host traps the VM (not returning to it at
    // all) and propagates E to the caller of the contract. This is simulated in
    // the native case by returning a (nontrivial) error E to us here, which we
    // then "reject" back to the host, which stores E in a temporary cell inside
    // any `TestContract` frame in progress and then _panics_, unwinding back to
    // a panic-catcher it installed when invoking the `TestContract` frame, and
    // then extracting E from the frame and returning it to its caller. This
    // simulates the "crash, but catching the error" behaviour of the WASM case.
    // This only works if we panic via `escalate_error_to_panic`.
    //
    // (The reason we don't just panic_any() here and let the panic-catcher do a
    // type-based catch is that there might _be_ no panic-catcher around us, and
    // we want to print out a nice error message in that case too, which
    // panic_any() does not do us the favor of producing. This is all very
    // subtle. See also soroban_env_host::Host::escalate_error_to_panic.)
    #[cfg(any(test, feature = "testutils"))]
    pub(crate) fn reject_err<T>(env: &Host, r: Result<T, HostError>) -> Result<T, Infallible> {
        r.map_err(|e| env.escalate_error_to_panic(e))
    }

    // When we're _not_ in a cfg enabling `soroban-env-{common,host}/testutils`,
    // there is no `Env::escalate_error_to_panic` to call, so we just panic
    // here. But this is ok because in that case there is also no multi-contract
    // calling machinery set up, nor probably any panic-catcher installed that
    // we need to hide error values for the benefit of. Any panic in this case
    // is probably going to unwind completely anyways. No special case needed.
    #[cfg(not(any(test, feature = "testutils")))]
    pub(crate) fn reject_err<T>(_env: &Host, r: Result<T, HostError>) -> Result<T, Infallible> {
        r.map_err(|e| panic!("{:?}", e))
    }

    #[doc(hidden)]
    impl<F, T> Convert<F, T> for super::Env
    where
        EnvImpl: Convert<F, T>,
    {
        type Error = <EnvImpl as Convert<F, T>>::Error;
        fn convert(&self, f: F) -> Result<T, Self::Error> {
            self.env_impl.convert(f)
        }
    }
}

// Testutils from the environment are pub here, and then pub re-exported out of
// the SDK in the crate::testutils mod.
#[cfg(any(test, feature = "testutils"))]
pub mod testutils {
    pub use super::internal::budget::Budget;
    pub use super::internal::LedgerInfo;
}

pub use internal::meta;
pub use internal::xdr;
pub use internal::Compare;
pub use internal::ConversionError;
pub use internal::EnvBase;
pub use internal::Error;
pub use internal::MapObject;
pub use internal::SymbolStr;
pub use internal::TryFromVal;
pub use internal::TryIntoVal;
pub use internal::Val;
pub use internal::VecObject;

pub trait IntoVal<E: internal::Env, T> {
    fn into_val(&self, e: &E) -> T;
}

pub trait FromVal<E: internal::Env, T> {
    fn from_val(e: &E, v: &T) -> Self;
}

impl<E: internal::Env, T, U> FromVal<E, T> for U
where
    U: TryFromVal<E, T>,
{
    fn from_val(e: &E, v: &T) -> Self {
        U::try_from_val(e, v).unwrap_optimized()
    }
}

impl<E: internal::Env, T, U> IntoVal<E, T> for U
where
    T: FromVal<E, Self>,
{
    fn into_val(&self, e: &E) -> T {
        T::from_val(e, self)
    }
}

use crate::auth::InvokerContractAuthEntry;
use crate::unwrap::UnwrapInfallible;
use crate::unwrap::UnwrapOptimized;
use crate::{
    crypto::Crypto, deploy::Deployer, events::Events, ledger::Ledger, logs::Logs, prng::Prng,
    storage::Storage, Address, Vec,
};
use internal::{
    AddressObject, Bool, BytesObject, DurationObject, I128Object, I256Object, I256Val, I64Object,
    StorageType, StringObject, Symbol, SymbolObject, TimepointObject, U128Object, U256Object,
    U256Val, U32Val, U64Object, U64Val, Void,
};

#[doc(hidden)]
#[derive(Clone)]
pub struct MaybeEnv {
    maybe_env_impl: internal::MaybeEnvImpl,
    #[cfg(any(test, feature = "testutils"))]
    snapshot: Option<Rc<LedgerSnapshot>>,
}

#[cfg(target_family = "wasm")]
impl TryFrom<MaybeEnv> for Env {
    type Error = Infallible;

    fn try_from(_value: MaybeEnv) -> Result<Self, Self::Error> {
        Ok(Env {
            env_impl: internal::EnvImpl {},
            #[cfg(any(test, feature = "testutils"))]
            snapshot: value.snapshot,
        })
    }
}

impl Default for MaybeEnv {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(target_family = "wasm")]
impl MaybeEnv {
    // separate function to be const
    pub const fn none() -> Self {
        Self {
            maybe_env_impl: internal::EnvImpl {},
            #[cfg(any(test, feature = "testutils"))]
            snapshot: None,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl MaybeEnv {
    // separate function to be const
    pub const fn none() -> Self {
        Self {
            maybe_env_impl: None,
            #[cfg(any(test, feature = "testutils"))]
            snapshot: None,
        }
    }
}

#[cfg(target_family = "wasm")]
impl From<Env> for MaybeEnv {
    fn from(value: Env) -> Self {
        MaybeEnv {
            maybe_env_impl: value.env_impl,
            #[cfg(any(test, feature = "testutils"))]
            snapshot: value.snapshot,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<MaybeEnv> for Env {
    type Error = ConversionError;

    fn try_from(value: MaybeEnv) -> Result<Self, Self::Error> {
        if let Some(env_impl) = value.maybe_env_impl {
            Ok(Env {
                env_impl,
                #[cfg(any(test, feature = "testutils"))]
                snapshot: value.snapshot,
            })
        } else {
            Err(ConversionError)
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<Env> for MaybeEnv {
    fn from(value: Env) -> Self {
        MaybeEnv {
            maybe_env_impl: Some(value.env_impl),
            #[cfg(any(test, feature = "testutils"))]
            snapshot: value.snapshot,
        }
    }
}

/// The [Env] type provides access to the environment the contract is executing
/// within.
///
/// The [Env] provides access to information about the currently executing
/// contract, who invoked it, contract data, functions for signing, hashing,
/// etc.
///
/// Most types require access to an [Env] to be constructed or converted.
#[derive(Clone)]
pub struct Env {
    env_impl: internal::EnvImpl,
    #[cfg(any(test, feature = "testutils"))]
    snapshot: Option<Rc<LedgerSnapshot>>,
}

impl Default for Env {
    #[cfg(not(any(test, feature = "testutils")))]
    fn default() -> Self {
        Self {
            env_impl: Default::default(),
        }
    }

    #[cfg(any(test, feature = "testutils"))]
    fn default() -> Self {
        Self::default_with_testutils()
    }
}

impl Env {
    /// Panic with the given error.
    ///
    /// Equivalent to `panic!`, but with an error value instead of a string.
    #[doc(hidden)]
    #[inline(always)]
    pub fn panic_with_error(&self, error: impl Into<internal::Error>) -> ! {
        _ = internal::Env::fail_with_error(self, error.into());
        #[cfg(target_family = "wasm")]
        core::arch::wasm32::unreachable();
        #[cfg(not(target_family = "wasm"))]
        unreachable!();
    }

    /// Get a [Storage] for accessing and updating persistent data owned by the
    /// currently executing contract.
    #[inline(always)]
    pub fn storage(&self) -> Storage {
        Storage::new(self)
    }

    /// Get [Events] for publishing events associated with the
    /// currently executing contract.
    #[inline(always)]
    pub fn events(&self) -> Events {
        Events::new(self)
    }

    /// Get a [Ledger] for accessing the current ledger.
    #[inline(always)]
    pub fn ledger(&self) -> Ledger {
        Ledger::new(self)
    }

    /// Get a deployer for deploying contracts.
    #[inline(always)]
    pub fn deployer(&self) -> Deployer {
        Deployer::new(self)
    }

    /// Get a [Crypto] for accessing the current cryptographic functions.
    #[inline(always)]
    pub fn crypto(&self) -> Crypto {
        Crypto::new(self)
    }

    /// Get a [Prng] for accessing the current functions which provide pseudo-randomness.
    ///
    /// # Warning
    ///
    /// **The pseudo-random generator returned is not suitable for
    /// security-sensitive work.**
    #[inline(always)]
    pub fn prng(&self) -> Prng {
        Prng::new(self)
    }

    /// Get the Address object corresponding to the current executing contract.
    pub fn current_contract_address(&self) -> Address {
        let address = internal::Env::get_current_contract_address(self).unwrap_infallible();
        unsafe { Address::unchecked_new(self.clone(), address) }
    }

    #[doc(hidden)]
    pub(crate) fn require_auth_for_args(&self, address: &Address, args: Vec<Val>) {
        internal::Env::require_auth_for_args(self, address.to_object(), args.to_object())
            .unwrap_infallible();
    }

    #[doc(hidden)]
    pub(crate) fn require_auth(&self, address: &Address) {
        internal::Env::require_auth(self, address.to_object()).unwrap_infallible();
    }

    /// Returns the contract call stack as a [`Vec`]
    /// of `(contract_id, function_name)`.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contractimpl, log, Env, Symbol};
    ///
    /// #[contract]
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn hello(env: Env) {
    ///         let stack = env.call_stack();
    ///         assert_eq!(stack.len(), 1);
    ///
    ///         let outer = stack.get_unchecked(0);
    ///         log!(&env, "{}", outer);
    ///     }
    /// }
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     let env = Env::default();
    ///     let contract_id = env.register_contract(None, Contract);
    ///     let client = ContractClient::new(&env, &contract_id);
    ///     client.hello();
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn call_stack(&self) -> Vec<(Address, crate::Symbol)> {
        // TODO: Change host fn to return Addresses, so that contracts do not
        // need to iterate over the call stack and do conversion.
        let stack = internal::Env::get_current_call_stack(self).unwrap_infallible();

        let stack =
            unsafe { Vec::<(AddressObject, crate::Symbol)>::unchecked_new(self.clone(), stack) };

        let mut stack_with_addresses = Vec::new(self);
        for (ao, sym) in stack.iter() {
            unsafe {
                stack_with_addresses.push_back((Address::unchecked_new(self.clone(), ao), sym))
            };
        }

        stack_with_addresses
    }

    /// Invokes a function of a contract that is registered in the [Env].
    ///
    /// # Panics
    ///
    /// Will panic if the `contract_id` does not match a registered contract,
    /// `func` does not match a function of the referenced contract, or the
    /// number of `args` do not match the argument count of the referenced
    /// contract function.
    ///
    /// Will panic if the contract that is invoked fails or aborts in anyway.
    ///
    /// Will panic if the value returned from the contract cannot be converted
    /// into the type `T`.
    pub fn invoke_contract<T>(
        &self,
        contract_address: &Address,
        func: &crate::Symbol,
        args: Vec<Val>,
    ) -> T
    where
        T: TryFromVal<Env, Val>,
    {
        let rv = internal::Env::call(
            self,
            contract_address.to_object(),
            func.to_symbol_val(),
            args.to_object(),
        )
        .unwrap_infallible();
        T::try_from_val(self, &rv)
            .map_err(|_| ConversionError)
            .unwrap()
    }

    /// Invokes a function of a contract that is registered in the [Env],
    /// returns an error if the invocation fails for any reason.
    pub fn try_invoke_contract<T, E>(
        &self,
        contract_address: &Address,
        func: &crate::Symbol,
        args: Vec<Val>,
    ) -> Result<Result<T, T::Error>, Result<E, E::Error>>
    where
        T: TryFromVal<Env, Val>,
        E: TryFrom<Error>,
    {
        let rv = internal::Env::try_call(
            self,
            contract_address.to_object(),
            func.to_symbol_val(),
            args.to_object(),
        )
        .unwrap_infallible();
        match internal::Error::try_from_val(self, &rv) {
            Ok(err) => Err(E::try_from(err)),
            Err(ConversionError) => Ok(T::try_from_val(self, &rv)),
        }
    }

    /// Authorizes sub-contract calls on behalf of the current contract.
    ///
    /// All the direct calls that the current contract performs are always
    /// considered to have been authorized. This is only needed to authorize
    /// deeper calls that originate from the next contract call from the current
    /// contract.
    ///
    /// For example, if the contract A calls contract B, contract
    /// B calls contract C and contract C calls `A.require_auth()`, then an
    /// entry corresponding to C call has to be passed in `auth_entries`. It
    /// doesn't matter if contract B called `require_auth` or not. If contract A
    /// calls contract B again, then `authorize_as_current_contract` has to be
    /// called again with the respective entries.
    ///
    ///
    pub fn authorize_as_current_contract(&self, auth_entries: Vec<InvokerContractAuthEntry>) {
        internal::Env::authorize_as_curr_contract(self, auth_entries.to_object())
            .unwrap_infallible();
    }

    /// Get the [Logs] for logging debug events.
    #[inline(always)]
    #[deprecated(note = "use [Env::logs]")]
    #[doc(hidden)]
    pub fn logger(&self) -> Logs {
        self.logs()
    }

    /// Get the [Logs] for logging debug events.
    #[inline(always)]
    pub fn logs(&self) -> Logs {
        Logs::new(self)
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::auth;
#[cfg(any(test, feature = "testutils"))]
use crate::testutils::{
    budget::Budget, random, Address as _, AuthorizedInvocation, ContractFunctionSet, Ledger as _,
    MockAuth, MockAuthContract,
};
#[cfg(any(test, feature = "testutils"))]
use crate::{Bytes, BytesN};
#[cfg(any(test, feature = "testutils"))]
use soroban_ledger_snapshot::LedgerSnapshot;
#[cfg(any(test, feature = "testutils"))]
use std::{path::Path, rc::Rc};
#[cfg(any(test, feature = "testutils"))]
use xdr::{
    Hash, LedgerEntry, LedgerKey, LedgerKeyContractData, ScErrorCode, ScErrorType,
    SorobanAuthorizationEntry,
};
#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl Env {
    #[doc(hidden)]
    pub fn host(&self) -> &internal::Host {
        &self.env_impl
    }

    fn default_with_testutils() -> Env {
        struct EmptySnapshotSource();

        impl internal::storage::SnapshotSource for EmptySnapshotSource {
            fn get(
                &self,
                _key: &Rc<xdr::LedgerKey>,
            ) -> Result<(Rc<xdr::LedgerEntry>, Option<u32>), soroban_env_host::HostError>
            {
                let err: internal::Error = (ScErrorType::Storage, ScErrorCode::MissingValue).into();
                Err(err.into())
            }

            fn has(&self, _key: &Rc<xdr::LedgerKey>) -> Result<bool, soroban_env_host::HostError> {
                Ok(false)
            }
        }

        let rf = Rc::new(EmptySnapshotSource());
        let storage = internal::storage::Storage::with_recording_footprint(rf);
        let budget = internal::budget::Budget::default();
        let env_impl = internal::EnvImpl::with_storage_and_budget(storage, budget.clone());
        env_impl
            .set_source_account(xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(
                xdr::Uint256(random()),
            )))
            .unwrap();
        env_impl
            .set_diagnostic_level(internal::DiagnosticLevel::Debug)
            .unwrap();
        env_impl.set_base_prng_seed([0; 32]).unwrap();
        let env = Env {
            env_impl,
            snapshot: None,
        };

        env.ledger().set(internal::LedgerInfo {
            protocol_version: 20,
            sequence_number: 0,
            timestamp: 0,
            network_id: [0; 32],
            base_reserve: 0,
            min_persistent_entry_expiration: 4096,
            min_temp_entry_expiration: 16,
            max_entry_expiration: 6_312_000,
        });

        env
    }

    /// Register a contract with the [Env] for testing.
    ///
    /// Passing a contract ID for the first arguments registers the contract
    /// with that contract ID. Providing `None` causes a random ID to be
    /// assigned to the contract.
    ///
    /// Registering a contract that is already registered replaces it.
    ///
    /// Returns the contract ID of the registered contract.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contractimpl, BytesN, Env, Symbol};
    ///
    /// #[contract]
    /// pub struct HelloContract;
    ///
    /// #[contractimpl]
    /// impl HelloContract {
    ///     pub fn hello(env: Env, recipient: Symbol) -> Symbol {
    ///         todo!()
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let env = Env::default();
    ///     let contract_id = env.register_contract(None, HelloContract);
    /// }
    /// ```
    pub fn register_contract<'a, T: ContractFunctionSet + 'static>(
        &self,
        contract_id: impl Into<Option<&'a Address>>,
        contract: T,
    ) -> Address {
        struct InternalContractFunctionSet<T: ContractFunctionSet>(pub(crate) T);
        impl<T: ContractFunctionSet> internal::ContractFunctionSet for InternalContractFunctionSet<T> {
            fn call(
                &self,
                func: &Symbol,
                env_impl: &internal::EnvImpl,
                args: &[Val],
            ) -> Option<Val> {
                let env = Env::with_impl(env_impl.clone());
                self.0.call(
                    crate::Symbol::try_from_val(&env, func)
                        .unwrap_infallible()
                        .to_string()
                        .as_str(),
                    env,
                    args,
                )
            }
        }

        let contract_id = if let Some(contract_id) = contract_id.into() {
            contract_id.clone()
        } else {
            Address::random(self)
        };
        self.env_impl
            .register_test_contract(
                contract_id.to_object(),
                Rc::new(InternalContractFunctionSet(contract)),
            )
            .unwrap();
        contract_id
    }

    /// Register a contract in a WASM file with the [Env] for testing.
    ///
    /// Passing a contract ID for the first arguments registers the contract
    /// with that contract ID. Providing `None` causes a random ID to be
    /// assigned to the contract.
    ///
    /// Registering a contract that is already registered replaces it.
    ///
    /// Returns the contract ID of the registered contract.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{BytesN, Env};
    ///
    /// const WASM: &[u8] = include_bytes!("../doctest_fixtures/contract.wasm");
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let env = Env::default();
    ///     env.register_contract_wasm(None, WASM);
    /// }
    /// ```
    pub fn register_contract_wasm<'a>(
        &self,
        contract_id: impl Into<Option<&'a Address>>,
        contract_wasm: impl IntoVal<Env, Bytes>,
    ) -> Address {
        let wasm_hash: BytesN<32> = self.deployer().upload_contract_wasm(contract_wasm);
        self.register_contract_with_optional_contract_id_and_executable(
            contract_id,
            xdr::ContractExecutable::Wasm(xdr::Hash(wasm_hash.into())),
        )
    }

    /// Register the built-in Stellar Asset Contract with provided admin address.
    ///
    /// Returns the contract ID of the registered token contract.
    ///
    /// The contract will wrap a randomly-generated Stellar asset. This function
    /// is useful for using in the tests when an arbitrary token contract
    /// instance is needed.
    pub fn register_stellar_asset_contract(&self, admin: Address) -> Address {
        let issuer_pk = random();
        let issuer_id = xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(xdr::Uint256(
            issuer_pk.clone(),
        )));

        self.host()
            .with_mut_storage(|storage| {
                let k = Rc::new(xdr::LedgerKey::Account(xdr::LedgerKeyAccount {
                    account_id: issuer_id.clone(),
                }));

                if !storage.has(
                    &k,
                    soroban_env_host::budget::AsBudget::as_budget(self.host()),
                )? {
                    let v = Rc::new(xdr::LedgerEntry {
                        data: xdr::LedgerEntryData::Account(xdr::AccountEntry {
                            account_id: issuer_id.clone(),
                            balance: 0,
                            flags: 0,
                            home_domain: Default::default(),
                            inflation_dest: None,
                            num_sub_entries: 0,
                            seq_num: xdr::SequenceNumber(0),
                            thresholds: xdr::Thresholds([1; 4]),
                            signers: xdr::VecM::default(),
                            ext: xdr::AccountEntryExt::V0,
                        }),
                        last_modified_ledger_seq: 0,
                        ext: xdr::LedgerEntryExt::V0,
                    });
                    storage.put(
                        &k,
                        &v,
                        None,
                        soroban_env_host::budget::AsBudget::as_budget(self.host()),
                    )?
                }
                Ok(())
            })
            .unwrap();

        let asset = xdr::Asset::CreditAlphanum4(xdr::AlphaNum4 {
            asset_code: xdr::AssetCode4([b'a', b'a', b'a', b'a']),
            issuer: issuer_id.clone(),
        });
        let create = xdr::HostFunction::CreateContract(xdr::CreateContractArgs {
            contract_id_preimage: xdr::ContractIdPreimage::Asset(asset),
            executable: xdr::ContractExecutable::Token,
        });

        let token_id: Address = self
            .env_impl
            .invoke_function(create)
            .unwrap()
            .try_into_val(self)
            .unwrap();

        let prev_auth_manager = self.env_impl.snapshot_auth_manager().unwrap();
        self.env_impl.switch_to_recording_auth(true).unwrap();
        self.invoke_contract::<()>(
            &token_id,
            &soroban_sdk_macros::internal_symbol_short!("set_admin"),
            (admin,).try_into_val(self).unwrap(),
        );
        self.env_impl.set_auth_manager(prev_auth_manager).unwrap();
        token_id
    }

    fn register_contract_with_optional_contract_id_and_executable<'a>(
        &self,
        contract_id: impl Into<Option<&'a Address>>,
        executable: xdr::ContractExecutable,
    ) -> Address {
        if let Some(contract_id) = contract_id.into() {
            self.register_contract_with_contract_id_and_executable(contract_id, executable);
            contract_id.clone()
        } else {
            self.register_contract_with_source(executable)
        }
    }

    fn register_contract_with_source(&self, executable: xdr::ContractExecutable) -> Address {
        let prev_auth_manager = self.env_impl.snapshot_auth_manager().unwrap();
        self.env_impl.switch_to_recording_auth(true).unwrap();

        let contract_id: Address = self
            .env_impl
            .invoke_function(xdr::HostFunction::CreateContract(xdr::CreateContractArgs {
                contract_id_preimage: xdr::ContractIdPreimage::Address(
                    xdr::ContractIdPreimageFromAddress {
                        address: xdr::ScAddress::Contract(xdr::Hash(random())),
                        salt: xdr::Uint256(random()),
                    },
                ),
                executable,
            }))
            .unwrap()
            .try_into_val(self)
            .unwrap();

        self.env_impl.set_auth_manager(prev_auth_manager).unwrap();

        contract_id
    }

    /// Set authorizations and signatures in the environment which will be
    /// consumed by contracts when they invoke [`Address::require_auth`] or
    /// [`Address::require_auth_for_args`] functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    ///
    /// This function can also be called on contract clients.
    ///
    /// To mock auth for testing, without requiring valid signatures, use
    /// [`mock_all_auths`][Self::mock_all_auths] or
    /// [`mock_auths`][Self::mock_auths]. If mocking of auths is enabled,
    /// calling [`set_auths`][Self::set_auths] disables any mocking.
    pub fn set_auths(&self, auths: &[SorobanAuthorizationEntry]) {
        self.env_impl
            .set_authorization_entries(auths.to_vec())
            .unwrap();
    }

    /// Mock authorizations in the environment which will cause matching invokes
    /// of [`Address::require_auth`] and [`Address::require_auth_for_args`] to
    /// pass.
    ///
    /// This function can also be called on contract clients.
    ///
    /// Authorizations not matching a mocked auth will fail.
    ///
    /// To mock all auths, use [`mock_all_auths`][Self::mock_all_auths].
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contractimpl, Env, Address, testutils::{Address as _, MockAuth, MockAuthInvoke}, IntoVal};
    ///
    /// #[contract]
    /// pub struct HelloContract;
    ///
    /// #[contractimpl]
    /// impl HelloContract {
    ///     pub fn hello(env: Env, from: Address) {
    ///         from.require_auth();
    ///         // TODO
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let env = Env::default();
    ///     let contract_id = env.register_contract(None, HelloContract);
    ///
    ///     let client = HelloContractClient::new(&env, &contract_id);
    ///     let addr = Address::random(&env);
    ///     client.mock_auths(&[
    ///         MockAuth {
    ///             address: &addr,
    ///             invoke: &MockAuthInvoke {
    ///                 contract: &contract_id,
    ///                 fn_name: "hello",
    ///                 args: (&addr,).into_val(&env),
    ///                 sub_invokes: &[],
    ///             },
    ///         },
    ///     ]).hello(&addr);
    /// }
    /// ```
    pub fn mock_auths(&self, auths: &[MockAuth]) {
        for a in auths {
            self.register_contract(a.address, MockAuthContract);
        }
        let auths = auths
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<std::vec::Vec<_>>();
        self.env_impl.set_authorization_entries(auths).unwrap();
    }

    /// Mock all calls to the [`Address::require_auth`] and
    /// [`Address::require_auth_for_args`] functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// When mocking is enabled, if the [`Address`] being authorized is the
    /// address of a contract, that contract's `__check_auth` function will not
    /// be called, and the contract does not need to exist or be registered in
    /// the test.
    ///
    /// When mocking is enabled, if the [`Address`] being authorized is the
    /// address of an account, the account does not need to exist.
    ///
    /// This function can also be called on contract clients.
    ///
    /// To disable mocking, see [`set_auths`][Self::set_auths].
    ///
    /// To access a list of auths that have occurred, see [`auths`][Self::auths].
    ///
    /// It is not currently possible to mock a subset of auths.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contractimpl, Env, Address, testutils::Address as _};
    ///
    /// #[contract]
    /// pub struct HelloContract;
    ///
    /// #[contractimpl]
    /// impl HelloContract {
    ///     pub fn hello(env: Env, from: Address) {
    ///         from.require_auth();
    ///         // TODO
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let env = Env::default();
    ///     let contract_id = env.register_contract(None, HelloContract);
    ///
    ///     env.mock_all_auths();
    ///
    ///     let client = HelloContractClient::new(&env, &contract_id);
    ///     let addr = Address::random(&env);
    ///     client.hello(&addr);
    /// }
    /// ```
    pub fn mock_all_auths(&self) {
        self.env_impl.switch_to_recording_auth(true).unwrap();
    }

    /// A version of `mock_all_auths` that allows authorizations that are not
    /// present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for general information and
    /// prefer using `mock_all_auths` unless non-root authorization is required.
    ///
    /// The only difference from `mock_all_auths` is that this won't return an
    /// error when `require_auth` hasn't been called in the root invocation for
    /// any given address. This is useful to test contracts that bundle calls to
    /// another contract without atomicity requirements (i.e. any contract call
    /// can be frontrun).
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contractimpl, Env, Address, testutils::Address as _};
    ///
    /// #[contract]
    /// pub struct ContractA;
    ///
    /// #[contractimpl]
    /// impl ContractA {
    ///     pub fn do_auth(env: Env, addr: Address) {
    ///         addr.require_auth();
    ///     }
    /// }
    /// #[contract]
    /// pub struct ContractB;
    ///
    /// #[contractimpl]
    /// impl ContractB {
    ///     pub fn call_a(env: Env, contract_a: Address, addr: Address) {
    ///         // Notice there is no `require_auth` call here.
    ///         ContractAClient::new(&env, &contract_a).do_auth(&addr);
    ///     }
    /// }
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let env = Env::default();
    ///     let contract_a = env.register_contract(None, ContractA);
    ///     let contract_b = env.register_contract(None, ContractB);
    ///     // The regular `env.mock_all_auths()` would result in the call
    ///     // failure.
    ///     env.mock_all_auths_allowing_non_root_auth();
    ///
    ///     let client = ContractBClient::new(&env, &contract_b);
    ///     let addr = Address::random(&env);
    ///     client.call_a(&contract_a, &addr);
    /// }
    /// ```
    pub fn mock_all_auths_allowing_non_root_auth(&self) {
        self.env_impl.switch_to_recording_auth(false).unwrap();
    }

    /// Returns a list of authorization trees that were seen during the last
    /// contract or authorized host function invocation.
    ///
    /// Use this in tests to verify that the expected authorizations with the
    /// expected arguments are required.
    ///
    /// The return value is a vector of authorizations represented by tuples of
    /// `(address, AuthorizedInvocation)`. `AuthorizedInvocation` describes the
    /// tree of `require_auth_for_args(address, args)` from the contract
    /// functions (or `require_auth` with all the arguments of the function
    /// invocation). It also might contain the authorized host functions (
    /// currently CreateContract is the only such function) in case if
    /// corresponding host functions have been called.
    ///
    /// Refer to documentation for `AuthorizedInvocation` for detailed
    /// information on its contents.
    ///
    /// The order of the returned vector is defined by the order of
    /// [`Address::require_auth`] calls. Repeated calls to
    /// [`Address::require_auth`] with the same address and args in the same
    /// tree of contract invocations will appear only once in the vector.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contractimpl, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, symbol_short, Address, Symbol, Env, IntoVal};
    ///
    /// #[contract]
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn transfer(env: Env, address: Address, amount: i128) {
    ///         address.require_auth();
    ///     }
    ///     pub fn transfer2(env: Env, address: Address, amount: i128) {
    ///         address.require_auth_for_args((amount / 2,).into_val(&env));
    ///     }
    /// }
    ///
    /// #[test]
    /// fn test() {
    /// # }
    /// # #[cfg(feature = "testutils")]
    /// # fn main() {
    ///     extern crate std;
    ///     let env = Env::default();
    ///     let contract_id = env.register_contract(None, Contract);
    ///     let client = ContractClient::new(&env, &contract_id);
    ///     env.mock_all_auths();
    ///     let address = Address::random(&env);
    ///     client.transfer(&address, &1000_i128);
    ///     assert_eq!(
    ///         env.auths(),
    ///         std::vec![(
    ///             address.clone(),
    ///             AuthorizedInvocation {
    ///                 function: AuthorizedFunction::Contract((
    ///                     client.address.clone(),
    ///                     symbol_short!("transfer"),
    ///                     (&address, 1000_i128,).into_val(&env)
    ///                 )),
    ///                 sub_invocations: std::vec![]
    ///             }
    ///         )]
    ///     );
    ///
    ///     client.transfer2(&address, &1000_i128);
    ///     assert_eq!(
    ///         env.auths(),
    ///        std::vec![(
    ///             address.clone(),
    ///             AuthorizedInvocation {
    ///                 function: AuthorizedFunction::Contract((
    ///                     client.address.clone(),
    ///                     symbol_short!("transfer2"),
    ///                     // `transfer2` requires auth for (amount / 2) == (1000 / 2) == 500.
    ///                     (500_i128,).into_val(&env)
    ///                 )),
    ///                 sub_invocations: std::vec![]
    ///             }
    ///         )]
    ///     );
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn auths(&self) -> std::vec::Vec<(Address, AuthorizedInvocation)> {
        let authorizations = self.env_impl.get_authenticated_authorizations().unwrap();
        authorizations
            .into_iter()
            .map(|(sc_addr, invocation)| {
                (
                    xdr::ScVal::Address(sc_addr).try_into_val(self).unwrap(),
                    AuthorizedInvocation::from_xdr(self, &invocation),
                )
            })
            .collect()
    }

    /// Invokes the special `__check_auth` function of contracts that implement
    /// the custom account interface.
    ///
    /// `__check_auth` can't be called outside of the host-managed `require_auth`
    /// calls. This test utility allows testing custom account contracts without
    /// the need to setup complex contract call trees and enabling the enforcing
    /// auth on the host side.
    ///
    /// This function requires to provide the template argument for error. Use
    /// `soroban_sdk::Error` if `__check_auth` doesn't return a special
    /// contract error and use the error with `contracterror` attribute
    /// otherwise.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contract, contracterror, contractimpl, testutils::{Address as _, BytesN as _}, vec, auth::Context, BytesN, Env, Vec, Val};
    ///
    /// #[contracterror]
    /// #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
    /// #[repr(u32)]
    /// pub enum NoopAccountError {
    ///     SomeError = 1,
    /// }
    /// #[contract]
    /// struct NoopAccountContract;
    /// #[contractimpl]
    /// impl NoopAccountContract {
    ///
    ///     #[allow(non_snake_case)]
    ///     pub fn __check_auth(
    ///         _env: Env,
    ///         _signature_payload: BytesN<32>,
    ///         signatures: Vec<Val>,
    ///         _auth_context: Vec<Context>,
    ///     ) -> Result<(), NoopAccountError> {
    ///         if signatures.is_empty() {
    ///             Err(NoopAccountError::SomeError)
    ///         } else {
    ///             Ok(())
    ///         }
    ///     }
    /// }
    /// #[test]
    /// fn test() {
    /// # }
    /// # fn main() {
    ///     let e: Env = Default::default();
    ///     let account_contract = NoopAccountContractClient::new(&e, &e.register_contract(None, NoopAccountContract));
    ///     // Non-succesful call of `__check_auth` with a `contracterror` error.
    ///     assert_eq!(
    ///         e.try_invoke_contract_check_auth::<NoopAccountError>(
    ///             &account_contract.address.contract_id(),
    ///             &BytesN::random(&e),
    ///             &vec![&e],
    ///             &vec![&e],
    ///         ),
    ///         // The inner `Result` is for conversion error and will be Ok
    ///         // as long as a valid error type used.
    ///         Err(Ok(NoopAccountError::SomeError))
    ///     );
    ///     // Successful call of `__check_auth` with a `soroban_sdk::Error`
    ///     // error - this should be compatible with any error type.
    ///     assert_eq!(
    ///         e.try_invoke_contract_check_auth::<soroban_sdk::Error>(
    ///             &account_contract.address.contract_id(),
    ///             &BytesN::random(&e),
    ///             &vec![&e, 0_i32.into()],
    ///             &vec![&e],
    ///         ),
    ///         Ok(())
    ///     );
    /// }
    /// ```
    pub fn try_invoke_contract_check_auth<E: TryFrom<Error>>(
        &self,
        contract: &BytesN<32>,
        signature_payload: &BytesN<32>,
        signatures: &Vec<Val>,
        auth_context: &Vec<auth::Context>,
    ) -> Result<(), Result<E, E::Error>> {
        let args = Vec::from_array(
            self,
            [
                signature_payload.to_val(),
                signatures.to_val(),
                auth_context.to_val(),
            ],
        );
        let res = self
            .host()
            .call_account_contract_check_auth(contract.to_object(), args.to_object());
        match res {
            Ok(rv) => Ok(rv.into_val(self)),
            Err(e) => Err(e.error.try_into()),
        }
    }

    fn register_contract_with_contract_id_and_executable(
        &self,
        contract_id: &Address,
        executable: xdr::ContractExecutable,
    ) {
        let contract_id_hash = Hash(contract_id.contract_id().into());
        let data_key = xdr::ScVal::LedgerKeyContractInstance;
        let key = Rc::new(LedgerKey::ContractData(LedgerKeyContractData {
            contract: xdr::ScAddress::Contract(contract_id_hash.clone()),
            key: data_key.clone(),
            durability: xdr::ContractDataDurability::Persistent,
        }));

        let instance = xdr::ScContractInstance {
            executable,
            storage: Default::default(),
        };

        let entry = Rc::new(LedgerEntry {
            ext: xdr::LedgerEntryExt::V0,
            last_modified_ledger_seq: 0,
            data: xdr::LedgerEntryData::ContractData(xdr::ContractDataEntry {
                contract: xdr::ScAddress::Contract(contract_id_hash.clone()),
                key: data_key,
                val: xdr::ScVal::ContractInstance(instance),
                durability: xdr::ContractDataDurability::Persistent,
                ext: xdr::ExtensionPoint::V0,
            }),
        });
        let expiration_ledger = self.ledger().sequence() + 1;
        self.env_impl
            .with_mut_storage(|storage| {
                storage.put(
                    &key,
                    &entry,
                    Some(expiration_ledger),
                    soroban_env_host::budget::AsBudget::as_budget(self.host()),
                )
            })
            .unwrap();
    }

    /// Run the function as if executed by the given contract ID.
    ///
    /// Used to write or read contract data, or take other actions in tests for
    /// setting up tests or asserting on internal state.
    pub fn as_contract<T>(&self, id: &Address, f: impl FnOnce() -> T) -> T {
        let id: [u8; 32] = id.contract_id().into();
        let func = Symbol::from_small_str("");
        let mut t: Option<T> = None;
        self.env_impl
            .with_test_contract_frame(id.into(), func, || {
                t = Some(f());
                Ok(().into())
            })
            .unwrap();
        t.unwrap()
    }

    /// Creates a new Env loaded with the [`LedgerSnapshot`].
    ///
    /// The ledger info and state in the snapshot are loaded into the Env.
    pub fn from_snapshot(s: LedgerSnapshot) -> Env {
        let info = s.ledger_info();

        let rs = Rc::new(s.clone());
        let storage = internal::storage::Storage::with_recording_footprint(rs.clone());
        let budget = internal::budget::Budget::default();
        let env_impl = internal::EnvImpl::with_storage_and_budget(storage, budget.clone());
        env_impl.switch_to_recording_auth(true).unwrap();

        let env = Env {
            env_impl,
            snapshot: Some(rs.clone()),
        };
        env.ledger().set(info);
        env
    }

    /// Creates a new Env loaded with the ledger snapshot loaded from the file.
    ///
    /// ### Panics
    ///
    /// If there is any error reading the file.
    pub fn from_snapshot_file(p: impl AsRef<Path>) -> Env {
        Self::from_snapshot(LedgerSnapshot::read_file(p).unwrap())
    }

    /// Create a snapshot from the Env's current state.
    pub fn to_snapshot(&self) -> LedgerSnapshot {
        let snapshot = self.snapshot.clone().unwrap_or_default();
        let mut snapshot = (*snapshot).clone();
        snapshot.set_ledger_info(self.ledger().get());
        let budget = soroban_env_host::budget::AsBudget::as_budget(&self.env_impl);
        let storage = self
            .env_impl
            .with_mut_storage(|s| Ok(s.map.clone()))
            .unwrap();
        snapshot.update_entries(storage.iter(budget).unwrap());
        snapshot
    }

    /// Create a snapshot file from the Env's current state.
    ///
    /// ### Panics
    ///
    /// If there is any error writing the file.
    pub fn to_snapshot_file(&self, p: impl AsRef<Path>) {
        self.to_snapshot().write_file(p).unwrap();
    }

    /// Get the budget that tracks the resources consumed for the environment.
    pub fn budget(&self) -> Budget {
        Budget::new(self.env_impl.budget_cloned())
    }
}

#[doc(hidden)]
impl Env {
    pub fn with_impl(env_impl: internal::EnvImpl) -> Env {
        Env {
            env_impl,
            #[cfg(any(test, feature = "testutils"))]
            snapshot: None,
        }
    }
}

#[doc(hidden)]
impl internal::EnvBase for Env {
    type Error = Infallible;

    // This exists to allow code in conversion paths to upgrade an Error to an
    // Env::Error with some control granted to the underlying Env (and panic
    // paths kept out of the host). We delegate this to our env_impl and then,
    // since our own Error type is Infallible, immediately throw it into either
    // the env_impl's Error escalation path (if testing), or just plain panic.
    #[cfg(not(target_family = "wasm"))]
    fn error_from_error_val(&self, e: crate::Error) -> Self::Error {
        let host_err = self.env_impl.error_from_error_val(e);
        #[cfg(any(test, feature = "testutils"))]
        self.env_impl.escalate_error_to_panic(host_err);
        #[cfg(not(any(test, feature = "testutils")))]
        panic!("{:?}", host_err);
    }

    // When targeting wasm we don't even need to do that, just delegate to
    // the Guest's impl, which calls core::arch::wasm32::unreachable.
    #[cfg(target_family = "wasm")]
    fn error_from_error_val(&self, e: crate::Error) -> Self::Error {
        self.env_impl.error_from_error_val(e)
    }

    // Note: the function `escalate_error_to_panic` only exists _on the `Env`
    // trait_ when the feature `soroban-env-common/testutils` is enabled. This
    // is because the host wants to never have this function even _compiled in_
    // when building for production, as it might be accidentally called (we have
    // mistakenly done so with conversion and comparison traits in the past).
    //
    // As a result, we only implement it here (fairly meaninglessly) when we're
    // in `cfg(test)` (which enables `soroban-env-host/testutils` thus
    // `soroban-env-common/testutils`) or when we've had our own `testutils`
    // feature enabled (which does the same).
    //
    // See the `internal::reject_err` functions above for more detail about what
    // it actually does (when implemented for real, on the host). In this
    // not-very-serious impl, since `Self::Error` is `Infallible`, this instance
    // can never actually be called and so its body is just a trivial
    // transformation from one empty type to another, for Type System Reasons.
    #[cfg(any(test, feature = "testutils"))]
    fn escalate_error_to_panic(&self, e: Self::Error) -> ! {
        match e {}
    }

    fn check_same_env(&self, other: &Self) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .check_same_env(&other.env_impl)
            .unwrap_optimized())
    }

    fn bytes_copy_from_slice(
        &self,
        b: BytesObject,
        b_pos: U32Val,
        slice: &[u8],
    ) -> Result<BytesObject, Self::Error> {
        Ok(self
            .env_impl
            .bytes_copy_from_slice(b, b_pos, slice)
            .unwrap_optimized())
    }

    fn bytes_copy_to_slice(
        &self,
        b: BytesObject,
        b_pos: U32Val,
        slice: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .bytes_copy_to_slice(b, b_pos, slice)
            .unwrap_optimized())
    }

    fn bytes_new_from_slice(&self, slice: &[u8]) -> Result<BytesObject, Self::Error> {
        Ok(self.env_impl.bytes_new_from_slice(slice).unwrap_optimized())
    }

    fn log_from_slice(&self, msg: &str, args: &[Val]) -> Result<Void, Self::Error> {
        Ok(self.env_impl.log_from_slice(msg, args).unwrap_optimized())
    }

    fn string_copy_to_slice(
        &self,
        b: StringObject,
        b_pos: U32Val,
        slice: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .string_copy_to_slice(b, b_pos, slice)
            .unwrap_optimized())
    }

    fn symbol_copy_to_slice(
        &self,
        b: SymbolObject,
        b_pos: U32Val,
        mem: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .symbol_copy_to_slice(b, b_pos, mem)
            .unwrap_optimized())
    }

    fn string_new_from_slice(&self, slice: &str) -> Result<StringObject, Self::Error> {
        Ok(self
            .env_impl
            .string_new_from_slice(slice)
            .unwrap_optimized())
    }

    fn symbol_new_from_slice(&self, slice: &str) -> Result<SymbolObject, Self::Error> {
        Ok(self
            .env_impl
            .symbol_new_from_slice(slice)
            .unwrap_optimized())
    }

    fn map_new_from_slices(&self, keys: &[&str], vals: &[Val]) -> Result<MapObject, Self::Error> {
        Ok(self
            .env_impl
            .map_new_from_slices(keys, vals)
            .unwrap_optimized())
    }

    fn map_unpack_to_slice(
        &self,
        map: MapObject,
        keys: &[&str],
        vals: &mut [Val],
    ) -> Result<Void, Self::Error> {
        Ok(self
            .env_impl
            .map_unpack_to_slice(map, keys, vals)
            .unwrap_optimized())
    }

    fn vec_new_from_slice(&self, vals: &[Val]) -> Result<VecObject, Self::Error> {
        Ok(self.env_impl.vec_new_from_slice(vals).unwrap_optimized())
    }

    fn vec_unpack_to_slice(&self, vec: VecObject, vals: &mut [Val]) -> Result<Void, Self::Error> {
        Ok(self
            .env_impl
            .vec_unpack_to_slice(vec, vals)
            .unwrap_optimized())
    }

    fn symbol_index_in_strs(&self, key: Symbol, strs: &[&str]) -> Result<U32Val, Self::Error> {
        Ok(self
            .env_impl
            .symbol_index_in_strs(key, strs)
            .unwrap_optimized())
    }
}

///////////////////////////////////////////////////////////////////////////////
/// X-macro use: impl Env for SDK's Env
///////////////////////////////////////////////////////////////////////////////

// This is a helper macro used only by impl_env_for_sdk below. It consumes a
// token-tree of the form:
//
//  {fn $fn_id:ident $args:tt -> $ret:ty}
//
// and produces the the corresponding method definition to be used in the
// SDK's Env implementation of the Env (calling through to the corresponding
// guest or host implementation).
macro_rules! sdk_function_helper {
    {$mod_id:ident, fn $fn_id:ident($($arg:ident:$type:ty),*) -> $ret:ty}
    =>
    {
        fn $fn_id(&self, $($arg:$type),*) -> Result<$ret, Self::Error> {
            internal::reject_err(&self.env_impl, self.env_impl.$fn_id($($arg),*))
        }
    };
}

// This is a callback macro that pattern-matches the token-tree passed by the
// x-macro (call_macro_with_all_host_functions) and produces a suite of
// forwarding-method definitions, which it places in the body of the declaration
// of the implementation of Env for the SDK's Env.
macro_rules! impl_env_for_sdk {
    {
        $(
            // This outer pattern matches a single 'mod' block of the token-tree
            // passed from the x-macro to this macro. It is embedded in a `$()*`
            // pattern-repetition matcher so that it will match all provided
            // 'mod' blocks provided.
            $(#[$mod_attr:meta])*
            mod $mod_id:ident $mod_str:literal
            {
                $(
                    // This inner pattern matches a single function description
                    // inside a 'mod' block in the token-tree passed from the
                    // x-macro to this macro. It is embedded in a `$()*`
                    // pattern-repetition matcher so that it will match all such
                    // descriptions.
                    $(#[$fn_attr:meta])*
                    { $fn_str:literal, fn $fn_id:ident $args:tt -> $ret:ty }
                )*
            }
        )*
    }

    =>  // The part of the macro above this line is a matcher; below is its expansion.

    {
        // This macro expands to a single item: the implementation of Env for
        // the SDK's Env struct used by client contract code running in a WASM VM.
        #[doc(hidden)]
        impl internal::Env for Env
        {
            $(
                $(
                    // This invokes the guest_function_helper! macro above
                    // passing only the relevant parts of the declaration
                    // matched by the inner pattern above. It is embedded in two
                    // nested `$()*` pattern-repetition expanders that
                    // correspond to the pattern-repetition matchers in the
                    // match section, but we ignore the structure of the 'mod'
                    // block repetition-level from the outer pattern in the
                    // expansion, flattening all functions from all 'mod' blocks
                    // into the implementation of Env for Guest.
                    sdk_function_helper!{$mod_id, fn $fn_id $args -> $ret}
                )*
            )*
        }
    };
}

// Here we invoke the x-macro passing generate_env_trait as its callback macro.
internal::call_macro_with_all_host_functions! { impl_env_for_sdk }
