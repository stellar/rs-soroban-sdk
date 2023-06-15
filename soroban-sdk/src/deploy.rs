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
//! ```
//! # use soroban_sdk::{contractimpl, BytesN, Env, Symbol};
//! #
//! # pub struct Contract;
//! #
//! # #[contractimpl]
//! # impl Contract {
//! #     pub fn f(env: Env, wasm_hash: BytesN<32>) {
//! #         let salt = [0u8; 32];
//! #         let deployer = env.deployer().with_current_contract(salt);
//! #         let contract_address = deployer.deploy(wasm_hash);
//! #     }
//! # }
//! #
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//! #     let env = Env::default();
//! #     let contract_address = env.register_contract(None, Contract);
//! #     // Install the contract code before deploying its instance.
//! #     let mock_wasm = [0u8; 100];
//! #     let wasm_hash = env.deployer().upload_contract_wasm(mock_wasm.as_slice());
//! #     ContractClient::new(&env, &contract_address).f(&wasm_hash);
//! # }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```

use crate::{
    env::internal::Env as _, unwrap::UnwrapInfallible, Address, Bytes, BytesN, Env, IntoVal,
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
    /// to `stellar-xdr` create for the exact definition.
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
    #[doc(hidden)]
    /// Returns what the address of the contract will be once deployed.
    pub fn deployed_address(&self) -> Address {
        todo!()
    }

    /// Deploy a contract that uses Wasm executable with provided hash.
    ///
    /// The address of the deployed contract is defined by the deployer address
    /// and provided salt.
    ///
    /// Returns the deployed contract's address.
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
}

pub struct DeployerWithAsset {
    env: Env,
    serialized_asset: Bytes,
}

impl DeployerWithAsset {
    /// Return the address of the contract defined by the deployer.
    #[doc(hidden)]
    pub fn contract_address(&self) -> Address {
        todo!()
    }

    pub fn deploy(&self) -> Address {
        self.env
            .create_asset_contract(self.serialized_asset.to_object())
            .unwrap_infallible()
            .into_val(&self.env)
    }
}
