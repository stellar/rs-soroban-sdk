#![no_std]

mod tests;

use soroban_sdk::{contractevent, Address};

// Events that are unique to the Stellar Asset Contract and not part of SEP-41 are defined here
// privately only for the purpose of generating the complete contract spec.

#[contractevent(topics = ["set_admin"], data_format = "single-value", export = false)]
pub(crate) struct SetAdmin {
    #[topic]
    pub admin: Address,
    pub new_admin: Address,
}

#[contractevent(topics = ["set_authorized"], data_format = "single-value", export = false)]
pub(crate) struct SetAuthorized {
    #[topic]
    pub id: Address,
    pub authorize: bool,
}

pub(crate) const XDR_INPUT: &[&[u8]] = &[
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_allowance(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_authorized(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_approve(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_balance(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_burn(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_burn_from(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_clawback(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_decimals(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_mint(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_name(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_set_admin(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_admin(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_set_authorized(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_symbol(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_transfer(),
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_transfer_from(),
    &soroban_token_sdk::events::Approve::spec_xdr(),
    &soroban_token_sdk::events::TransferLegacy::spec_xdr(),
    &soroban_token_sdk::events::Transfer::spec_xdr(),
    &soroban_token_sdk::events::Burn::spec_xdr(),
    &soroban_token_sdk::events::Mint::spec_xdr(),
    &soroban_token_sdk::events::Clawback::spec_xdr(),
    &SetAdmin::spec_xdr(),
    &SetAuthorized::spec_xdr(),
];

pub(crate) const XDR_LEN: usize = 7492;

/// Returns the contract spec for Stellar Asset contract.
pub const fn xdr() -> &'static [u8] {
    &XDR
}

/// The contract spec for the Stellar Asset contract.
const XDR: [u8; XDR_LEN] = {
    let input = XDR_INPUT;
    // Concatenate all XDR for each item that makes up the token spec.
    let mut output = [0u8; XDR_LEN];
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
};
