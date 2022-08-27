use crate::{
    env::internal::Env as _, namespace::DeployerNamespace, Bytes, BytesN, Env, TryFromVal,
};

pub trait DeployWasm {
    fn deploy_wasm(&self, env: &Env, salt: Bytes, wasm: Bytes) -> BytesN<32>;
}

pub trait DeployToken {
    fn deploy_token(&self, env: &Env, salt: Bytes) -> BytesN<32>;
}

pub struct Deployer {
    pub env: Env,
    pub namespace: DeployerNamespace,
}

impl Deployer {
    pub(crate) fn new(env: Env, namespace: DeployerNamespace) -> Self {
        Self { env, namespace }
    }

    pub fn deploy_wasm(&self, salt: impl Into<Bytes>, wasm: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        match self.namespace {
            DeployerNamespace::Current(c) => c.deploy_wasm(env, salt.into(), wasm.into()),
            DeployerNamespace::Ed25519(ed25519) => {
                todo!()
            }
        }
    }

    pub fn deploy_token(&self, salt: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        match self.namespace {
            DeployerNamespace::Current(c) => c.deploy_token(env, salt.into()),
            DeployerNamespace::Ed25519(ed25519) => {
                todo!()
            }
        }
    }
}
