//! Deploy contains types for deploying contracts.
//!
//! Contracts are assigned an ID that is derived from a set of arguments. A
//! contract may choose which set of arguments to use to deploy with:
//!
//! - [Deployer::with_current_contract] – A contract deployed by the currently
//! executing contract will have an ID derived from the currently executing
//! contract's ID.
//!
//! The deployer can be created using [Env::deployer].
//!
//! ### Examples
//!
//! #### Deploy a contract without constructor (or 0-argument constructor)
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, BytesN, Env, Symbol};
//!
//! const DEPLOYED_WASM: &[u8] = include_bytes!("../doctest_fixtures/contract.wasm");
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn deploy(env: Env, wasm_hash: BytesN<32>) {
//!         let salt = [0u8; 32];
//!         let deployer = env.deployer().with_current_contract(salt);
//!         let contract_address = deployer.deploy_v2(wasm_hash, ());
//!         // ...
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_address = env.register(Contract, ());
//!     let contract = ContractClient::new(&env, &contract_address);
//!     // Upload the contract code before deploying its instance.
//!     let wasm_hash = env.deployer().upload_contract_wasm(DEPLOYED_WASM);
//!     contract.deploy(&wasm_hash);
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! #### Deploy a contract with a multi-argument constructor
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, BytesN, Env, Symbol, IntoVal};
//!
//! const DEPLOYED_WASM_WITH_CTOR: &[u8] = include_bytes!("../doctest_fixtures/contract_with_constructor.wasm");
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn deploy_with_constructor(env: Env, wasm_hash: BytesN<32>) {
//!         let salt = [1u8; 32];
//!         let deployer = env.deployer().with_current_contract(salt);
//!         let contract_address = deployer.deploy_v2(
//!              wasm_hash,
//!              (1_u32, 2_i64),
//!         );
//!         // ...
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_address = env.register(Contract, ());
//!     let contract = ContractClient::new(&env, &contract_address);
//!     // Upload the contract code before deploying its instance.
//!     let wasm_hash = env.deployer().upload_contract_wasm(DEPLOYED_WASM_WITH_CTOR);
//!     contract.deploy_with_constructor(&wasm_hash);
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
//!
//! #### Derive before deployment what the address of a contract will be
//!
//! ```
//! use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, IntoVal};
//!
//! #[contract]
//! pub struct Contract;
//!
//! #[contractimpl]
//! impl Contract {
//!     pub fn deploy_contract_address(env: Env) -> Address {
//!         let salt = [1u8; 32];
//!         let deployer = env.deployer().with_current_contract(salt);
//!         // Deployed contract address is deterministic and can be accessed
//!         // before deploying the contract. It is derived from the deployer
//!         // (the current contract's address) and the salt passed in above.
//!         deployer.deployed_address()
//!     }
//! }
//!
//! #[test]
//! fn test() {
//! # }
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//!     let env = Env::default();
//!     let contract_address = env.register(Contract, ());
//!     let contract = ContractClient::new(&env, &contract_address);
//!     assert_eq!(
//!         contract.deploy_contract_address(),
//!         Address::from_str(&env, "CBESJIMX7J53SWJGJ7WQ6QTLJI4S5LPPJNC2BNVD63GIKAYCDTDOO322"),
//!     );
//! }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```

use crate::{
    env::internal::Env as _, unwrap::UnwrapInfallible, Address, Bytes, BytesN, ConstructorArgs,
    Env, IntoVal,
};

/// Deployer provides access to deploying contracts.
pub struct Deployer {
    env: Env,
}

impl Deployer {
    pub(crate) fn new(env: &Env) -> Deployer {
        Deployer { env: env.clone() }
    }

    pub fn env(&self) -> &Env {
        &self.env
    }

    /// Get a deployer that deploys contract that derive the contract IDs
    /// from the current contract and provided salt.
    pub fn with_current_contract(
        &self,
        salt: impl IntoVal<Env, BytesN<32>>,
    ) -> DeployerWithAddress {
        DeployerWithAddress {
            env: self.env.clone(),
            address: self.env.current_contract_address(),
            salt: salt.into_val(&self.env),
        }
    }

    /// Get a deployer that deploys contracts that derive the contract ID
    /// from the provided address and salt.
    ///
    /// The deployer address must authorize all the deployments.
    pub fn with_address(
        &self,
        address: Address,
        salt: impl IntoVal<Env, BytesN<32>>,
    ) -> DeployerWithAddress {
        DeployerWithAddress {
            env: self.env.clone(),
            address,
            salt: salt.into_val(&self.env),
        }
    }

    /// Get a deployer that deploys an instance of Stellar Asset Contract
    /// corresponding to the provided serialized asset.
    ///
    /// `serialized_asset` is the Stellar `Asset` XDR serialized to bytes. Refer
    /// to `[soroban_sdk::xdr::Asset]`
    pub fn with_stellar_asset(
        &self,
        serialized_asset: impl IntoVal<Env, Bytes>,
    ) -> DeployerWithAsset {
        DeployerWithAsset {
            env: self.env.clone(),
            serialized_asset: serialized_asset.into_val(&self.env),
        }
    }

    /// Upload the contract Wasm code to the network.
    ///
    /// Returns the hash of the uploaded Wasm that can be then used for
    /// the contract deployment.
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
    ///     env.deployer().upload_contract_wasm(WASM);
    /// }
    /// ```
    pub fn upload_contract_wasm(&self, contract_wasm: impl IntoVal<Env, Bytes>) -> BytesN<32> {
        self.env
            .upload_wasm(contract_wasm.into_val(&self.env).to_object())
            .unwrap_infallible()
            .into_val(&self.env)
    }

    /// Replaces the executable of the current contract with the provided Wasm.
    ///
    /// The Wasm blob identified by the `wasm_hash` has to be already present
    /// in the ledger (uploaded via `[Deployer::upload_contract_wasm]`).
    ///
    /// The function won't do anything immediately. The contract executable
    /// will only be updated after the invocation has successfully finished.
    pub fn update_current_contract_wasm(&self, wasm_hash: impl IntoVal<Env, BytesN<32>>) {
        self.env
            .update_current_contract_wasm(wasm_hash.into_val(&self.env).to_object())
            .unwrap_infallible();
    }

    /// Extend the TTL of the contract instance and code.
    ///
    /// Extends the TTL of the instance and code only if the TTL for the provided contract is below `threshold` ledgers.
    /// The TTL will then become `extend_to`. Note that the `threshold` check and TTL extensions are done for both the
    /// contract code and contract instance, so it's possible that one is bumped but not the other depending on what the
    /// current TTL's are.
    ///
    /// The TTL is the number of ledgers between the current ledger and the final ledger the data can still be accessed.
    pub fn extend_ttl(&self, contract_address: Address, threshold: u32, extend_to: u32) {
        self.env
            .extend_contract_instance_and_code_ttl(
                contract_address.to_object(),
                threshold.into(),
                extend_to.into(),
            )
            .unwrap_infallible();
    }

    /// Extend the TTL of the contract instance.
    ///
    /// Same as [`extend_ttl`](Self::extend_ttl) but only for contract instance.
    pub fn extend_ttl_for_contract_instance(
        &self,
        contract_address: Address,
        threshold: u32,
        extend_to: u32,
    ) {
        self.env
            .extend_contract_instance_ttl(
                contract_address.to_object(),
                threshold.into(),
                extend_to.into(),
            )
            .unwrap_infallible();
    }

    /// Extend the TTL of the contract code.
    ///
    /// Same as [`extend_ttl`](Self::extend_ttl) but only for contract code.
    pub fn extend_ttl_for_code(&self, contract_address: Address, threshold: u32, extend_to: u32) {
        self.env
            .extend_contract_code_ttl(
                contract_address.to_object(),
                threshold.into(),
                extend_to.into(),
            )
            .unwrap_infallible();
    }
}

/// A deployer that deploys a contract that has its ID derived from the provided
/// address and salt.
pub struct DeployerWithAddress {
    env: Env,
    address: Address,
    salt: BytesN<32>,
}

impl DeployerWithAddress {
    /// Return the address of the contract defined by the deployer.
    ///
    /// This function can be called at anytime, before or after the contract is
    /// deployed, because contract addresses are deterministic.
    pub fn deployed_address(&self) -> Address {
        self.env
            .get_contract_id(self.address.to_object(), self.salt.to_object())
            .unwrap_infallible()
            .into_val(&self.env)
    }

    /// Deploy a contract that uses Wasm executable with provided hash.
    ///
    /// The address of the deployed contract is defined by the deployer address
    /// and provided salt.
    ///
    /// Returns the deployed contract's address.
    #[deprecated(note = "use deploy_v2")]
    pub fn deploy(&self, wasm_hash: impl IntoVal<Env, BytesN<32>>) -> Address {
        let env = &self.env;
        let address_obj = env
            .create_contract(
                self.address.to_object(),
                wasm_hash.into_val(env).to_object(),
                self.salt.to_object(),
            )
            .unwrap_infallible();
        unsafe { Address::unchecked_new(env.clone(), address_obj) }
    }

    /// Deploy a contract that uses Wasm executable with provided hash.
    ///
    /// The constructor args will be passed to the contract's constructor. Pass
    /// `()` for contract's with no constructor or a constructor with zero
    /// arguments.
    ///
    /// The address of the deployed contract is defined by the deployer address
    /// and provided salt.
    ///
    /// Returns the deployed contract's address.
    pub fn deploy_v2<A>(
        &self,
        wasm_hash: impl IntoVal<Env, BytesN<32>>,
        constructor_args: A,
    ) -> Address
    where
        A: ConstructorArgs,
    {
        let env = &self.env;
        let address_obj = env
            .create_contract_with_constructor(
                self.address.to_object(),
                wasm_hash.into_val(env).to_object(),
                self.salt.to_object(),
                constructor_args.into_val(env).to_object(),
            )
            .unwrap_infallible();
        unsafe { Address::unchecked_new(env.clone(), address_obj) }
    }
}

pub struct DeployerWithAsset {
    env: Env,
    serialized_asset: Bytes,
}

impl DeployerWithAsset {
    /// Return the address of the contract defined by the deployer.
    ///
    /// This function can be called at anytime, before or after the contract is
    /// deployed, because contract addresses are deterministic.
    pub fn deployed_address(&self) -> Address {
        self.env
            .get_asset_contract_id(self.serialized_asset.to_object())
            .unwrap_infallible()
            .into_val(&self.env)
    }

    pub fn deploy(&self) -> Address {
        self.env
            .create_asset_contract(self.serialized_asset.to_object())
            .unwrap_infallible()
            .into_val(&self.env)
    }
}

#[cfg(any(test, feature = "testutils"))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
mod testutils {
    use crate::deploy::Deployer;
    use crate::Address;

    impl crate::testutils::Deployer for Deployer {
        fn get_contract_instance_ttl(&self, contract: &Address) -> u32 {
            self.env
                .host()
                .get_contract_instance_live_until_ledger(contract.to_object())
                .unwrap()
                .checked_sub(self.env.ledger().sequence())
                .unwrap()
        }

        fn get_contract_code_ttl(&self, contract: &Address) -> u32 {
            self.env
                .host()
                .get_contract_code_live_until_ledger(contract.to_object())
                .unwrap()
                .checked_sub(self.env.ledger().sequence())
                .unwrap()
        }
    }
}
