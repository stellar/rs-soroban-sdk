use crate::{
    deployer::{DeployToken, DeployWasm},
    env::internal::Env as _,
    Bytes, BytesN, Env, TryFromVal,
};

pub struct CurrentNamespace;

impl DeployWasm for CurrentNamespace {
    fn deploy_wasm(&self, env: &Env, salt: Bytes, wasm: Bytes) -> BytesN<32> {
        let id = env.create_contract_from_contract(wasm.to_object(), salt.to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
    }
}

impl DeployToken for CurrentNamespace {
    fn deploy_token(&self, env: &Env, salt: Bytes) -> BytesN<32> {
        let id = env.create_token_from_contract(salt.to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
    }
}

pub struct ContractNamespace {
    pub contract_id: BytesN<32>,
}

pub struct Ed25519Namespace {
    pub public_key: BytesN<32>,
}

pub enum IdNamespace {
    Current(CurrentNamespace),
    Contract(ContractNamespace),
    Ed25519(Ed25519Namespace),
}

impl From<CurrentNamespace> for IdNamespace {
    fn from(v: CurrentNamespace) -> Self {
        Self::Current(v)
    }
}

impl From<ContractNamespace> for IdNamespace {
    fn from(v: ContractNamespace) -> Self {
        Self::Contract(v)
    }
}

impl From<Ed25519Namespace> for IdNamespace {
    fn from(v: Ed25519Namespace) -> Self {
        Self::Ed25519(v)
    }
}

pub enum DeployerNamespace {
    Current(CurrentNamespace),
    Ed25519(Ed25519Namespace),
}

impl From<Ed25519Namespace> for DeployerNamespace {
    fn from(v: Ed25519Namespace) -> Self {
        Self::Ed25519(v)
    }
}

impl From<CurrentNamespace> for DeployerNamespace {
    fn from(v: CurrentNamespace) -> Self {
        Self::Current(v)
    }
}
