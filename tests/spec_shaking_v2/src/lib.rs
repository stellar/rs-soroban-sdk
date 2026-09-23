#![cfg_attr(target_family = "wasm", no_std)]

// The contract is built with soroban-sdk 28.0.0 from crates.io, the last SDK to
// record spec shaking version 2, so that the tests check the current
// soroban-spec still shakes a v2 contract as it did. That SDK is a wasm only
// dependency, which leaves the host build with the tests alone.
#[cfg(target_family = "wasm")]
mod contract;

#[cfg(test)]
mod test;
