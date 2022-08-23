use soroban_sdk::{contracttype, Bytes, BytesN, Env, EnvVal, Symbol, Vec};

#[derive(Clone)]
#[contracttype]
pub struct KeyedEd25519Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

pub type AccountAuthorization = Vec<KeyedEd25519Signature>;

#[derive(Clone)]
#[contracttype]
pub struct KeyedAccountAuthorization {
    pub public_key: BytesN<32>,
    pub signatures: AccountAuthorization,
}

#[derive(Clone)]
#[contracttype]
pub enum KeyedAuthorization {
    Contract,
    Ed25519(KeyedEd25519Signature),
    Account(KeyedAccountAuthorization),
}

impl KeyedAuthorization {
    pub fn get_identifier(&self, env: &Env) -> Identifier {
        match self {
            KeyedAuthorization::Contract => Identifier::Contract(env.get_invoking_contract()),
            KeyedAuthorization::Ed25519(kea) => Identifier::Ed25519(kea.public_key.clone()),
            KeyedAuthorization::Account(kaa) => Identifier::Account(kaa.public_key.clone()),
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
    pub parameters: Vec<EnvVal>,
}

#[derive(Clone)]
#[contracttype]
pub enum Message {
    V0(MessageV0),
}
