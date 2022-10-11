#![no_std]

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

impl Token {
    /// Obtains the full spec for the Token contract
    pub fn spec_xdr() -> [u8; 2037] {
        let input: &[&[u8]] = &[
            &Self::spec_xdr_allowance(),
            &Self::spec_xdr_approve(),
            &Self::spec_xdr_balance(),
            &Self::spec_xdr_burn(),
            &Self::spec_xdr_decimals(),
            &Self::spec_xdr_export(),
            &Self::spec_xdr_freeze(),
            &Self::spec_xdr_import(),
            &Self::spec_xdr_init(),
            &Self::spec_xdr_is_frozen(),
            &Self::spec_xdr_mint(),
            &Self::spec_xdr_name(),
            &Self::spec_xdr_nonce(),
            &Self::spec_xdr_set_admin(),
            &Self::spec_xdr_symbol(),
            &Self::spec_xdr_unfreeze(),
            &Self::spec_xdr_xfer(),
            &Self::spec_xdr_xfer_from(),
            &TokenMetadata::spec_xdr(),
            &soroban_auth::Identifier::spec_xdr(),
            &soroban_auth::Signature::spec_xdr(),
            &soroban_auth::Ed25519Signature::spec_xdr(),
            &soroban_auth::AccountSignatures::spec_xdr(),
        ];

        let mut output = [0u8; 2037];
        let mut input_i = 0;
        let mut output_i = 0;
        while input_i < input.len() {
            let subinput = input[input_i];
            let mut subinput_i = 0;
            while subinput_i < subinput.len() {
                output[output_i] = subinput[subinput_i];
                output_i += 1;
                subinput_i += 1;
            }
            input_i += 1;
        }
        // Unfortunately we cannot call assert_eq!() in a const function
        if output_i != output.len() - 1 {
            panic!("unexpected output length",);
        }
        output
    }
}
