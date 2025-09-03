//! # Migrating from v22 to v23
//!
//! 1. [`MuxedAddress` replaces `Address` as the `to` of the `TokenInterface::transfer`][v23_token_transfer]

pub mod v23_token_transfer;
