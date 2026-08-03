#![no_std]

mod tests;

use soroban_sdk::{contractevent, Address, BytesN, String};

// SAC-specific variants of SEP-41 events and events unique to the Stellar Asset Contract are
// defined here privately only for generating the complete contract spec.
// All SAC events include sep0011_asset (the SEP-0011 asset string, e.g. "native" or
// "USDC:GABC...") as a trailing topic, per the SAC implementation in rs-soroban-env.
// These structs share the same base names as the token-sdk event structs so the spec
// `name` field matches and the subset invariant test can verify them.

#[contractevent(topics = ["approve"], data_format = "vec", export = false)]
pub(crate) struct Approve {
    #[topic]
    pub from: Address,
    #[topic]
    pub spender: Address,
    #[topic]
    pub sep0011_asset: String,
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contractevent(topics = ["transfer"], data_format = "single-value", export = false)]
pub(crate) struct TransferWithAmountOnly {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub amount: i128,
}

#[contractevent(topics = ["transfer"], data_format = "map", export = false)]
pub(crate) struct Transfer {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub to_muxed_id: Option<u64>,
    pub amount: i128,
}

#[contractevent(topics = ["transfer"], data_format = "map", export = false)]
pub(crate) struct TransferWithMuxedString {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub to_muxed_id: Option<String>,
    pub amount: i128,
}

#[contractevent(topics = ["transfer"], data_format = "map", export = false)]
pub(crate) struct TransferWithMuxedBytes {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub to_muxed_id: Option<BytesN<32>>,
    pub amount: i128,
}

#[contractevent(topics = ["burn"], data_format = "single-value", export = false)]
pub(crate) struct Burn {
    #[topic]
    pub from: Address,
    #[topic]
    pub sep0011_asset: String,
    pub amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "single-value", export = false)]
pub(crate) struct MintWithAmountOnly {
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "map", export = false)]
pub(crate) struct Mint {
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub to_muxed_id: Option<u64>,
    pub amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "map", export = false)]
pub(crate) struct MintWithMuxedString {
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub to_muxed_id: Option<String>,
    pub amount: i128,
}

#[contractevent(topics = ["mint"], data_format = "map", export = false)]
pub(crate) struct MintWithMuxedBytes {
    #[topic]
    pub to: Address,
    #[topic]
    pub sep0011_asset: String,
    pub to_muxed_id: Option<BytesN<32>>,
    pub amount: i128,
}

#[contractevent(topics = ["clawback"], data_format = "single-value", export = false)]
pub(crate) struct Clawback {
    #[topic]
    pub from: Address,
    #[topic]
    pub sep0011_asset: String,
    pub amount: i128,
}

#[contractevent(topics = ["set_admin"], data_format = "single-value", export = false)]
pub(crate) struct SetAdmin {
    #[topic]
    pub admin: Address,
    #[topic]
    pub sep0011_asset: String,
    pub new_admin: Address,
}

#[contractevent(topics = ["set_authorized"], data_format = "single-value", export = false)]
pub(crate) struct SetAuthorized {
    #[topic]
    pub id: Address,
    #[topic]
    pub sep0011_asset: String,
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
    &soroban_sdk::token::StellarAssetFnSpec::spec_xdr_trust(),
    &Approve::spec_xdr(),
    &TransferWithAmountOnly::spec_xdr(),
    &Transfer::spec_xdr(),
    &TransferWithMuxedString::spec_xdr(),
    &TransferWithMuxedBytes::spec_xdr(),
    &Burn::spec_xdr(),
    &MintWithAmountOnly::spec_xdr(),
    &Mint::spec_xdr(),
    &MintWithMuxedString::spec_xdr(),
    &MintWithMuxedBytes::spec_xdr(),
    &Clawback::spec_xdr(),
    &SetAdmin::spec_xdr(),
    &SetAuthorized::spec_xdr(),
];

pub(crate) const XDR_LEN: usize = {
    let input = XDR_INPUT;
    let mut len = 0usize;
    let mut i = 0;
    while i < input.len() {
        len += input[i].len();
        i += 1;
    }
    len
};

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
