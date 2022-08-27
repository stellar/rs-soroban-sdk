use crate::{
    env::internal::Env as _,
    namespace::{CurrentNamespace, Ed25519Namespace},
    Bytes, BytesN, Env, TryFromVal,
};

pub struct Deployer {
    env: Env,
    namespace: DeployerNamespace,
}

impl Deployer {
    pub(crate) fn new(env: &Env, namespace: DeployerNamespace) -> Self {
        Self {
            env: env.clone(),
            namespace,
        }
    }

    pub fn deploy_wasm(&self, salt: impl Into<Bytes>, wasm: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        match &self.namespace {
            DeployerNamespace::Current(c) => c.deploy_wasm(env, salt.into(), wasm.into()),
            DeployerNamespace::Ed25519(_ed25519) => todo!(),
        }
    }

    pub fn deploy_token(&self, salt: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        match &self.namespace {
            DeployerNamespace::Current(c) => c.deploy_token(env, salt.into()),
            DeployerNamespace::Ed25519(_ed25519) => todo!(),
        }
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

trait DeployWasm {
    fn deploy_wasm(&self, env: &Env, salt: Bytes, wasm: Bytes) -> BytesN<32>;
}

trait DeployToken {
    fn deploy_token(&self, env: &Env, salt: Bytes) -> BytesN<32>;
}

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
