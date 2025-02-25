use crate::{contractclient, contractspecfn, Address, Env};

/// Extension interface for Token contracts that implement the transfer_muxed
/// extension, such as the Stellar Asset Contract.
///
/// Defined by [SEP-??].
///
/// [SEP-??]: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-00??.md
///
/// Tokens allow holders of the token to transfer tokens to other addresses, and
/// the transfer_muxed extension allows a token to be transferred with an
/// accompanying muxed ID for both the from and to addresses. The muxed IDs are
/// emitted in respective events.
///
/// Tokens implementing the extension expose a single function for doing so:
/// - [`transfer_muxed`][Self::transfer_muxed]
#[contractspecfn(name = "super::StellarAssetSpec", export = false)]
#[contractclient(crate_path = "crate", name = "TokenMuxedExtTransferClient")]
pub trait TokenMuxedExtInterface {
    /// Transfer `amount` from `from` to `to`.
    ///
    /// Passess through the `from_id` and `to_id` to the event.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be
    ///   withdrawn from.
    /// * `from_mux` - The muxed ID of the sender to be emitted in the event.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `to_mux` - The muxed ID of the receiver to be emitted in the event.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer_muxed", from: Address, to: Address],
    /// data = {amount: i128, from_id: u64, to_id: u64}`
    fn transfer_muxed(env: Env, from: Address, from_id: u64, to: Address, to_id: u64, amount: i128);
}
