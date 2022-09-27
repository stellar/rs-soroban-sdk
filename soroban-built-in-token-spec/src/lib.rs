#![no_std]
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
    fn init_wrap(_env: Env, _metadata: ClassicMetadata);

    /// init_token creates a token contract that does not wrap an asset on the classic side.
    /// No checks are done on the contractID.
    fn init_token(_env: Env, _admin: Identifier, _metadata: TokenMetadata);

    fn nonce(_env: Env, _id: Identifier) -> BigInt;

    fn allowance(_env: Env, _from: Identifier, _spender: Identifier) -> BigInt;

    fn approve(_env: Env, _from: Signature, _nonce: BigInt, _spender: Identifier, _amount: BigInt);

    fn balance(_env: Env, _id: Identifier) -> BigInt;

    fn is_frozen(_env: Env, _id: Identifier) -> bool;

    fn xfer(_env: Env, _from: Signature, _nonce: BigInt, _to: Identifier, _amount: BigInt);

    fn xfer_from(
        _env: Env,
        _spender: Signature,
        _nonce: BigInt,
        _from: Identifier,
        _to: Identifier,
        _amount: BigInt,
    );

    fn burn(_env: Env, _admin: Signature, _nonce: BigInt, _from: Identifier, _amount: BigInt);

    fn freeze(_env: Env, _admin: Signature, _nonce: BigInt, _id: Identifier);

    fn mint(_env: Env, _admin: Signature, _nonce: BigInt, _to: Identifier, _amount: BigInt);

    fn set_admin(_env: Env, _admin: Signature, _nonce: BigInt, _new_admin: Identifier);

    fn unfreeze(_env: Env, _admin: Signature, _nonce: BigInt, _id: Identifier);

    fn decimals(_env: Env) -> u32;

    fn name(_env: Env) -> Bytes;

    fn symbol(_env: Env) -> Bytes;

    fn to_smart(_env: Env, _id: Signature, _nonce: BigInt, _amount: i64);

    fn to_classic(_env: Env, _id: Signature, _nonce: BigInt, _amount: i64);
}

pub struct Token;

#[contractimpl]
impl TokenTrait for Token {
    fn init_wrap(_env: Env, _metadata: ClassicMetadata) {
        panic!("calling into interface");
    }

    fn init_token(_env: Env, _admin: Identifier, _metadata: TokenMetadata) {
        panic!("calling into interface");
    }

    fn nonce(_env: Env, _id: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    fn allowance(_env: Env, _from: Identifier, _spender: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    fn approve(_env: Env, _from: Signature, _nonce: BigInt, _spender: Identifier, _amount: BigInt) {
        panic!("calling into interface");
    }

    fn balance(_env: Env, _id: Identifier) -> BigInt {
        panic!("calling into interface");
    }

    fn is_frozen(_env: Env, _id: Identifier) -> bool {
        panic!("calling into interface");
    }

    fn xfer(_env: Env, _from: Signature, _nonce: BigInt, _to: Identifier, _amount: BigInt) {
        panic!("calling into interface");
    }

    fn xfer_from(
        _env: Env,
        _spender: Signature,
        _nonce: BigInt,
        _from: Identifier,
        _to: Identifier,
        _amount: BigInt,
    ) {
        panic!("calling into interface");
    }

    fn burn(_env: Env, _admin: Signature, _nonce: BigInt, _from: Identifier, _amount: BigInt) {
        panic!("calling into interface");
    }

    fn freeze(_env: Env, _admin: Signature, _nonce: BigInt, _id: Identifier) {
        panic!("calling into interface");
    }

    fn mint(_env: Env, _admin: Signature, _nonce: BigInt, _to: Identifier, _amount: BigInt) {
        panic!("calling into interface");
    }

    fn set_admin(_env: Env, _admin: Signature, _nonce: BigInt, _new_admin: Identifier) {
        panic!("calling into interface");
    }

    fn unfreeze(_env: Env, _admin: Signature, _nonce: BigInt, _id: Identifier) {
        panic!("calling into interface");
    }

    fn decimals(_env: Env) -> u32 {
        panic!("calling into interface");
    }

    fn name(_env: Env) -> Bytes {
        panic!("calling into interface");
    }

    fn symbol(_env: Env) -> Bytes {
        panic!("calling into interface");
    }

    fn to_smart(_env: Env, _id: Signature, _nonce: BigInt, _amount: i64) {
        panic!("calling into interface");
    }

    fn to_classic(_env: Env, _id: Signature, _nonce: BigInt, _amount: i64) {
        panic!("calling into interface");
    }
}
