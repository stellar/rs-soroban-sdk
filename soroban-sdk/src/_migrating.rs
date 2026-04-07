//! # Migrating from v25 to v26
//!
//! 1. Add support for [CAP-78: Host functions for performing limited TTL extensions](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0078.md).
//!    New `extend_ttl_with_limits` methods on [`Persistent`], [`Instance`], and
//!    [`Deployer`] provide bounded control over TTL extensions with `min_extension` and
//!    `max_extension` parameters.
//!
//! 2. Add support for [CAP-82: Checked 256-bit integer arithmetic host functions](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0082.md).
//!    New `checked_*` methods on [`U256`] and [`I256`] (`checked_add`, `checked_sub`,
//!    `checked_mul`, `checked_div`, `checked_pow`, `checked_rem_euclid`, `checked_shl`,
//!    `checked_shr`) return `Option` instead of panicking on overflow. Also adds 
//!    `min_value` and `max_value` methods on [`U256`] and [`I256`] to fetch the 
//!    value bounds of each type.
//!
//! 3. Add support for [CAP-80: Host functions for efficient ZK BN254 use cases](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0080.md).
//!    [`BN254`] gains scalar field arithmetic (`Fr` `Add`/`Sub`/`Mul` traits, `pow`, `inv`),
//!    multi-scalar multiplication (`g1_msm`), and curve validation (`g1_is_on_curve`).
//!    [`BLS12381`] gains `g1_is_on_curve` and `g2_is_on_curve`.
//!
//! 4. Add support for [CAP-79: Host functions for muxed address strkey conversions](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0079.md).
//!    New method [`MuxedAddress::to_strkey`] converts muxed addresses to Stellar strkey format.
//!
//! 5. Add support for [CAP-73: Allow SAC to create G-account balances](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0073.md).
//!    Update the [`StellarAssetInterface`] to include the new method [`StellarAssetInterface::trust`]. This method creates
//!    an unlimited trustline for the contract's asset, if applicable.
//!
//! [`Persistent`]: crate::storage::Persistent
//! [`Instance`]: crate::storage::Instance
//! [`Deployer`]: crate::deploy::Deployer
//! [`U256`]: crate::U256
//! [`I256`]: crate::I256
//! [`MuxedAddress::to_strkey`]: crate::MuxedAddress::to_strkey
//! [`StellarAssetInterface`]: crate::token::StellarAssetInterface
//! [`StellarAssetInterface::trust`]: crate::token::StellarAssetInterface::trust
//! [`BN254`]: crate::crypto::bn254
//! [`BLS12381`]: crate::crypto::bls12_381
//!
//! # Migrating from v23 to v25
//!
//! 1. [`Events::all()` return type changed from `Vec<(Address, Vec<Val>, Val)>` to `ContractEvents`][v25_event_testing].
//!    The new type supports the old comparison format, so most code will continue to work.
//!    New methods like `filter_by_contract()` and XDR comparison via `to_xdr()` are now available.
//!
//! 2. [BN254 (alt_bn128) elliptic curve support added][v25_bn254].
//!    Access via `env.crypto().bn254()` for G1/G2 point operations and pairing checks.
//!
//! 3. [Poseidon and Poseidon2 permutation functions added][v25_poseidon].
//!    Available via `CryptoHazmat` under the `hazmat-crypto` feature for advanced
//!    cryptographic use cases.
//!
//! 4. [`contracttrait` macro added for reusable contract interfaces][v25_contracttrait].
//!    Define traits with default implementations using `#[contracttrait]`, then implement them
//!    in contracts using `#[contractimpl(contracttrait)]`.
//!
//! 5. [Resource limit enforcement enabled by default in tests][v25_resource_limits].
//!    `Env::default()` now enforces Mainnet resource limits for contract invocations.
//!    Tests will fail if limits are exceeded. This provides early warning of contracts that
//!    may be too resource-heavy for Mainnet. If you see test failures after upgrading,
//!    use `env.cost_estimate().disable_resource_limits()` to opt-out while optimizing.
//!
//! [v25_contracttrait]: v25_contracttrait
//! [v25_resource_limits]: v25_resource_limits
//!
//! # Migrating from v22 to v23
//!
//! 1. [`contractevent` replaces `Events::publish`][v23_contractevent]
//!   
//! 2. [`MuxedAddress` replaces `Address` as the `to` of the `TokenInterface::transfer`].
//!    This change concerns `soroban-token-sdk` and is documented in detail in
//!    `soroban-token-sdk` crate migration guide.
//!
//! 3. [Accessing archived persistent entries in tests no longer results in a panic][v23_archived_testing],
//!    automatic restoration is emulated instead. Note, that instance storage is a
//!    persistent entry as well.
//!
//! # Migrating from v21 to v22
//!
//! 1. [`Env::register`] and [`Env::register_at`] replace [`Env::register_contract`] and [`Env::register_contract_wasm`].
//!
//!    [`register`] registers both native contracts previously registered with
//!    [`register_contract`] and Wasm contracts previously registered with
//!    [`register_contract_wasm`]. It accepts a tuple that is passed to the
//!    contracts constructor. Pass `()` if the contract has no constructor.
//!
//!    ```
//!    use soroban_sdk::{contract, contractimpl, Env};
//!
//!    #[contract]
//!    pub struct Contract;
//!
//!    #[contractimpl]
//!    impl Contract {
//!        // ..
//!    }
//!
//!    #[test]
//!    fn test() {
//!    # }
//!    # #[cfg(feature = "testutils")]
//!    # fn main() {
//!        let env = Env::default();
//!        let address = env.register(
//!            Contract,  // 👈 👀 The contract being registered, or a Wasm `&[u8]`.
//!            (),        // 👈 👀 The constructor arguments, or ().
//!        );
//!        // ..
//!    }
//!    # #[cfg(not(feature = "testutils"))]
//!    # fn main() { }
//!    ```
//!
//!    [`register_at`] registers both native contracts previously registered
//!    with [`register_contract`] and Wasm contracts previously registered with
//!    [`register_contract_wasm`], and allows setting the address that the
//!    contract is registered at. It accepts a tuple that is passed to the
//!    contracts constructor. Pass `()` if the contract has no constructor.
//!
//!    ```
//!    use soroban_sdk::{contract, contractimpl, Env, Address, testutils::Address as _};
//!
//!    #[contract]
//!    pub struct Contract;
//!
//!    #[contractimpl]
//!    impl Contract {
//!        // ..
//!    }
//!
//!    #[test]
//!    fn test() {
//!    # }
//!    # #[cfg(feature = "testutils")]
//!    # fn main() {
//!        let env = Env::default();
//!        let address = Address::generate(&env);
//!        env.register_at(
//!            &address,   // 👈 👀 The address to register the contract at.
//!            Contract,  // 👈 👀 The contract being registered, or a Wasm `&[u8]`.
//!            (),        // 👈 👀 The constructor arguments, or ().
//!        );
//!        // ..
//!    }
//!    # #[cfg(not(feature = "testutils"))]
//!    # fn main() { }
//!    ```
//!
//! 2. [`DeployerWithAddress::deploy_v2`] replaces [`DeployerWithAddress::deploy`].
//!
//!    [`deploy_v2`] is the same as [`deploy`], except it accepts a list of
//!    arguments to be passed to the contracts constructor that will be called
//!    when it is deployed. For deploying existing contracts that do not have
//!    constructors, pass `()`.
//!
//!    ```
//!    use soroban_sdk::{contract, contractimpl, BytesN, Env};
//!
//!    #[contract]
//!    pub struct Contract;
//!
//!    #[contractimpl]
//!    impl Contract {
//!        pub fn exec(env: Env, wasm_hash: BytesN<32>) {
//!            let salt = [0u8; 32];
//!            let deployer = env.deployer().with_current_contract(salt);
//!            // Pass `()` for contracts that have no contstructor, or have a
//!            // constructor and require no arguments. Pass arguments in a
//!            // tuple if any required.
//!            let contract_address = deployer.deploy_v2(wasm_hash, ());
//!        }
//!    }
//!
//!    #[test]
//!    fn test() {
//!    # }
//!    # #[cfg(feature = "testutils")]
//!    # fn main() {
//!        let env = Env::default();
//!        let contract_address = env.register(Contract, ());
//!        let contract = ContractClient::new(&env, &contract_address);
//!        // Upload the contract code before deploying its instance.
//!        const WASM: &[u8] = include_bytes!("../doctest_fixtures/contract.wasm");
//!        let wasm_hash = env.deployer().upload_contract_wasm(WASM);
//!        contract.exec(&wasm_hash);
//!    }
//!    # #[cfg(not(feature = "testutils"))]
//!    # fn main() { }
//!    ```
//!
//! 2. Deprecated [`fuzz_catch_panic`]. Use [`Env::try_invoke_contract`] and the `try_` client functions instead.
//!
//!    The `fuzz_catch_panic` function could be used in fuzz tests to catch a contract panic. Improved behavior can be found by invoking a contract with the `try_` variant of the invoke function contract clients.
//!
//!    ```
//!    use libfuzzer_sys::fuzz_target;
//!    use soroban_sdk::{contract, contracterror, contractimpl, Env, testutils::arbitrary::*};
//!
//!    #[contract]
//!    pub struct Contract;
//!
//!    #[contracterror]
//!    #[derive(Debug, PartialEq)]
//!    pub enum Error {
//!        Overflow = 1,
//!    }
//!
//!    #[contractimpl]
//!    impl Contract {
//!        pub fn add(x: u32, y: u32) -> Result<u32, Error> {
//!            x.checked_add(y).ok_or(Error::Overflow)
//!        }
//!    }
//!
//!    #[derive(Arbitrary, Debug)]
//!    pub struct Input {
//!        pub x: u32,
//!        pub y: u32,
//!    }
//!
//!    fuzz_target!(|input: Input| {
//!        let env = Env::default();
//!        let id = env.register(Contract, ());
//!        let client = ContractClient::new(&env, &id);
//!
//!        let result = client.try_add(&input.x, &input.y);
//!        match result {
//!            // Returned if the function succeeds, and the value returned is
//!            // the type expected.
//!            Ok(Ok(_)) => {}
//!            // Returned if the function succeeds, and the value returned is
//!            // NOT the type expected.
//!            Ok(Err(_)) => panic!("unexpected type"),
//!            // Returned if the function fails, and the error returned is
//!            // recognised as part of the contract errors enum.
//!            Err(Ok(_)) => {}
//!            // Returned if the function fails, and the error returned is NOT
//!            // recognised, or the contract panic'd.
//!            Err(Err(_)) => panic!("unexpected error"),
//!        }
//!    });
//!
//!    # fn main() { }
//!    ```
//!
//! 3. Events in test snapshots are now reduced to only contract events and system events. Diagnostic events will no longer appear in test snapshots.
//!
//!    This will cause all test snapshot JSON files generated by the SDK to change when upgrading to this major version of the SDK. The change should be isolated to events and should omit only diagnostic events.
//!
//! [`Env::register`]: crate::Env::register
//! [`register`]: crate::Env::register
//! [`Env::register_at`]: crate::Env::register_at
//! [`register_at`]: crate::Env::register_at
//! [`Env::register_contract`]: crate::Env::register_contract
//! [`register_contract`]: crate::Env::register_contract
//! [`Env::register_contract_wasm`]: crate::Env::register_contract_wasm
//! [`register_contract_wasm`]: crate::Env::register_contract_wasm
//! [`DeployerWithAddress::deploy_v2`]: crate::deploy::DeployerWithAddress::deploy_v2
//! [`deploy_v2`]: crate::deploy::DeployerWithAddress::deploy_v2
//! [`DeployerWithAddress::deploy`]: crate::deploy::DeployerWithAddress::deploy
//! [`deploy`]: crate::deploy::DeployerWithAddress::deploy
//! [`fuzz_catch_panic`]: crate::testutils::arbitrary::fuzz_catch_panic
//! [`Env::try_invoke_contract`]: crate::Env::try_invoke_contract
//!
//! # Migrating from v20 to v21
//!
//! 1. [`CustomAccountInterface::__check_auth`] function `signature_payload` parameter changes from type [`BytesN<32>`] to [`Hash<32>`].
//!
//!    The two types are interchangeable. [`Hash<32>`] contains a [`BytesN<32>`] and can only be constructed in contexts where the value has been generated by a secure cryptographic function.
//!
//!    To convert from a [`Hash<32>`] to a [`BytesN<32>`], use [`Hash<32>::to_bytes`] or [`Into::into`].
//!
//!    Current implementations of the interface will see a build error, and should change [`BytesN<32>`] to [`Hash<32>`].
//!
//!    ```
//!    use soroban_sdk::{
//!        auth::{Context, CustomAccountInterface}, contract,
//!        contracterror, contractimpl, crypto::Hash, Env,
//!        Vec,
//!    };
//!
//!    #[contract]
//!    pub struct Contract;
//!
//!    #[contracterror]
//!    pub enum Error {
//!        AnError = 1,
//!        // ...
//!    }
//!
//!    #[contractimpl]
//!    impl CustomAccountInterface for Contract {
//!        type Signature = ();
//!        type Error = Error;
//!
//!        fn __check_auth(
//!            env: Env,
//!            signature_payload: Hash<32>, // 👈 👀
//!            signatures: (),
//!            auth_contexts: Vec<Context>,
//!        ) -> Result<(), Self::Error> {
//!            // ...
//!    #       todo!()
//!        }
//!    }
//!
//!    # fn main() { }
//!    ```
//!
//! [`CustomAccountInterface::__check_auth`]: crate::auth::CustomAccountInterface::__check_auth
//! [`BytesN<32>`]: crate::BytesN
//! [`Hash<32>`]: crate::crypto::Hash
//! [`Hash<32>::to_bytes`]: crate::crypto::Hash::to_bytes

pub mod v23_archived_testing;
pub mod v23_contractevent;
pub mod v25_bn254;
pub mod v25_contracttrait;
pub mod v25_event_testing;
pub mod v25_poseidon;
pub mod v25_resource_limits;
