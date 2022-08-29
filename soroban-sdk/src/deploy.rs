//! Deploy provides types for deploying contracts.
//!
//! Contracts are assigned an ID that is derived from the namespace that the
//! contract was deployed with.
//!
//! - [CurrentNamespace] – A contract deployed by the currently executing
//! contract with [CurrentNamespace] will have an ID derived from the currently
//! executing contract's ID.
//!
//! - [ContractNamespace] – A contract deployed by some other contract will have
//! an ID derived from its contract ID.
//!
//! - [Ed25519Namespace] – A contract deployed by the currently executing
//! contract with the [Ed25519Namespace] will have an ID derived from its
//! ed25519 public key.
//!
//! The deployer for a namespace can be created using [Env::deployer].
//!
//! The contract ID for a deployed contract can be derived using [Env::contract_id].

use crate::{env::internal::Env as _, Bytes, BytesN, Env, TryFromVal};

/// Namespace of the currently executing contract.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CurrentNamespace {
    pub salt: Bytes,
}

/// Namespace of a specific contract ID.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ContractNamespace {
    pub contract_id: BytesN<32>,
    pub salt: Bytes,
}

/// Namespace of an ed25519 public key.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ed25519Namespace {
    pub public_key: BytesN<32>,
    pub salt: Bytes,
}

/// Deployer is a [Namespace] that the currently executing contract can deploy
/// contracts for.
///
/// The currently executing contract can deploy contracts with two namespaces:
/// - [CurrentNamespace]
/// - [Ed25519Namespace]
pub trait Deployer<D> {
    fn deployer(&self, env: &Env) -> D;
}

impl Deployer<CurrentNamespaceDeployer> for CurrentNamespace {
    fn deployer(&self, env: &Env) -> CurrentNamespaceDeployer {
        CurrentNamespaceDeployer {
            env: env.clone(),
            namespace: self.clone(),
        }
    }
}

impl Deployer<CurrentNamespaceDeployer> for &CurrentNamespace {
    fn deployer(&self, env: &Env) -> CurrentNamespaceDeployer {
        (*self).deployer(env)
    }
}

impl Deployer<Ed25519NamespaceDeployer> for Ed25519Namespace {
    fn deployer(&self, env: &Env) -> Ed25519NamespaceDeployer {
        Ed25519NamespaceDeployer {
            env: env.clone(),
            namespace: self.clone(),
        }
    }
}

impl Deployer<Ed25519NamespaceDeployer> for &Ed25519Namespace {
    fn deployer(&self, env: &Env) -> Ed25519NamespaceDeployer {
        (*self).deployer(env)
    }
}

/// Provides functionality for deploying a contract with a contract ID derived
/// from the [CurrentNamespace].
pub struct CurrentNamespaceDeployer {
    env: Env,
    namespace: CurrentNamespace,
}

impl CurrentNamespaceDeployer {
    pub(crate) fn new(env: &Env, namespace: CurrentNamespace) -> Self {
        Self {
            env: env.clone(),
            namespace,
        }
    }

    /// Return the ID of the contract defined by the deployer.
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
        let id = env.create_contract_from_contract(
            wasm.into().to_object(),
            self.namespace.salt.to_object(),
        );
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
        let id = env.create_token_from_contract(self.namespace.salt.to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
    }
}

/// Provides functionality for deploying a contract with a contract ID derived
/// from a [Ed25519Namespace].
pub struct Ed25519NamespaceDeployer {
    env: Env,
    namespace: Ed25519Namespace,
}

impl Ed25519NamespaceDeployer {
    pub(crate) fn new(env: &Env, namespace: Ed25519Namespace) -> Self {
        Self {
            env: env.clone(),
            namespace,
        }
    }

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
            self.namespace.salt.to_object(),
            self.namespace.public_key.to_object(),
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
            self.namespace.salt.to_object(),
            self.namespace.public_key.to_object(),
            signature.into().to_object(),
        );
        BytesN::try_from_val(env, id).unwrap()
    }
}
