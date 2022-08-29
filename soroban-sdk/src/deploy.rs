//! Deploy contains types for deploying contracts.
//!
//! Contracts are assigned an ID that is derived from a set of arguments. A
//! contract may choose which set of arguments to use to deploy with:
//!
//! - [Deployer::current] – A contract deployed by the currently executing
//! contract will have an ID derived from the currently executing contract's ID.
//!
//! - [Deployer::ed25519] – A contract deployed by the currently executing
//! contract with an ed25519 public key will have an ID derived from the ed25519
//! public key.
//!
//! The deployer can be created using [Env::deployer].

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
    pub fn current(&self, salt: impl Into<Bytes>) -> DeployerDerivedFromCurrentContract {
        DeployerDerivedFromCurrentContract {
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
    ) -> DeployerDerivedFromOtherContract {
        DeployerDerivedFromOtherContract {
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
    ) -> DeployerDerivedFromEd25519 {
        DeployerDerivedFromEd25519 {
            env: self.env().clone(),
            public_key: public_key.into(),
            salt: salt.into(),
        }
    }
}

/// A deployer that deploys a contract that has its ID derived from the current
/// contract ID and the provided salt.
pub struct DeployerDerivedFromCurrentContract {
    env: Env,
    salt: Bytes,
}

impl DeployerDerivedFromCurrentContract {
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

#[doc(hidden)]
/// A deployer for contracts that derive their contract IDs from the
/// give contract ID and the provided salt.
///
/// This deployer is unable to actually deploy contracts because the currently
/// executing contract can only deploy contracts with IDs derived from its own
/// contract ID or an ed25519 public key.
pub struct DeployerDerivedFromOtherContract {
    env: Env,
    contract_id: BytesN<32>,
    salt: Bytes,
}

impl DeployerDerivedFromOtherContract {
    #[doc(hidden)]
    /// Return the ID of the contract defined by the deployer.
    pub fn id(&self) -> BytesN<32> {
        todo!()
    }
}

/// A deployer that deploys a contract that has its ID derived from the given
/// ed25519 public key and the provided salt.
pub struct DeployerDerivedFromEd25519 {
    env: Env,
    public_key: BytesN<32>,
    salt: Bytes,
}

impl DeployerDerivedFromEd25519 {
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
