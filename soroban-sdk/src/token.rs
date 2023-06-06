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
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of funds to be drawn from.
    /// * `spender` - The address spending the funds held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Increase the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of funds to be drawn from.
    /// * `spender` - The address being authorized to spend the funds held by
    ///   `from`.
    /// * `amount` - The additional funds to be made availabe to `spender`.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["increase_allowance", from: Address,
    /// spender: Address], data = [amount: i128]`
    fn increase_allowance(env: Env, from: Address, spender: Address, amount: i128);

    /// Decrease the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of funds to be drawn from.
    /// * `spender` - The address being (de-)authorized to spend the funds held
    ///   by `from`.
    /// * `amount` - The funds which are no longer availabe for use by
    ///   `spender`. If `amount` is greater than the current allowance, set the
    ///   allowance to 0.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["decrease_allowance", from: Address,
    /// spender: Address], data = [amount: i128]`
    fn decrease_allowance(env: Env, from: Address, spender: Address, amount: i128);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried. If the
    ///   address has no existing balance, returns 0.
    fn balance(env: Env, id: Address) -> i128;

    /// Returns the spendable balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a spendable balance is being queried.
    ///   This will return the same value as `balance()` unless this is called
    ///   on the Stellar Asset Contract, in which case this can be less due to
    ///   reserves/liabilities.
    fn spendable_balance(env: Env, id: Address) -> i128;

    /// Returns true if `id` is authorized to use its balance.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which token authorization is being checked.
    fn authorized(env: Env, id: Address) -> bool;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of funds which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred funds.
    /// * `amount` - The amount of funds to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the transfer, and having its
    ///   allowance consumed during the transfer.
    /// * `from` - The address holding the balance of funds which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred funds.
    /// * `amount` - The amount of funds to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = [amount: i128]`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of funds which will be
    ///   burned from.
    /// * `amount` - The amount of funds to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = [amount:
    /// i128]`
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the burn, and having its allowance
    ///   consumed during the burn.
    /// * `from` - The address holding the balance of funds which will be
    ///   burned from.
    /// * `amount` - The amount of funds to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = [amount:
    /// i128]`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Sets whether the account is authorized to use its balance. If
    /// `authorized` is true, `id` should be able to use its balance.
    ///
    /// # Arguments
    ///
    /// * `id` - The address being (de-)authorized.
    /// * `authorize` - Whether or not `id` can use its balance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["set_authorized", id: Address], data =
    /// [authorize: bool]`
    fn set_authorized(env: Env, id: Address, authorize: bool);

    /// Mints `amount` to `to`.
    ///
    /// # Arguments
    ///
    /// * `to` - The address which will receive the minted funds.
    /// * `amount` - The amount of funds to be minted.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["mint", admin: Address, to: Address], data
    /// = [amount: i128]`
    fn mint(env: Env, to: Address, amount: i128);

    /// Clawback `amount` from `from` account. `amount` is burned in the
    /// clawback process.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance from which the clawback will
    ///   take funds.
    /// * `amount` - The amount of funds to be clawed back.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["clawback", admin: Address, to: Address],
    /// data = [amount: i128]`
    fn clawback(env: Env, from: Address, amount: i128);

    /// Sets the administrator to the specified address `new_admin`.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The address which will henceforth be the administrator
    ///   of this token contract.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["set_admin", admin: Address], data =
    /// [new_admin: Address]`
    fn set_admin(env: Env, new_admin: Address);

    /// Returns the number of decimals used to represent amounts of this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn name(env: Env) -> Bytes;

    /// Returns the symbol for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
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
