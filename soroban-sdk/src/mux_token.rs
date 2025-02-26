//! Token contains types for calling and accessing token contracts, including
//! the Stellar Asset Contract.
//!
//! See [`TokenInterface`] for the interface of token contracts such as the
//! Stellar Asset Contract.
//!
//! Use [`TokenClient`] for calling token contracts such as the Stellar Asset
//! Contract.

use crate::{contractclient, contractspecfn, Address, Env, MuxedAddress, String};

// The interface below was copied from
// https://github.com/stellar/rs-soroban-env/blob/main/soroban-env-host/src/native_contract/token/contract.rs
// at commit b3c188f48dec51a956c1380fb6fe92201a3f716b.
//
// Differences between this interface and the built-in contract
// 1. The return values here don't return Results.
// 2. The implementations have been replaced with a panic.
// 3. &Host type usage are replaced with Env

#[contractspecfn(name = "StellarAssetMuxSpec", export = false)]
#[contractclient(crate_path = "crate", name = "MuxTokenClient")]
pub trait MuxTokenInterface {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// The amount returned is the amount that spender is allowed to transfer
    /// out of from's balance. When the spender transfers amounts, the allowance
    /// will be reduced by the amount transferred.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`.
    ///
    /// The amount set is the amount that spender is approved to transfer out of
    /// from's balance. The spender will be allowed to transfer amounts, and
    /// when an amount is transferred the allowance will be reduced by the
    /// amount transferred.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address being authorized to spend the tokens held by
    ///   `from`.
    /// * `amount` - The tokens to be made available to `spender`.
    /// * `expiration_ledger` - The ledger number where this allowance expires. Cannot
    ///    be less than the current ledger number unless the amount is being set to 0.
    ///    An expired entry (where expiration_ledger < the current ledger number)
    ///    should be treated as a 0 amount allowance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address,
    /// spender: Address], data = [amount: i128, expiration_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried. If the
    ///   address has no existing balance, returns 0.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = amount: i128`
    fn transfer(env: Env, from: MuxedAddress, to: MuxedAddress, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance that
    /// `spender` has on `from`'s balance. Authorized by spender
    /// (`spender.require_auth()`).
    ///
    /// The spender will be allowed to transfer the amount from from's balance
    /// if the amount is less than or equal to the allowance that the spender
    /// has on the from's balance. The spender's allowance on from's balance
    /// will be reduced by the amount.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the transfer, and having its
    ///   allowance consumed during the transfer.
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address],
    /// data = amount: i128`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    ///
    /// Reduces from's balance by the amount, without transferring the balance
    /// to another holder's balance.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = amount:
    /// i128`
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// Reduces from's balance by the amount, without transferring the balance
    /// to another holder's balance.
    ///
    /// The spender will be allowed to burn the amount from from's balance, if
    /// the amount is less than or equal to the allowance that the spender has
    /// on the from's balance. The spender's allowance on from's balance will be
    /// reduced by the amount.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the burn, and having its allowance
    ///   consumed during the burn.
    /// * `from` - The address holding the balance of tokens which will be
    ///   burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address], data = amount:
    /// i128`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

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
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    ///
    /// # Panics
    ///
    /// If the contract has not yet been initialized.
    fn symbol(env: Env) -> String;
}

#[doc(hidden)]
pub struct StellarAssetMuxSpec;

/// Interface for admin capabilities for Token contracts, such as the Stellar
/// Asset Contract.
#[contractspecfn(name = "StellarAssetMuxSpec", export = false)]
#[contractclient(crate_path = "crate", name = "StellarAssetMuxClient")]
pub trait StellarAssetInterface {
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

    /// Returns the admin of the contract.
    ///
    /// # Panics
    ///
    /// If the admin is not set.
    fn admin(env: Env) -> Address;

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

    /// Returns true if `id` is authorized to use its balance.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which token authorization is being checked.
    fn authorized(env: Env, id: Address) -> bool;

    /// Mints `amount` to `to`.
    ///
    /// # Arguments
    ///
    /// * `to` - The address which will receive the minted tokens.
    /// * `amount` - The amount of tokens to be minted.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["mint", admin: Address, to: Address], data
    /// = amount: i128`
    fn mint(env: Env, to: Address, amount: i128);

    /// Clawback `amount` from `from` account. `amount` is burned in the
    /// clawback process.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance from which the clawback will
    ///   take tokens.
    /// * `amount` - The amount of tokens to be clawed back.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["clawback", admin: Address, to: Address],
    /// data = amount: i128`
    fn clawback(env: Env, from: Address, amount: i128);
}

pub(crate) const SPEC_XDR_INPUT: &[&[u8]] = &[
    &StellarAssetMuxSpec::spec_xdr_allowance(),
    &StellarAssetMuxSpec::spec_xdr_authorized(),
    &StellarAssetMuxSpec::spec_xdr_approve(),
    &StellarAssetMuxSpec::spec_xdr_balance(),
    &StellarAssetMuxSpec::spec_xdr_burn(),
    &StellarAssetMuxSpec::spec_xdr_burn_from(),
    &StellarAssetMuxSpec::spec_xdr_clawback(),
    &StellarAssetMuxSpec::spec_xdr_decimals(),
    &StellarAssetMuxSpec::spec_xdr_mint(),
    &StellarAssetMuxSpec::spec_xdr_name(),
    &StellarAssetMuxSpec::spec_xdr_set_admin(),
    &StellarAssetMuxSpec::spec_xdr_admin(),
    &StellarAssetMuxSpec::spec_xdr_set_authorized(),
    &StellarAssetMuxSpec::spec_xdr_symbol(),
    &StellarAssetMuxSpec::spec_xdr_transfer(),
    &StellarAssetMuxSpec::spec_xdr_transfer_from(),
];

pub(crate) const SPEC_XDR_LEN: usize = 6532;

impl StellarAssetMuxSpec {
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
