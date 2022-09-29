use soroban_sdk::{contracttype, AccountId, Bytes, BytesN};

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

#[derive(Clone)]
#[contracttype]
pub enum ClassicMetadata {
    Native,
    AlphaNum4(AlphaNum4Metadata),
    AlphaNum12(AlphaNum12Metadata),
}

#[derive(Clone)]
#[contracttype]
pub enum Metadata {
    Token(TokenMetadata),
    Native,
    AlphaNum4(AlphaNum4Metadata),
    AlphaNum12(AlphaNum12Metadata),
}
