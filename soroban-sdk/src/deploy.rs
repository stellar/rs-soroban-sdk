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
//! # use soroban_sdk::{contractimpl, symbol, BytesN, Env, Symbol};
//! #
//! # pub struct Contract;
//! #
//! # #[contractimpl]
//! # impl Contract {
//! #     pub fn f(env: Env, wasm_hash: BytesN<32>) {
//! #         let salt = [0u8; 32];
//! let deployer = env.deployer().with_current_contract(&salt);
//! let contract_id = deployer.deploy(&wasm_hash);
//! #     }
//! # }
//! #
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//! #     let env = Env::default();
//! #     let contract_id = BytesN::from_array(&env, &[0; 32]);
//! #     env.register_contract(&contract_id, Contract);
//! #     // Install the contract code before deploying its instance.
//! #     let wasm_hash = env.install_contract_wasm(&[0u8; 100]);
//! #     ContractClient::new(&env, &contract_id).f(&wasm_hash);
//! # }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
use crate::{env::internal::Env as _, Bytes, BytesN, Env, IntoVal};

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
    /// from the current contract and the provided salt.
    pub fn with_current_contract(
        &self,
        salt: impl IntoVal<Env, Bytes>,
    ) -> DeployerWithCurrentContract {
        let env = self.env();
        DeployerWithCurrentContract {
            env: env.clone(),
            salt: salt.into_val(env),
        }
    }

    #[doc(hidden)]
    /// Get a deployer for contracts that derive their contract IDs from the
    /// given contract ID and the provided salt.
    pub fn with_other_contract(
        &self,
        contract_id: impl IntoVal<Env, BytesN<32>>,
        salt: impl IntoVal<Env, Bytes>,
    ) -> DeployerWithOtherContract {
        let env = self.env();
        DeployerWithOtherContract {
            env: env.clone(),
            contract_id: contract_id.into_val(env),
            salt: salt.into_val(env),
        }
    }
}

/// A deployer that deploys a contract that has its ID derived from the current
/// contract ID and the provided salt.
pub struct DeployerWithCurrentContract {
    env: Env,
    salt: Bytes,
}

impl DeployerWithCurrentContract {
    /// Return the ID of the contract defined by the deployer.
    #[doc(hidden)]
    pub fn id(&self) -> BytesN<32> {
        todo!()
    }

    /// Deploy a contract.
    ///
    /// The contract ID from the currently executing contract and the given salt
    /// will be used to derive a contract ID for the deployed contract.
    ///
    /// Returns the deployed contract's ID.
    pub fn deploy(&self, wasm_hash: impl IntoVal<Env, BytesN<32>>) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_contract_from_contract(
            wasm_hash.into_val(env).to_object(),
            self.salt.to_object(),
        );
        unsafe { BytesN::<32>::unchecked_new(env.clone(), id) }
    }
}

#[doc(hidden)]
/// A deployer for contracts that derive their contract IDs from the
/// given contract ID and the provided salt.
///
/// This deployer is unable to actually deploy contracts because the currently
/// executing contract can only deploy contracts with IDs derived from its own
/// contract ID or an ed25519 public key.
pub struct DeployerWithOtherContract {
    env: Env,
    contract_id: BytesN<32>,
    salt: Bytes,
}

impl DeployerWithOtherContract {
    #[doc(hidden)]
    /// Return the ID of the contract defined by the deployer.
    pub fn id(&self) -> BytesN<32> {
        todo!()
    }
}
