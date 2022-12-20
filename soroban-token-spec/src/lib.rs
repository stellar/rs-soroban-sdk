#![no_std]

use soroban_auth::{Identifier, Signature};
use soroban_sdk::{contractimpl, Bytes, Env};

mod tests;

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
    pub fn nonce(env: Env, id: Identifier) -> i128 {
        panic!("calling into interface");
    }

    pub fn allowance(env: Env, from: Identifier, spender: Identifier) -> i128 {
        panic!("calling into interface");
    }

    pub fn incr_allow(env: Env, from: Signature, nonce: i128, spender: Identifier, amount: i128) {
        panic!("calling into interface");
    }

    pub fn decr_allow(env: Env, from: Signature, nonce: i128, spender: Identifier, amount: i128) {
        panic!("calling into interface");
    }

    pub fn balance(env: Env, id: Identifier) -> i128 {
        panic!("calling into interface");
    }

    pub fn spendable(env: Env, id: Identifier) -> i128 {
        panic!("calling into interface");
    }

    pub fn authorized(env: Env, id: Identifier) -> bool {
        panic!("calling into interface");
    }

    pub fn xfer(env: Env, from: Signature, nonce: i128, to: Identifier, amount: i128) {
        panic!("calling into interface");
    }

    pub fn xfer_from(
        env: Env,
        spender: Signature,
        nonce: i128,
        from: Identifier,
        to: Identifier,
        amount: i128,
    ) {
        panic!("calling into interface");
    }

    pub fn burn(env: Env, from: Signature, nonce: i128, amount: i128) {
        panic!("calling into interface");
    }

    pub fn burn_from(env: Env, spender: Signature, nonce: i128, from: Identifier, amount: i128) {
        panic!("calling into interface");
    }

    pub fn set_auth(env: Env, admin: Signature, nonce: i128, id: Identifier, authorize: bool) {
        panic!("calling into interface");
    }

    pub fn mint(env: Env, admin: Signature, nonce: i128, to: Identifier, amount: i128) {
        panic!("calling into interface");
    }

    pub fn clawback(env: Env, admin: Signature, nonce: i128, from: Identifier, amount: i128) {
        panic!("calling into interface");
    }

    pub fn set_admin(env: Env, admin: Signature, nonce: i128, new_admin: Identifier) {
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
}

/// Returns the XDR spec for the Token contract.
#[doc(hidden)]
pub const fn spec_xdr() -> [u8; 2012] {
    let input: &[&[u8]] = &[
        &Token::spec_xdr_allowance(),
        &Token::spec_xdr_authorized(),
        &Token::spec_xdr_balance(),
        &Token::spec_xdr_burn(),
        &Token::spec_xdr_burn_from(),
        &Token::spec_xdr_clawback(),
        &Token::spec_xdr_decimals(),
        &Token::spec_xdr_decr_allow(),
        &Token::spec_xdr_incr_allow(),
        &Token::spec_xdr_mint(),
        &Token::spec_xdr_name(),
        &Token::spec_xdr_nonce(),
        &Token::spec_xdr_set_admin(),
        &Token::spec_xdr_set_auth(),
        &Token::spec_xdr_spendable(),
        &Token::spec_xdr_symbol(),
        &Token::spec_xdr_xfer(),
        &Token::spec_xdr_xfer_from(),
        &soroban_auth::Identifier::spec_xdr(),
        &soroban_auth::Signature::spec_xdr(),
        &soroban_auth::Ed25519Signature::spec_xdr(),
        &soroban_auth::AccountSignatures::spec_xdr(),
    ];

    // Concatenate all XDR for each item that makes up the token spec.
    let mut output = [0u8; 2012];
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

    // Check that the numbers of bytes written is equal to the number of bytes
    // expected in the output.
    if output_i != output.len() {
        panic!("unexpected output length",);
    }

    output
}
