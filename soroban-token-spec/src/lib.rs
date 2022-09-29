#![no_std]
#![allow(unused_variables)]
use soroban_sdk::{contractimpl, BigInt, Bytes, Env};

use soroban_auth::{Identifier, Signature};

mod public_types;
use crate::public_types::{ClassicMetadata, TokenMetadata};

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
impl Token {
    /// init_wrap can create a contract for a wrapped classic asset
    /// (Native, AlphaNum4, or AlphaNum12). It will fail if the contractID
    /// of this contract does not match the expected contractID for this asset
    /// returned by Host::get_contract_id_from_asset. This function should only be
    /// called by the create_token_wrapper host function for this reason.
    ///
    /// No admin will be set for the Native token, so any function that checks the admin
    /// (burn, freeze, unfreeze, mint, set_admin) will always fail
    pub fn init_wrap(env: Env, metadata: ClassicMetadata) {
        panic!("calling into interface");
    }

    /// init_token creates a token contract that does not wrap an asset on the classic side.
    /// No checks are done on the contractID.
    pub fn init_token(env: Env, admin: Identifier, metadata: TokenMetadata) {
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

    pub fn to_smart(env: Env, id: Signature, nonce: BigInt, amount: i64) {
        panic!("calling into interface");
    }

    pub fn to_classic(env: Env, id: Signature, nonce: BigInt, amount: i64) {
        panic!("calling into interface");
    }
}
