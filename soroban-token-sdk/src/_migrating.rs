//! # Migrating from v23 to v26
//!
//! 1. Remove the deprecated event format. For details, see the [migration guide for v23 contract events].
//!    And for an example using the `soroban_token_sdk` directly, see the [migration guide for v23 token transfer][v23_token_transfer].
//!
//! [migration guide for v23 contract events]: soroban_sdk::_migrating::v23_contractevent
//!
//! # Migrating from v22 to v23
//!
//! 1. [`MuxedAddress` replaces `Address` as the `to` of the `TokenInterface::transfer`][v23_token_transfer]

pub mod v23_token_transfer;
