use crate::BytesN;

/// Namespace defines the namespace that other contracts IDs are defined with.
///
/// A contract deployed by the current contract will have an ID derived from the
/// a [CurrentNamespace], which is equilavent to a [ContractNamespace] with its
/// `contract_id` set to the currently executing contract's ID.
///
/// A contract deployed with an ed25519 public key and signature will have an ID
/// derived from a [Ed25519Namespace].
#[doc(hidden)]
pub enum Namespace {
    Current(CurrentNamespace),
    Contract(ContractNamespace),
    Ed25519(Ed25519Namespace),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CurrentNamespace;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ContractNamespace {
    pub contract_id: BytesN<32>,
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ed25519Namespace {
    pub public_key: BytesN<32>,
}

impl From<CurrentNamespace> for Namespace {
    fn from(v: CurrentNamespace) -> Self {
        Self::Current(v)
    }
}

impl From<&CurrentNamespace> for Namespace {
    fn from(v: &CurrentNamespace) -> Self {
        Self::Current(*v)
    }
}

impl From<ContractNamespace> for Namespace {
    fn from(v: ContractNamespace) -> Self {
        Self::Contract(v)
    }
}

impl From<&ContractNamespace> for Namespace {
    fn from(v: &ContractNamespace) -> Self {
        Self::Contract(v.clone())
    }
}

impl From<Ed25519Namespace> for Namespace {
    fn from(v: Ed25519Namespace) -> Self {
        Self::Ed25519(v)
    }
}

impl From<&Ed25519Namespace> for Namespace {
    fn from(v: &Ed25519Namespace) -> Self {
        Self::Ed25519(v.clone())
    }
}
