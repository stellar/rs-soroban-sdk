#![cfg_attr(not(feature = "std"), no_std)]

// Tests link std even when testing the no_std surface.
#[cfg(test)]
extern crate std;

#[cfg(feature = "std")]
pub mod read;
#[cfg(feature = "std")]
pub mod reduce;
pub mod shaking;
