#![no_std]

#[cfg(any(feature = "testutils", test))]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
pub mod testutils;
