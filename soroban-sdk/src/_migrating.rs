//! # Migrating from v22 to v23
//! 
//! 1. The [`Events::publish`] method is deprecated in favor of `#[contractevent]`
//!    macro.
//!
//!    `#[contractevent]` macro provides a more convenient and type-safe way to
//!    define and publish events. It is recommended to migrate the existing
//!    events to use `#[contractevent]`.
//!   
//!    For example, consider the following event publish code:
//!   
//!    ```
//!    // ... inside some function ...
//!    // When counter is incremented, publish an event with topics 'counter'
//!    // and 'increment', and data `count`.
//!    env.events().publish((symbol_short!("counter"), symbol_short!("increment")),
//!                          count);
//!    // When counter is decremented, publish an event with topics 'counter'
//!    // and 'decrement', and data `count`.
//!    env.events().publish((symbol_short!("counter"), symbol_short!("decrement")),
//!                          count);
//!    ```
//!    
//!    This can be replaced with the following code using `#[contractevent]`:
//!
//!    ```
//!    use soroban_sdk::contractevent;
//!    // The event will always have the first topic 'counter'.
//!    #[contractevent(topics = ["counter"])]
//!    pub struct CounterEvent {
//!      // The second topic can be customized to reflect the type of counter
//!      // change.
//!      #[topic]
//!      counter_change: Symbol,
//!      count: u32,
//!    }
//!    
//!    // ... inside some function ...
//!    // When counter is incremented, publish an event with topics 'counter'
//!    // and 'increment', and data `count`.
//!    CounterEvent {
//!        counter_change: symbol_short!("increment"),
//!        count,
//!    }.publish(&env);
//!    // When counter is decremented, publish an event with topics 'counter'
//!    // and 'increment', and data `count`.
//!    CounterEvent {
//!        counter_change: symbol_short!("decrement"),
//!        count,
//!    }.publish(&env);
//!    ```
//!   More examples of using `#[contractevent]` can be found in the event test
//!   module of the SDK (https://github.com/stellar/rs-soroban-sdk/blob/main/soroban-sdk/src/tests/contract_event.rs)
//!   
//! 2. Token interface has been updated to use [`MuxedAddress`] instead of
//!    [`Address`] for the transfer destination.
//!
//!    Note, that Stellar Asset contract supports the updated interface as well.
//!    
//!    `MuxedAddress` is a special type that is compatible with `Address` in
//!    most of the contexts. If a contract function accepts a `MuxedAddress`
//!    argument, then it can also accept an `Address` argument seamlessly, so
//!    the existing contracts or tests that interact with the updated token
//!    interface don't require any changes.
//!    
//!    The token implementations should be updated to use the new interface.
//!    `MuxedAddress` allows users to attach a 64-bit ID to the payment
//!    destination in order to identify a 'virtual' account, such as an exchange
//!    deposit account. Note, that this change is not sufficient to support
//!    the exchange deposits for the token contracts, but is necessary for that.
//!
//!    The necessary token modification is very minimal. Consider the following
//!    `transfer` implementation that still uses `Address` destination:
//!
//!    ```
//!    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
//!       // Authorize the transfer source.
//!       from.require_auth();
//!       // Token-specific implementation of balance movement.
//!       token_impl::move_balance(&env, &from, &to, amount);
//!       // Publish an event (notice that this uses the new event format - see
//!       // the previous migration step).
//!       soroban_sdk::token::Transfer {
//!           from,
//!           to,
//!           amount,
//!       }.publish(&env);
//!    }
//!    ```
//!    
//!    The updated implementation would look as follows:
//!
//!    ```
//!    fn transfer(env: Env, from: Address, muxed_to: MuxedAddress, amount: i128) {
//!       // Authorize the transfer source.
//!       from.require_auth();
//!       // Extract the underlying Address by dropping the ID.
//!       let to = muxed_to.address();
//!       // Token-specific implementation of balance movement (same as before).
//!       token_impl::move_balance(&env, &from, &to, amount);
//!       // Publish an event (notice that this uses the new event format - see
//!       // the previous migration step).
//!       soroban_sdk::token::TransferMuxed { // 👈 👀 New event kind for supporting the muxed destination
//!           from,
//!           to,
//!           to_muxed_id: muxed_to.id(), // 👈 👀 New field to capture the ID.
//!           amount,
//!       }.publish(&env);
//!    }
//!    ```
//!   
//!    That's the extent of the necessary changes. Note, that the transfer 
//!    events produced in different contexts (e.g. in case of `transfer_from`)
//!    don't require any changes, because the [`MuxedAddress`] is only necessary
//!    in a few narrow scenarios (such as direct transfer to an exchange).
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
