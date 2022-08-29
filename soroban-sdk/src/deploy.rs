//! Deploy provides types for deploying contracts.
//!
//! Contracts are assigned an ID that is derived from a set of arguments. A
//! contract may choose which set of arguments to use to deploy with.  There are
//! three deployers that provide access to deployments with different arguments:
//!
//! - [CurrentDeployer] – A contract deployed by the currently executing
//! contract will have an ID derived from the currently executing contract's ID.
//!
//! - [Ed25519Deployer] – A contract deployed by the currently executing
//! contract with an ed25519 public key will have an ID derived from the ed25519
//! public key.
//!
//! The deployer for a namespace can be created using [Env::deployer].
//!
//! The contract ID for a deployed contract can be derived using [Env::contract_id].

use crate::{env::internal::Env as _, Bytes, BytesN, Env, TryFromVal};

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
    pub fn current(&self, salt: impl Into<Bytes>) -> CurrentDeployer {
        CurrentDeployer {
            env: self.env().clone(),
            salt: salt.into(),
        }
    }

    #[doc(hidden)]
    /// Get a deployer for contracts that derive their contract IDs from the
    /// give contract ID and the provided salt.
    pub fn contract(
        &self,
        contract_id: impl Into<BytesN<32>>,
        salt: impl Into<Bytes>,
    ) -> ContractDeployer {
        ContractDeployer {
            env: self.env().clone(),
            contract_id: contract_id.into(),
            salt: salt.into(),
        }
    }

    /// Get a deployer for contracts that derive their contract IDs from the
    /// give ed25519 public key and the provided salt.
    pub fn ed25519(
        &self,
        public_key: impl Into<BytesN<32>>,
        salt: impl Into<Bytes>,
    ) -> Ed25519Deployer {
        Ed25519Deployer {
            env: self.env().clone(),
            public_key: public_key.into(),
            salt: salt.into(),
        }
    }
}

/// A deployer that deploys contracts that derive their contract IDs from the
/// current contract and the provided salt.
pub struct CurrentDeployer {
    env: Env,
    salt: Bytes,
}

impl CurrentDeployer {
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
    pub fn deploy(&self, wasm: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_contract_from_contract(wasm.into().to_object(), self.salt.to_object());
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

/// A deployer for contracts that derive their contract IDs from the
/// give contract ID and the provided salt.
///
/// This deployer is unable to actually deploy contracts because the currently
/// executing contract can only deploy contracts with IDs derived from its own
/// contract ID or an ed25519 public key.
pub struct ContractDeployer {
    env: Env,
    contract_id: BytesN<32>,
    salt: Bytes,
}

impl ContractDeployer {
    #[doc(hidden)]
    /// Return the ID of the contract defined by the deployer.
    pub fn id(&self) -> BytesN<32> {
        todo!()
    }
}

/// A deployer for contracts that derive their contract IDs from the give
/// ed25519 public key and the provided salt.
pub struct Ed25519Deployer {
    env: Env,
    public_key: BytesN<32>,
    salt: Bytes,
}

impl Ed25519Deployer {
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
    pub fn deploy(&self, wasm: impl Into<Bytes>, signature: impl Into<BytesN<64>>) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_contract_from_ed25519(
            wasm.into().to_object(),
            self.salt.to_object(),
            self.public_key.to_object(),
            signature.into().to_object(),
        );
        BytesN::try_from_val(env, id).unwrap()
    }

    /// Deploy a built-in token contract.
    ///
    /// The ed25519 public key and the given salt will be used to derive a
    /// contract ID for the deployed contract.
    ///
    /// Returns the deployed contract's ID.
    pub fn deploy_token(&self, signature: impl Into<BytesN<64>>) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_token_from_ed25519(
            self.salt.to_object(),
            self.public_key.to_object(),
            signature.into().to_object(),
        );
        BytesN::try_from_val(env, id).unwrap()
    }
}
