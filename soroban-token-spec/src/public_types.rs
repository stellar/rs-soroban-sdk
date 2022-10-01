use soroban_sdk::{accounts::AccountId, contracttype, Bytes, BytesN};

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: Bytes,
    pub symbol: Bytes,
    pub decimals: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct AlphaNum4Metadata {
    pub asset_code: BytesN<4>,
    pub issuer: AccountId,
}

#[derive(Clone)]
#[contracttype]
pub struct AlphaNum12Metadata {
    pub asset_code: BytesN<12>,
    pub issuer: AccountId,
}
