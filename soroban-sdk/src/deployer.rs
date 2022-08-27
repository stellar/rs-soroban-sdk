use crate::{
    env::internal::Env as _,
    namespace::{CurrentNamespace, Ed25519Namespace},
    Bytes, BytesN, Env, TryFromVal,
};

pub trait Deployer<D> {
    fn deployer(&self, env: &Env) -> D;
}

impl Deployer<CurrentNamespaceDeployer> for CurrentNamespace {
    fn deployer(&self, env: &Env) -> CurrentNamespaceDeployer {
        CurrentNamespaceDeployer {
            env: env.clone(),
            namespace: *self,
        }
    }
}

impl Deployer<CurrentNamespaceDeployer> for &CurrentNamespace {
    fn deployer(&self, env: &Env) -> CurrentNamespaceDeployer {
        CurrentNamespaceDeployer {
            env: env.clone(),
            namespace: **self,
        }
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
        Ed25519NamespaceDeployer {
            env: env.clone(),
            namespace: (*self).clone(),
        }
    }
}

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

    pub fn deploy_wasm(&self, salt: impl Into<Bytes>, wasm: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        let id =
            env.create_contract_from_contract(wasm.into().to_object(), salt.into().to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
    }

    pub fn deploy_token(&self, salt: impl Into<Bytes>) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_token_from_contract(salt.into().to_object());
        BytesN::<32>::try_from_val(env, id).unwrap()
    }
}

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

    pub fn deploy_wasm(
        &self,
        salt: impl Into<Bytes>,
        wasm: impl Into<Bytes>,
        signature: impl Into<BytesN<64>>,
    ) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_contract_from_ed25519(
            wasm.into().to_object(),
            salt.into().to_object(),
            self.namespace.public_key.to_object(),
            signature.into().to_object(),
        );
        BytesN::<32>::try_from_val(env, id).unwrap()
    }

    pub fn deploy_token(
        &self,
        salt: impl Into<Bytes>,
        signature: impl Into<BytesN<64>>,
    ) -> BytesN<32> {
        let env = &self.env;
        let id = env.create_token_from_ed25519(
            salt.into().to_object(),
            self.namespace.public_key.to_object(),
            signature.into().to_object(),
        );
        BytesN::<32>::try_from_val(env, id).unwrap()
    }
}
