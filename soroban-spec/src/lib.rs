#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod read;
pub mod shaking;
#[cfg(feature = "std")]
pub mod simplify;
