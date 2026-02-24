//! Allocator used by contracts built with the `alloc` feature.
//!
//! The `alloc` feature is **disabled by default**. It exists to support a
//! use-case that is both expensive (in terms of CPU time and code size) and
//! typically unnecessary: allocating unbounded memory, dynamically, in the wasm
//! guest's linear-memory heap. Soroban was designed to avoid this need most of
//! the time: host objects like [`Vec`](crate::Vec), [`Map`](crate::Map), or
//! [`Bytes`](crate::Bytes) already support dynamic growth, but live in the host
//! heap and are efficient to use from the guest without any guest memory
//! allocator. Moreover even if a contract _does_ want guest-memory storage of
//! dynamic data, it can accomplish it in bounded static memory using a crate
//! like [`heapless`](https://crates.io/crates/heapless). Turning on the `alloc`
//! feature should usually be the last choice for contracts with no better
//! option.
//!
//! # Enabling
//!
//! Add the `alloc` feature to `soroban-sdk` in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! soroban-sdk = { version = "...", features = ["alloc"] }
//! ```
//!
//! With the feature enabled the SDK registers a global bump-pointer allocator
//! that services all allocations made through Rust's [`alloc`](alloc_crate)
//! APIs. This makes heap-allocated types such as `alloc::vec::Vec` and
//! `alloc::string::String` available inside contracts, and enables SDK helpers
//! that require allocation (e.g. [`Bytes::to_alloc_vec`]).
//!
//! [alloc_crate]: https://doc.rust-lang.org/alloc/
//! [`Bytes::to_alloc_vec`]: crate::Bytes::to_alloc_vec
//!
//! # Using a Custom Allocator
//!
//! The bump-pointer allocator provided by the `alloc` feature is just one
//! possible implementation.  A contract is free to define its own global
//! allocator by implementing [`GlobalAlloc`] and registering it with the
//! [`#[global_allocator]`](macro@global_allocator) attribute.  See the
//! [Rust `GlobalAlloc` documentation][global_alloc_docs] for details.
//!
//! If you supply your own allocator there is no need to enable the `alloc`
//! feature.
//!
//! [`GlobalAlloc`]: core::alloc::GlobalAlloc
//! [global_alloc_docs]: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html
//!
//! # How the `alloc` Allocator Works
//!
//! The `alloc` allocator is a simple bump-pointer (arena) allocator. Each call to
//! `alloc` advances a cursor through Wasm linear memory, growing the memory as
//! needed. **`dealloc` is a no-op** — memory is never freed during contract
//! execution. All allocations made during an invocation persist until the host
//! destroys the VM instance at the end of the invocation.
//!
//! This design is a good fit for Soroban contracts because each invocation runs
//! to completion and then the entire VM is discarded. There is no long-lived
//! process that would benefit from returning memory to the allocator.

#[cfg(target_family = "wasm")]
mod bump_pointer;
