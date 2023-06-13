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
//! let deployer = env.deployer().with_current_contract();
//! let contract_address = deployer.deploy(&salt, &wasm_hash);
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

    /// Get a deployer that deploys contracts that derive their contract IDs
    /// from the current contract.
    pub fn with_current_contract(&self) -> DeployerWithAddress {
        DeployerWithAddress {
            env: self.env.clone(),
            address: self.env.current_contract_address(),
        }
    }

    /// Get a deployer that deploys contracts that derive their contract IDs
    /// from the provided address.
    ///
    /// The deployer address must authorize all the deployments.
    pub fn with_address(&self, address: Address) -> DeployerWithAddress {
        DeployerWithAddress {
            env: self.env.clone(),
            address,
        }
    }

    /// Deploys a new instance of Stellar Asset Contract corresponding to
    /// provided serialized asset.
    ///
    /// `serialized_asset` is the Stellar `Asset` XDR serialized to bytes. Refer
    /// to `stellar-xdr` create for the exact definition.
    pub fn deploy_stellar_asset(&self, serialized_asset: impl IntoVal<Env, Bytes>) -> Address {
        self.env
            .create_asset_contract(serialized_asset.into_val(&self.env).to_object())
            .unwrap_infallible()
            .into_val(&self.env)
    }

    /// Upload the contract Wasm code to the network.
    ///
    /// Returns the hash of the uploaded Wasm that can be then used for
    /// the contract deployment.
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
}

impl DeployerWithAddress {
    /// Deploy a contract that uses Wasm executable with provided hash.
    ///
    /// The address of the deployed contract is defined by the deployer address
    /// and provided salt.
    ///
    /// Returns the deployed contract's address.
    pub fn deploy(
        &self,
        salt: &impl IntoVal<Env, BytesN<32>>,
        wasm_hash: &impl IntoVal<Env, BytesN<32>>,
    ) -> Address {
        let env = &self.env;
        let address_obj = env
            .create_contract(
                self.address.to_object(),
                wasm_hash.into_val(env).to_object(),
                salt.into_val(env).to_object(),
            )
            .unwrap_infallible();
        unsafe { Address::unchecked_new(env.clone(), address_obj) }
    }
}
