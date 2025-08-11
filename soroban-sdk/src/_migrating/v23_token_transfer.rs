//! Token interface has been updated to use [`MuxedAddress`] instead of
//! [`Address`] for the transfer destination.
//!
//! Note, that Stellar Asset contract supports the updated interface as well.
//!
//! `MuxedAddress` is a special type that is compatible with `Address` in
//! most of the contexts. If a contract function accepts a `MuxedAddress`
//! argument, then it can also accept an `Address` argument seamlessly, so
//! the existing contracts or tests that interact with the updated token
//! interface don't require any changes.
//!
//! The token implementations should be updated to use the new interface.
//! `MuxedAddress` allows users to attach a 64-bit ID to the payment
//! destination in order to identify a 'virtual' account, such as an exchange
//! deposit account. Note, that this change is not sufficient to support
//! the exchange deposits for the token contracts, but is necessary for that.
//!
//! The necessary token modification is very minimal. Consider the following
//! `transfer` implementation that still uses `Address` destination:
//!
//! ```
//! use soroban_sdk::{Env, Address, token};
//! // ... inside some token contract ...
//! fn transfer(env: Env, from: Address, to: Address, amount: i128) {
//!    // Authorize the transfer source.
//!    from.require_auth();
//!    // Token-specific implementation of balance movement.
//!    token_impl::move_balance(&env, &from, &to, amount);
//!    // Publish an event (notice that this uses the new event format - see
//!    // the previous migration step).
//!    token::Transfer {
//!        from,
//!        to,
//!        amount,
//!    }.publish(&env);
//! }
//!
//! mod token_impl {
//!   use soroban_sdk::{Env, Address};
//!   pub fn move_balance(env: &Env, from: &Address, to: &Address, amount: i128) {
//!     // Token-specific implementation of balance movement.
//!   }
//! }
//! ```
//!
//! The updated implementation would look as follows:
//!
//! ```
//! use soroban_sdk::{Env, Address, MuxedAddress, token};
//! // ... inside some token contract ...
//! fn transfer(env: Env, from: Address, muxed_to: MuxedAddress, amount: i128) {
//!    // Authorize the transfer source.
//!    from.require_auth();
//!    // Extract the underlying Address by dropping the ID.
//!    let to = muxed_to.address();
//!    // Token-specific implementation of balance movement (same as before).
//!    token_impl::move_balance(&env, &from, &to, amount);
//!    // Publish an appropriate transfer event that includes the muxed ID
//!    // when it's non-None.
//!    token::publish_transfer_event(&env, &from, &muxed_to, amount);
//! }
//!
//! mod token_impl {
//!   use soroban_sdk::{Env, Address};
//!   pub fn move_balance(env: &Env, from: &Address, to: &Address, amount: i128) {
//!     // Token-specific implementation of balance movement.
//!     // This requires no changes compared to the previous version.
//!   }
//! }
//! ```
//!
//! That's the extent of the necessary changes. Note, that the transfer
//! events produced in different contexts (e.g. in case of `transfer_from`)
//! don't require any changes, because the [`MuxedAddress`] is only necessary
//! in a few narrow scenarios (such as direct transfer to an exchange).
//!
//! [`Events::publish`]: crate::events::Events::publish
//! [`Address`]: crate::MuxedAddress
//! [`MuxedAddress`]: crate::MuxedAddress
//! [`contractevent`]: crate::contractevent


