//! Token contains types for calling and accessing token contracts, including
//! the Stellar Asset Contract.
//!
//! See [`Interface`] for the interface of token contracts such as the Stellar
//! Asset Contract.
//!
//! See [`Client`] for calling token contracts such as the Stellar Asset
//! Contract.
use crate as soroban_sdk;
use crate::{contractclient, Address, Bytes, Env};


// The interface below was copied from
// https://github.com/stellar/rs-soroban-env/blob/main/soroban-env-host/src/native_contract/token/contract.rs
// at commit b3c188f48dec51a956c1380fb6fe92201a3f716b.
//
// Differences between this interface and the built-in contract
// 1. The return values here don't return Results.
// 2. The implementations have been replaced with a panic.
// 3. &Host type usage are replaced with Env

/// Interface for Token contracts, such as the Stellar Asset Contract.
#[contractclient(name = "Client")]
pub trait Interface {
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    fn increase_allowance(env: Env, from: Address, spender: Address, amount: i128);
    fn decrease_allowance(env: Env, from: Address, spender: Address, amount: i128);
    fn balance(env: Env, id: Address) -> i128;
    fn spendable_balance(env: Env, id: Address) -> i128;
    fn authorized(env: Env, id: Address) -> bool;
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);
    fn set_authorized(env: Env, id: Address, authorize: bool);
    fn mint(env: Env, to: Address, amount: i128);
    fn clawback(env: Env, from: Address, amount: i128);
    fn set_admin(env: Env, new_admin: Address);
    fn decimals(env: Env) -> u32;
    fn name(env: Env) -> Bytes;
    fn symbol(env: Env) -> Bytes;
}

// TODO: Modify the macros to still provide a way to get this spec.

// const SPEC_XDR_INPUT: &[&[u8]] = &[
//     &Token::spec_xdr_allowance(),
//     &Token::spec_xdr_authorized(),
//     &Token::spec_xdr_balance(),
//     &Token::spec_xdr_burn(),
//     &Token::spec_xdr_burn_from(),
//     &Token::spec_xdr_clawback(),
//     &Token::spec_xdr_decimals(),
//     &Token::spec_xdr_decrease_allowance(),
//     &Token::spec_xdr_increase_allowance(),
//     &Token::spec_xdr_mint(),
//     &Token::spec_xdr_name(),
//     &Token::spec_xdr_set_admin(),
//     &Token::spec_xdr_set_authorized(),
//     &Token::spec_xdr_spendable_balance(),
//     &Token::spec_xdr_symbol(),
//     &Token::spec_xdr_transfer(),
//     &Token::spec_xdr_transfer_from(),
// ];

// const SPEC_XDR_LEN: usize = 1108;

// /// Returns the XDR spec for the Token contract.
// #[doc(hidden)]
// pub const fn spec_xdr() -> [u8; SPEC_XDR_LEN] {
//     let input = SPEC_XDR_INPUT;
//     // Concatenate all XDR for each item that makes up the token spec.
//     let mut output = [0u8; SPEC_XDR_LEN];
//     let mut input_i = 0;
//     let mut output_i = 0;
//     while input_i < input.len() {
//         let subinput = input[input_i];
//         let mut subinput_i = 0;
//         while subinput_i < subinput.len() {
//             output[output_i] = subinput[subinput_i];
//             output_i += 1;
//             subinput_i += 1;
//         }
//         input_i += 1;
//     }

//     // Check that the numbers of bytes written is equal to the number of bytes
//     // expected in the output.
//     if output_i != output.len() {
//         panic!("unexpected output length",);
//     }

//     output
// }
