#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod read;
#[cfg(feature = "std")]
pub mod reduce;
pub mod shaking;
