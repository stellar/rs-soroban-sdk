//! Deploy contains types for deploying contracts.
//!
//! Contracts are assigned an ID that is derived from a set of arguments. A
//! contract may choose which set of arguments to use to deploy with:
//!
//! - [Deployer::with_current_contract] – A contract deployed by the currently
//! executing contract will have an ID derived from the currently executing
//! contract's ID.
//!
//! - [Deployer::with_ed25519] – A contract deployed by the currently executing
//! contract with an ed25519 public key will have an ID derived from the ed25519
//! public key.
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
//! #     pub fn f(env: Env) {
//! # let wasm = [0u8; 32];
//! # let salt = [0u8; 32];
//! let deployer = env.deployer().with_current_contract(&salt);
//! let contract_id = deployer.deploy(&wasm);
//! #     }
//! # }
//! #
//! # #[cfg(feature = "testutils")]
//! # fn main() {
//! #     let env = Env::default();
//! #     let contract_id = BytesN::from_array(&env, &[0; 32]);
//! #     env.register_contract(&contract_id, Contract);
//! #     ContractClient::new(&env, &contract_id).f();
//! # }
//! # #[cfg(not(feature = "testutils"))]
//! # fn main() { }
//! ```
use crate::{env::internal::Env as _, Bytes, BytesN, Env, IntoVal, TryFromVal};

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

    #[doc(hidden)]
    /// Get a deployer for contracts that derive their contract IDs from the
    /// given ed25519 public key and the provided salt.
    pub fn with_ed25519(
        &self,
        public_key: impl IntoVal<Env, BytesN<32>>,
        salt: impl IntoVal<Env, Bytes>,
    ) -> DeployerWithEd25519 {
        let env = self.env();
        DeployerWithEd25519 {
            env: env.clone(),
            public_key: public_key.into_val(env),
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
    pub fn deploy(&self, wasm: impl IntoVal<Env, Bytes>) -> BytesN<32> {
        let env = &self.env;
        let id = env
            .create_contract_from_contract(wasm.into_val(env).to_object(), self.salt.to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
    }

    /// Deploy a built-in token contract.
    ///
    /// The contract ID from the currently executing contract and the given salt
    /// will be used to derive a contract ID for the deployed contract.
    ///
    /// Returns the deployed contract's ID.
    pub fn deploy_token(&self) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_token_from_contract(self.salt.to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
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

#[doc(hidden)]
/// A deployer that deploys a contract that has its ID derived from the given
/// ed25519 public key and the provided salt.
pub struct DeployerWithEd25519 {
    env: Env,
    public_key: BytesN<32>,
    salt: Bytes,
}

impl DeployerWithEd25519 {
    #[doc(hidden)]
    /// Return the ID of the contract defined by the deployer.
    pub fn id(&self) -> BytesN<32> {
        todo!()
    }

    /// Deploy a contract.
    ///
    /// The ed25519 public key and the given salt will be used to derive a
    /// contract ID for the deployed contract.
    ///
    /// Returns the deployed contract's ID.
    pub fn deploy(
        &self,
        wasm: impl IntoVal<Env, Bytes>,
        signature: impl IntoVal<Env, BytesN<64>>,
    ) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_contract_from_ed25519(
            wasm.into_val(env).to_object(),
            self.salt.to_object(),
            self.public_key.to_object(),
            signature.into_val(env).to_object(),
        );
        BytesN::try_from_val(env, id).unwrap()
    }

    /// Deploy a built-in token contract.
    ///
    /// The ed25519 public key and the given salt will be used to derive a
    /// contract ID for the deployed contract.
    ///
    /// Returns the deployed contract's ID.
    pub fn deploy_token(&self, signature: impl IntoVal<Env, BytesN<64>>) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_token_from_ed25519(
            self.salt.to_object(),
            self.public_key.to_object(),
            signature.into_val(env).to_object(),
        );
        BytesN::try_from_val(env, id).unwrap()
    }
}
