//! Token contains types for calling and accessing token contracts, including
//! the Stellar Asset Contract.
//!
//! See [`Interface`] for the interface of token contracts such as the Stellar
//! Asset Contract.
//!
//! Use [`Client`] for calling token contracts such as the Stellar Asset
//! Contract.

use crate::{contractclient, contractspecfn, Address, Bytes, Env};

// The interface below was copied from
// https://github.com/stellar/rs-soroban-env/blob/main/soroban-env-host/src/native_contract/token/contract.rs
// at commit b3c188f48dec51a956c1380fb6fe92201a3f716b.
//
// Differences between this interface and the built-in contract
// 1. The return values here don't return Results.
// 2. The implementations have been replaced with a panic.
// 3. &Host type usage are replaced with Env

/// Spec contains the contract spec of Token contracts, such as the Stellar
/// Asset Contract.
pub struct Spec;

/// Interface for Token contracts, such as the Stellar Asset Contract.
#[contractspecfn(name = "Spec", export = false)]
#[contractclient(crate_path = "crate", name = "Client")]
pub trait Interface {
    /// Get the allowance for `spender` to transfer from `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Increase the allowance by `amount` for `spender` to transfer/burn from `from`.
    /// Emit event with topics = ["increase_allowance", from: Address, spender: Address], data = [amount: i128]
    fn increase_allowance(env: Env, from: Address, spender: Address, amount: i128);

    /// Decrease the allowance by `amount` for `spender` to transfer/burn from `from`.
    /// If `amount` is greater than the current allowance, set the allowance to 0.
    /// Emit event with topics = ["decrease_allowance", from: Address, spender: Address], data = [amount: i128]
    fn decrease_allowance(env: Env, from: Address, spender: Address, amount: i128);

    /// Get the balance of `id`.
    fn balance(env: Env, id: Address) -> i128;

    /// Get the spendable balance of `id`. This will return the same value as balance()
    /// unless this is called on the Stellar Asset Contract, in which case this can
    /// be less due to reserves/liabilities.
    fn spendable_balance(env: Env, id: Address) -> i128;

    /// Returns true if `id` is authorized to use its balance.
    fn authorized(env: Env, id: Address) -> bool;

    /// Transfer `amount` from `from` to `to`.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [amount: i128]
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of `spender`.
    /// Authorized by spender (`spender.require_auth()`).
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [amount: i128]
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    /// Emit event with topics = ["burn", from: Address], data = [amount: i128]
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    /// Emit event with topics = ["burn", from: Address], data = [amount: i128]
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Sets whether the account is authorized to use its balance.
    /// If `authorized` is true, `id` should be able to use its balance.
    /// Emit event with topics = ["set_authorized", id: Address], data = [authorize: bool]
    fn set_authorized(env: Env, id: Address, authorize: bool);

    /// Mints `amount` to `to`.
    /// Emit event with topics = ["mint", admin: Address, to: Address], data = [amount: i128]
    fn mint(env: Env, to: Address, amount: i128);

    /// Clawback `amount` from `from` account. `amount` is burned.
    /// Emit event with topics = ["clawback", admin: Address, to: Address], data = [amount: i128]
    fn clawback(env: Env, from: Address, amount: i128);

    /// Sets the administrator to the specified address `new_admin`.
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(env: Env, new_admin: Address);

    /// Get the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    /// Get the name for this token.
    fn name(env: Env) -> Bytes;

    /// Get the symbol for this token.
    fn symbol(env: Env) -> Bytes;
}

pub(crate) const SPEC_XDR_INPUT: &[&[u8]] = &[
    &Spec::spec_xdr_allowance(),
    &Spec::spec_xdr_authorized(),
    &Spec::spec_xdr_balance(),
    &Spec::spec_xdr_burn(),
    &Spec::spec_xdr_burn_from(),
    &Spec::spec_xdr_clawback(),
    &Spec::spec_xdr_decimals(),
    &Spec::spec_xdr_decrease_allowance(),
    &Spec::spec_xdr_increase_allowance(),
    &Spec::spec_xdr_mint(),
    &Spec::spec_xdr_name(),
    &Spec::spec_xdr_set_admin(),
    &Spec::spec_xdr_set_authorized(),
    &Spec::spec_xdr_spendable_balance(),
    &Spec::spec_xdr_symbol(),
    &Spec::spec_xdr_transfer(),
    &Spec::spec_xdr_transfer_from(),
];

pub(crate) const SPEC_XDR_LEN: usize = 1108;

impl Spec {
    /// Returns the XDR spec for the Token contract.
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
}
