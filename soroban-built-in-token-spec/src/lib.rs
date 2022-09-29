#![no_std]
#![allow(unused_variables)]
use soroban_sdk::{contractimpl, BigInt, Bytes, Env};

use soroban_auth::{Identifier, Signature};

mod public_types;
use crate::public_types::{ClassicMetadata, TokenMetadata};

pub trait TokenTrait {
    /// init_wrap can create a contract for a wrapped classic asset
    /// (Native, AlphaNum4, or AlphaNum12). It will fail if the contractID
    /// of this contract does not match the expected contractID for this asset
    /// returned by Host::get_contract_id_from_asset. This function should only be
    /// called by the create_token_wrapper host function for this reason.
    ///
    /// No admin will be set for the Native token, so any function that checks the admin
    /// (burn, freeze, unfreeze, mint, set_admin) will always fail
    fn init_wrap(env: Env, metadata: ClassicMetadata);

    /// init_token creates a token contract that does not wrap an asset on the classic side.
    /// No checks are done on the contractID.
    fn init_token(env: Env, admin: Identifier, metadata: TokenMetadata);

    fn nonce(env: Env, id: Identifier) -> BigInt;

    fn allowance(env: Env, from: Identifier, spender: Identifier) -> BigInt;

    fn approve(env: Env, from: Signature, nonce: BigInt, spender: Identifier, amount: BigInt);

    fn balance(env: Env, id: Identifier) -> BigInt;

    fn is_frozen(env: Env, id: Identifier) -> bool;

    fn xfer(env: Env, from: Signature, nonce: BigInt, to: Identifier, amount: BigInt);

    fn xfer_from(
        env: Env,
        spender: Signature,
        nonce: BigInt,
        from: Identifier,
        to: Identifier,
        amount: BigInt,
    );

    fn burn(env: Env, admin: Signature, nonce: BigInt, from: Identifier, amount: BigInt);

    fn freeze(env: Env, admin: Signature, nonce: BigInt, id: Identifier);

    fn mint(env: Env, admin: Signature, nonce: BigInt, to: Identifier, amount: BigInt);

    fn set_admin(env: Env, admin: Signature, nonce: BigInt, new_admin: Identifier);

    fn unfreeze(env: Env, admin: Signature, nonce: BigInt, id: Identifier);

    fn decimals(env: Env) -> u32;

    fn name(env: Env) -> Bytes;

    fn symbol(env: Env) -> Bytes;

    fn to_smart(env: Env, id: Signature, nonce: BigInt, amount: i64);

    fn to_classic(env: Env, id: Signature, nonce: BigInt, amount: i64);
}

pub struct Token;

#[contractimpl]
impl TokenTrait for Token {
    fn init_wrap(env: Env, metadata: ClassicMetadata) {
        panic!("calling into interface");
    }

    fn init_token(env: Env, admin: Identifier, metadata: TokenMetadata) {
        panic!("calling into interface");
    }

    fn nonce(env: Env, id: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    fn allowance(env: Env, from: Identifier, spender: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    fn approve(env: Env, from: Signature, nonce: BigInt, spender: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    fn balance(env: Env, id: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    fn is_frozen(env: Env, id: Identifier) -> bool {
        panic!("calling into interface");
    }

    fn xfer(env: Env, from: Signature, nonce: BigInt, to: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    fn xfer_from(
        env: Env,
        spender: Signature,
        nonce: BigInt,
        from: Identifier,
        to: Identifier,
        amount: BigInt,
    ) {
        panic!("calling into interface");
    }

    fn burn(env: Env, admin: Signature, nonce: BigInt, from: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    fn freeze(env: Env, admin: Signature, nonce: BigInt, id: Identifier) {
        panic!("calling into interface");
    }

    fn mint(env: Env, admin: Signature, nonce: BigInt, to: Identifier, amount: BigInt) {
        panic!("calling into interface");
    }

    fn set_admin(env: Env, admin: Signature, nonce: BigInt, new_admin: Identifier) {
        panic!("calling into interface");
    }

    fn unfreeze(env: Env, admin: Signature, nonce: BigInt, id: Identifier) {
        panic!("calling into interface");
    }

    fn decimals(env: Env) -> u32 {
        panic!("calling into interface");
    }

    fn name(env: Env) -> Bytes {
        panic!("calling into interface");
    }

    fn symbol(env: Env) -> Bytes {
        panic!("calling into interface");
    }

    fn to_smart(env: Env, id: Signature, nonce: BigInt, amount: i64) {
        panic!("calling into interface");
    }

    fn to_classic(env: Env, id: Signature, nonce: BigInt, amount: i64) {
        panic!("calling into interface");
    }
}
