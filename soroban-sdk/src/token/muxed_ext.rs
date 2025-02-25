use crate::{
    contractclient, contractspecfn, Address, BytesN, ConversionError, Env, String, TryFromVal,
    TryIntoVal, Val,
};
use core::fmt::Debug;

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
    /// data = {amount: i128, from_mux: Mux, to_mux: Mux}`
    fn transfer_muxed(
        env: Env,
        from: Address,
        from_mux: Mux,
        to: Address,
        to_mux: Mux,
        amount: i128,
    );
}

/// Mux is a value that off-chain identifies a sub-identifier of an Address
/// on-chain.
///
/// A mux is also commonly referred to as a memo.
///
/// A mux may be a void (none), an ID (64-bit number), a String, or a 32-byte
/// Hash.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mux {
    None,
    Id(u64),
    Text(String),
    Hash(BytesN<32>),
}

impl From<()> for Mux {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<u64> for Mux {
    fn from(v: u64) -> Self {
        Self::Id(v)
    }
}

impl From<String> for Mux {
    fn from(v: String) -> Self {
        Self::Text(v)
    }
}

impl From<BytesN<32>> for Mux {
    fn from(v: BytesN<32>) -> Self {
        Self::Hash(v)
    }
}

impl TryFromVal<Env, Val> for Mux {
    type Error = ConversionError;

    fn try_from_val(env: &Env, v: &Val) -> Result<Self, Self::Error> {
        if v.is_void() {
            Ok(Self::None)
        } else if let Ok(v) = v.try_into_val(env) {
            Ok(Self::Id(v))
        } else if let Ok(v) = v.try_into_val(env) {
            Ok(Self::Text(v))
        } else if let Ok(v) = v.try_into_val(env) {
            Ok(Self::Hash(v))
        } else {
            Err(ConversionError)
        }
    }
}

impl TryFromVal<Env, Mux> for Val {
    type Error = ConversionError;

    fn try_from_val(env: &Env, v: &Mux) -> Result<Self, Self::Error> {
        match v {
            Mux::None => Ok(Val::VOID.to_val()),
            Mux::Id(v) => v.try_into_val(env).map_err(|_| ConversionError),
            Mux::Text(v) => v.try_into_val(env).map_err(|_| ConversionError),
            Mux::Hash(v) => v.try_into_val(env).map_err(|_| ConversionError),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::ScVal;

#[cfg(not(target_family = "wasm"))]
impl From<&Mux> for ScVal {
    fn from(v: &Mux) -> Self {
        match v {
            Mux::None => ScVal::Void,
            Mux::Id(v) => ScVal::U64(*v),
            Mux::Text(v) => v.into(),
            Mux::Hash(v) => v.into(),
        }
    }
}
