use crate::BytesN;

pub struct CurrentNamespace;

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
