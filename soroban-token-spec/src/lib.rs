#![no_std]
use core::panic;

use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, contracttype, BigInt, Bytes, Env};

mod tests;

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: Bytes,
    pub symbol: Bytes,
    pub decimals: u32,
}

/// The interface below was copied from
/// https://github.com/stellar/rs-soroban-env/blob/main/soroban-env-host/src/native_contract/token/contract.rs
/// at commit b3c188f48dec51a956c1380fb6fe92201a3f716b.
///
/// Differences between this interface and the built-in contract
/// 1. The return values here don't return Results.
/// 2. The implementations have been replaced with a panic.
/// 3. &Host type usage are replaced with Env
pub struct Token;

#[contractimpl]
#[allow(unused_variables)]
impl Token {
    /// Init creates a token contract that does not wrap an asset on the classic
    /// side. No checks are done on the contractID.
    pub fn init(env: Env, admin: Identifier, metadata: TokenMetadata) {
        panic!("calling into interface");
    }

    pub fn nonce(env: Env, id: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    pub fn allowance(env: Env, from: Identifier, spender: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    pub fn approve(env: Env, from: Signature, nonce: BigInt, spender: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    pub fn balance(env: Env, id: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    pub fn is_frozen(env: Env, id: Identifier) -> bool {
        panic!("calling into interface");
    }

    pub fn xfer(env: Env, from: Signature, nonce: BigInt, to: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    pub fn xfer_from(
        env: Env,
        spender: Signature,
        nonce: BigInt,
        from: Identifier,
        to: Identifier,
        amount: BigInt,
    ) {
        panic!("calling into interface");
    }

    pub fn burn(env: Env, admin: Signature, nonce: BigInt, from: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    pub fn freeze(env: Env, admin: Signature, nonce: BigInt, id: Identifier) {
        panic!("calling into interface");
    }

    pub fn mint(env: Env, admin: Signature, nonce: BigInt, to: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    pub fn set_admin(env: Env, admin: Signature, nonce: BigInt, new_admin: Identifier) {
        panic!("calling into interface");
    }

    pub fn unfreeze(env: Env, admin: Signature, nonce: BigInt, id: Identifier) {
        panic!("calling into interface");
    }

    pub fn decimals(env: Env) -> u32 {
        panic!("calling into interface");
    }

    pub fn name(env: Env) -> Bytes {
        panic!("calling into interface");
    }

    pub fn symbol(env: Env) -> Bytes {
        panic!("calling into interface");
    }

    pub fn import(env: Env, id: Signature, nonce: BigInt, amount: i64) {
        panic!("calling into interface");
    }

    pub fn export(env: Env, id: Signature, nonce: BigInt, amount: i64) {
        panic!("calling into interface");
    }
}

/// Obtains the full spec for the Token contract
pub const fn get_token_contract_spec_xdr() -> [u8; 2232] {
    const spec_xdr: [u8; 2232] = {
        let entries: &[&[u8]] = &[
            &Token::spec_xdr_allowance(),
            &Token::spec_xdr_approve(),
            &Token::spec_xdr_balance(),
            &Token::spec_xdr_burn(),
            &Token::spec_xdr_decimals(),
            &Token::spec_xdr_export(),
            &Token::spec_xdr_freeze(),
            &Token::spec_xdr_import(),
            &Token::spec_xdr_init(),
            &Token::spec_xdr_is_frozen(),
            &Token::spec_xdr_mint(),
            &Token::spec_xdr_name(),
            &Token::spec_xdr_nonce(),
            &Token::spec_xdr_set_admin(),
            &Token::spec_xdr_symbol(),
            &Token::spec_xdr_unfreeze(),
            &Token::spec_xdr_xfer(),
            &Token::spec_xdr_xfer_from(),
            &TokenMetadata::spec_xdr(),
            &soroban_auth::Identifier::spec_xdr(),
            &soroban_auth::Signature::spec_xdr(),
            &soroban_auth::Ed25519Signature::spec_xdr(),
            &soroban_auth::AccountSignatures::spec_xdr(),
            &soroban_auth::SignaturePayload::spec_xdr(),
            &soroban_auth::SignaturePayloadV0::spec_xdr(),
        ];

        let mut spec_xdr = [0u8; 2232];
        let mut next: usize = 0;

        let mut j: usize = 0;
        while j < entries.len() {
            let entry = entries[j];
            let mut k: usize = 0;
            while k < entry.len() {
                spec_xdr[next] = entry[k];
                next += 1;
                k += 1;
            }
            j += 1;
        }
        if next != spec_xdr.len() {
            panic!("the lenth of spec_xdr is too large")
        }

        spec_xdr
    };
    spec_xdr
}
