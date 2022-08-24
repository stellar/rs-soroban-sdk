use soroban_sdk::{contracttype, Bytes, BytesN, Env, RawVal, Symbol, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Ed25519Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

#[derive(Clone)]
#[contracttype]
pub struct AccountSignatures {
    pub account_id: BytesN<32>,
    pub signatures: Vec<Ed25519Signature>,
}

#[derive(Clone)]
#[contracttype]
pub enum Signature {
    Contract,
    Ed25519(Ed25519Signature),
    Account(AccountSignatures),
}

impl Signature {
    pub fn get_identifier(&self, env: &Env) -> Identifier {
        match self {
            Signature::Contract => Identifier::Contract(env.get_invoking_contract()),
            Signature::Ed25519(kea) => Identifier::Ed25519(kea.public_key.clone()),
            Signature::Account(kaa) => Identifier::Account(kaa.account_id.clone()),
        }
    }
}

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum Identifier {
    Contract(BytesN<32>),
    Ed25519(BytesN<32>),
    Account(BytesN<32>),
}

#[derive(Clone)]
#[contracttype]
pub struct MessageV0 {
    pub function: Symbol,
    pub contrct_id: BytesN<32>,
    pub network_id: Bytes,
    pub args: Vec<RawVal>,
}

#[derive(Clone)]
#[contracttype]
pub enum Message {
    V0(MessageV0),
}
