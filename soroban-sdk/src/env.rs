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
pub use internal::MapObject;
pub use internal::RawVal;
pub use internal::RawValConvertible;
pub use internal::Status;
pub use internal::SymbolStr;
pub use internal::TryFromVal;
pub use internal::TryIntoVal;
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

use crate::unwrap::UnwrapInfallible;
use crate::unwrap::UnwrapOptimized;
use crate::{
    crypto::Crypto, deploy::Deployer, events::Events, ledger::Ledger, logging::Logger,
    storage::Storage, Address, BytesN, Vec,
};
use internal::{
    AddressObject, Bool, BytesObject, I128Object, I256Object, I64Object, Object, StringObject,
    Symbol, SymbolObject, U128Object, U256Object, U32Val, U64Object, U64Val, Void,
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
    pub fn panic_with_error(&self, error: impl Into<Status>) {
        _ = internal::Env::fail_with_status(self, error.into());
        unreachable!()
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

    /// Get the Address object corresponding to the current executing contract.
    pub fn current_contract_address(&self) -> Address {
        let address = internal::Env::get_current_contract_address(self).unwrap_infallible();
        unsafe { Address::unchecked_new(self.clone(), address) }
    }

    #[doc(hidden)]
    pub(crate) fn require_auth_for_args(&self, address: &Address, args: Vec<RawVal>) {
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
    /// use soroban_sdk::{contractimpl, log, BytesN, Env, Symbol};
    ///
    /// pub struct Contract;
    ///
    /// #[contractimpl]
    /// impl Contract {
    ///     pub fn hello(env: Env) {
    ///         let stack = env.call_stack();
    ///         assert_eq!(stack.len(), 1);
    ///
    ///         let outer = stack.get(0).unwrap().unwrap();
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
            unsafe { Vec::<(BytesN<32>, crate::Symbol)>::unchecked_new(self.clone(), stack) };

        let mut stack_with_addresses = Vec::new(self);
        for (id, sym) in stack.iter_unchecked() {
            stack_with_addresses.push_back((Address::from_contract_id(&id), sym));
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
        contract_id: &Address,
        func: &crate::Symbol,
        args: Vec<RawVal>,
    ) -> T
    where
        T: TryFromVal<Env, RawVal>,
    {
        let rv = internal::Env::call(
            self,
            contract_id.contract_id().to_object(),
            func.to_val(),
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
        contract_id: &Address,
        func: &crate::Symbol,
        args: Vec<RawVal>,
    ) -> Result<Result<T, T::Error>, Result<E, E::Error>>
    where
        T: TryFromVal<Env, RawVal>,
        E: TryFrom<Status>,
    {
        let Some(contract_id) = contract_id.try_contract_id() else {
            return Err(E::try_from(Status::from_status(xdr::ScStatus::UnknownError(xdr::ScUnknownErrorCode::General))));
        };
        let rv = internal::Env::try_call(
            self,
            contract_id.to_object(),
            func.to_val(),
            args.to_object(),
        )
        .unwrap_infallible();
        match Status::try_from_val(self, &rv) {
            Ok(status) => Err(E::try_from(status)),
            Err(ConversionError) => Ok(T::try_from_val(self, &rv)),
        }
    }

    /// Get the [Logger] for logging debug events.
    #[inline(always)]
    pub fn logger(&self) -> Logger {
        Logger::new(self)
    }

    #[doc(hidden)]
    pub fn log_value<V: IntoVal<Env, RawVal>>(&self, v: V) {
        internal::Env::log_value(self, v.into_val(self)).unwrap_infallible();
    }

    /// Replaces the executable of the current contract with the provided Wasm.
    ///
    /// The Wasm blob identified by the `wasm_hash` has to be already present
    /// on-chain (the upload happens via `INSTALL_CONTRACT_CODE` host function
    /// or via `install_contract_wasm` test function in unit tests).
    ///
    /// The function won't do anything immediately. The contract executable
    /// will only be updated after the invocation has successfully finished.
    pub fn update_current_contract_wasm(&self, wasm_hash: &BytesN<32>) {
        internal::Env::update_current_contract_wasm(self, wasm_hash.to_object())
            .unwrap_infallible();
    }
}

#[cfg(any(test, feature = "testutils"))]
use crate::testutils::{
    budget::Budget, random, Address as _, ContractFunctionSet, Ledger as _, MockAuth,
    MockAuthContract,
};
#[cfg(any(test, feature = "testutils"))]
use soroban_ledger_snapshot::LedgerSnapshot;
#[cfg(any(test, feature = "testutils"))]
use std::{path::Path, rc::Rc};
#[cfg(any(test, feature = "testutils"))]
use xdr::{ContractAuth, Hash, LedgerEntry, LedgerKey, LedgerKeyContractData};
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
            ) -> Result<Rc<xdr::LedgerEntry>, soroban_env_host::HostError> {
                use xdr::{ScHostStorageErrorCode, ScStatus};
                let status: internal::Status =
                    ScStatus::HostStorageError(ScHostStorageErrorCode::MissingKeyInGet).into();
                Err(status.into())
            }

            fn has(&self, _key: &Rc<xdr::LedgerKey>) -> Result<bool, soroban_env_host::HostError> {
                Ok(false)
            }
        }

        let rf = Rc::new(EmptySnapshotSource());
        let storage = internal::storage::Storage::with_recording_footprint(rf);
        let budget = internal::budget::Budget::default();
        let env_impl = internal::EnvImpl::with_storage_and_budget(storage, budget.clone());
        env_impl.set_source_account(xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(
            xdr::Uint256(random()),
        )));
        let env = Env {
            env_impl,
            snapshot: None,
        };

        env.ledger().set(internal::LedgerInfo {
            protocol_version: 0,
            sequence_number: 0,
            timestamp: 0,
            network_id: [0; 32],
            base_reserve: 0,
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
    /// use soroban_sdk::{contractimpl, BytesN, Env, Symbol};
    ///
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
                args: &[RawVal],
            ) -> Option<RawVal> {
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
                contract_id.contract_id().to_object(),
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
        contract_wasm: &[u8],
    ) -> Address {
        let wasm_hash: BytesN<32> = self.install_contract_wasm(contract_wasm);
        self.register_contract_with_optional_contract_id_and_executable(
            contract_id,
            xdr::ScContractExecutable::WasmRef(xdr::Hash(wasm_hash.into())),
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
                        soroban_env_host::budget::AsBudget::as_budget(self.host()),
                    )?
                }
                Ok(())
            })
            .unwrap();

        let asset = xdr::Asset::CreditAlphanum4(xdr::AlphaNum4 {
            asset_code: xdr::AssetCode4(random()),
            issuer: issuer_id.clone(),
        });

        let create = xdr::HostFunction {
            args: xdr::HostFunctionArgs::CreateContract(xdr::CreateContractArgs {
                contract_id: xdr::ContractId::Asset(asset.clone()),
                executable: xdr::ScContractExecutable::Token,
            }),
            auth: Default::default(),
        };

        let token_id: BytesN<32> = self.env_impl.invoke_functions(vec![create]).unwrap()[0]
            .try_into_val(self)
            .unwrap();

        // Set the admin of the token to the passed in address. This operation
        // could be performed by calling the token contracts `set_admin`
        // function, however doing so would require modifying the authorization
        // state of the environment and tests may have setup authorization, and
        // the environment does not provide anyway for us to snapshot the
        // current auth setup, modify it, then reset it. This might be possible
        // after this issue is resolved:
        // https://github.com/stellar/rs-soroban-env/issues/785.
        self.host()
            .with_mut_storage(|storage| {
                let key = xdr::ScVal::Vec(Some(xdr::ScVec(
                    [xdr::ScVal::Symbol(xdr::ScSymbol(
                        "Admin".try_into().unwrap(),
                    ))]
                    .try_into()
                    .unwrap(),
                )));
                let val = xdr::ScVal::try_from(admin).unwrap();
                storage.put(
                    &Rc::new(xdr::LedgerKey::ContractData(xdr::LedgerKeyContractData {
                        contract_id: Hash(token_id.to_array()),
                        key: key.clone(),
                    })),
                    &Rc::new(xdr::LedgerEntry {
                        last_modified_ledger_seq: 0,
                        data: xdr::LedgerEntryData::ContractData(xdr::ContractDataEntry {
                            contract_id: Hash(token_id.to_array()),
                            key,
                            val,
                        }),
                        ext: xdr::LedgerEntryExt::V0,
                    }),
                    soroban_env_host::budget::AsBudget::as_budget(self.host()),
                )?;
                Ok(())
            })
            .unwrap();

        Address::from_contract_id(&token_id)
    }

    fn register_contract_with_optional_contract_id_and_executable<'a>(
        &self,
        contract_id: impl Into<Option<&'a Address>>,
        executable: xdr::ScContractExecutable,
    ) -> Address {
        if let Some(contract_id) = contract_id.into() {
            self.register_contract_with_contract_id_and_executable(contract_id, executable);
            contract_id.clone()
        } else {
            self.register_contract_with_source(executable)
        }
    }

    fn register_contract_with_source(&self, executable: xdr::ScContractExecutable) -> Address {
        let prev_source_account = self.env_impl.source_account();
        self.env_impl
            .set_source_account(xdr::AccountId(xdr::PublicKey::PublicKeyTypeEd25519(
                xdr::Uint256(random()),
            )));

        let contract_id: BytesN<32> = self
            .env_impl
            .invoke_functions(vec![xdr::HostFunction {
                args: xdr::HostFunctionArgs::CreateContract(xdr::CreateContractArgs {
                    contract_id: xdr::ContractId::SourceAccount(xdr::Uint256(random())),
                    executable,
                }),
                auth: Default::default(),
            }])
            .unwrap()[0]
            .try_into_val(self)
            .unwrap();

        if let Some(prev_acc) = prev_source_account {
            self.env_impl.set_source_account(prev_acc);
        } else {
            self.env_impl.remove_source_account();
        }

        Address::from_contract_id(&contract_id)
    }

    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke [`Address::require_auth`] or
    /// [`Address::require_auth_for_args`] functions.
    ///
    /// This function can also be called on contract clients.
    ///
    /// To mock auth for testing, use [`mock_all_auths`][Self::mock_all_auths]
    /// or [`mock_auths`][Self::mock_auths]. If mocking of auths is enabled,
    /// calling [`set_auths`][Self::set_auths] disables any mocking.
    pub fn set_auths(&self, auths: &[ContractAuth]) {
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
    /// use soroban_sdk::{contractimpl, Env, Address, testutils::{Address as _, MockAuth, MockAuthInvoke}, IntoVal};
    ///
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
    ///             nonce: 0,
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
    /// use soroban_sdk::{contractimpl, Env, Address, testutils::Address as _};
    ///
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
        self.env_impl.switch_to_recording_auth();
    }

    /// Returns a list of authorizations that were seen during the last contract
    /// invocation.
    ///
    /// Use this in tests to verify that the expected authorizations with the
    /// expected arguments are required.
    ///
    /// The return value is a vector of authorizations represented by tuples of
    /// `(address, contract_id, function_name, args)` corresponding to the calls
    /// of `require_auth_for_args(address, args)` from the contract function
    /// `(contract_id, function_name)` (or `require_auth` with all the arguments
    /// of the function invocation).
    ///
    /// The order of the returned vector is defined by the order of
    /// [`Address::require_auth`] calls. Repeated calls to
    /// [`Address::require_auth`] with the same address and args in the same
    /// tree of contract invocations will appear only once in the vector. Calls
    /// to [`Address::require_auth`] in disjoint call trees for the same address
    /// will present in the list.
    ///
    /// ### Examples
    /// ```
    /// use soroban_sdk::{contractimpl, testutils::Address as _, Address, Symbol, Env, IntoVal};
    ///
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
    ///             client.contract.clone(),
    ///             Symbol::short("transfer"),
    ///             (&address, 1000_i128,).into_val(&env)
    ///         )]
    ///     );
    ///
    ///     client.transfer2(&address, &1000_i128);
    ///     assert_eq!(
    ///         env.auths(),
    ///         std::vec![(
    ///             address.clone(),
    ///             client.contract.clone(),
    ///             Symbol::short("transfer2"),
    ///             // `transfer2` requires auth for (amount / 2) == (1000 / 2) == 500.
    ///             (500_i128,).into_val(&env)
    ///         )]
    ///     );
    /// }
    /// # #[cfg(not(feature = "testutils"))]
    /// # fn main() { }
    /// ```
    pub fn auths(&self) -> std::vec::Vec<(Address, Address, crate::Symbol, Vec<RawVal>)> {
        use xdr::{ScBytes, ScVal};
        let authorizations = self.env_impl.get_authenticated_authorizations().unwrap();
        authorizations
            .iter()
            .map(|a| {
                let mut args = Vec::new(self);
                for v in a.3.iter() {
                    args.push_back(RawVal::try_from_val(self, v).unwrap());
                }
                (
                    Address::try_from_val(self, &ScVal::Address(a.0.clone())).unwrap(),
                    Address::from_contract_id(
                        &BytesN::<32>::try_from_val(
                            self,
                            &ScVal::Bytes(ScBytes(a.1.as_slice().to_vec().try_into().unwrap())),
                        )
                        .unwrap(),
                    ),
                    crate::Symbol::try_from_val(self, &a.2).unwrap(),
                    args,
                )
            })
            .collect()
    }

    fn register_contract_with_contract_id_and_executable(
        &self,
        contract_id: &Address,
        executable: xdr::ScContractExecutable,
    ) {
        let contract_id_hash = Hash(contract_id.contract_id().into());
        let data_key = xdr::ScVal::LedgerKeyContractExecutable;
        let key = Rc::new(LedgerKey::ContractData(LedgerKeyContractData {
            contract_id: contract_id_hash.clone(),
            key: data_key.clone(),
        }));
        let entry = Rc::new(LedgerEntry {
            ext: xdr::LedgerEntryExt::V0,
            last_modified_ledger_seq: 0,
            data: xdr::LedgerEntryData::ContractData(xdr::ContractDataEntry {
                contract_id: contract_id_hash.clone(),
                key: data_key,
                val: xdr::ScVal::ContractExecutable(executable),
            }),
        });
        self.env_impl
            .with_mut_storage(|storage| storage.put(&key, &entry, &self.env_impl.budget_cloned()))
            .unwrap();
    }

    /// Install the contract WASM code to the [Env] for testing.
    ///
    /// Returns the hash of the installed code that can be then used for
    /// the contract deployment.
    ///
    /// Useful for contract factory testing, otherwise use
    /// `register_contract_wasm` function that installs and deploys the contract
    /// in a single call.
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
    ///     env.install_contract_wasm(WASM);
    /// }
    /// ```
    pub fn install_contract_wasm(&self, contract_wasm: &[u8]) -> BytesN<32> {
        self.env_impl
            .invoke_functions(vec![xdr::HostFunction {
                args: xdr::HostFunctionArgs::UploadContractWasm(xdr::UploadContractWasmArgs {
                    code: contract_wasm.try_into().unwrap(),
                }),
                auth: Default::default(),
            }])
            .unwrap()[0]
            .try_into_val(self)
            .unwrap()
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
        env_impl.switch_to_recording_auth();

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
        self.env_impl.with_budget(|b| Budget::new(b))
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

    fn as_mut_any(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn check_same_env(&self, other: &Self) {
        self.env_impl.check_same_env(&other.env_impl);
    }

    fn deep_clone(&self) -> Self {
        Env {
            env_impl: self.env_impl.deep_clone(),
            #[cfg(any(test, feature = "testutils"))]
            snapshot: self.snapshot.clone(),
        }
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

    fn log_static_fmt_val(&self, fmt: &'static str, v: RawVal) -> Result<(), Self::Error> {
        Ok(self.env_impl.log_static_fmt_val(fmt, v).unwrap_optimized())
    }

    fn log_static_fmt_static_str(
        &self,
        fmt: &'static str,
        s: &'static str,
    ) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .log_static_fmt_static_str(fmt, s)
            .unwrap_optimized())
    }

    fn log_static_fmt_val_static_str(
        &self,
        fmt: &'static str,
        v: RawVal,
        s: &'static str,
    ) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .log_static_fmt_val_static_str(fmt, v, s)
            .unwrap_optimized())
    }

    fn log_static_fmt_general(
        &self,
        fmt: &'static str,
        v: &[RawVal],
        s: &[&'static str],
    ) -> Result<(), Self::Error> {
        Ok(self
            .env_impl
            .log_static_fmt_general(fmt, v, s)
            .unwrap_optimized())
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

    fn map_new_from_slices(
        &self,
        keys: &[&str],
        vals: &[RawVal],
    ) -> Result<MapObject, Self::Error> {
        Ok(self
            .env_impl
            .map_new_from_slices(keys, vals)
            .unwrap_optimized())
    }

    fn map_unpack_to_slice(
        &self,
        map: MapObject,
        keys: &[&str],
        vals: &mut [RawVal],
    ) -> Result<Void, Self::Error> {
        Ok(self
            .env_impl
            .map_unpack_to_slice(map, keys, vals)
            .unwrap_optimized())
    }

    fn vec_new_from_slice(&self, vals: &[RawVal]) -> Result<VecObject, Self::Error> {
        Ok(self.env_impl.vec_new_from_slice(vals).unwrap_optimized())
    }

    fn vec_unpack_to_slice(
        &self,
        vec: VecObject,
        vals: &mut [RawVal],
    ) -> Result<Void, Self::Error> {
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
