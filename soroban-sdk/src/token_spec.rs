#![no_std]

use soroban_sdk_macros::contractimpl;

use crate::{Address, Env, Bytes};


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
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        panic!("calling into interface");
    }

    pub fn increase_allowance(env: Env, from: Address, spender: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn decrease_allowance(env: Env, from: Address, spender: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        panic!("calling into interface");
    }

    pub fn spendable_balance(env: Env, id: Address) -> i128 {
        panic!("calling into interface");
    }

    pub fn authorized(env: Env, id: Address) -> bool {
        panic!("calling into interface");
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn set_authorized(env: Env, id: Address, authorize: bool) {
        panic!("calling into interface");
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn clawback(env: Env, from: Address, amount: i128) {
        panic!("calling into interface");
    }

    pub fn set_admin(env: Env, new_admin: Address) {
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

const SPEC_XDR_INPUT: &[&[u8]] = &[
    &Token::spec_xdr_allowance(),
    &Token::spec_xdr_authorized(),
    &Token::spec_xdr_balance(),
    &Token::spec_xdr_burn(),
    &Token::spec_xdr_burn_from(),
    &Token::spec_xdr_clawback(),
    &Token::spec_xdr_decimals(),
    &Token::spec_xdr_decrease_allowance(),
    &Token::spec_xdr_increase_allowance(),
    &Token::spec_xdr_mint(),
    &Token::spec_xdr_name(),
    &Token::spec_xdr_set_admin(),
    &Token::spec_xdr_set_authorized(),
    &Token::spec_xdr_spendable_balance(),
    &Token::spec_xdr_symbol(),
    &Token::spec_xdr_transfer(),
    &Token::spec_xdr_transfer_from(),
];

const SPEC_XDR_LEN: usize = 1108;

/// Returns the XDR spec for the Token contract.
#[doc(hidden)]
pub const fn spec_xdr() -> [u8; SPEC_XDR_LEN] {
    let input = SPEC_XDR_INPUT;
    // Concatenate all XDR for each item that makes up the token spec.
    let mut output = [0u8; SPEC_XDR_LEN];
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
