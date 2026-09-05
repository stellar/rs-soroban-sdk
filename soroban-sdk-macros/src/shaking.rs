//! Generates the `SpecShakingMarker` impl for contract types.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! Markers are embedded in `spec_shaking_marker()` functions with a volatile read.
//! When the type is used, the function is called and the marker is included.
//! When the type is unused, the function is DCE'd along with its marker.
//!
//! Only the entries that nothing in a spec references by name carry a marker:
//! events, and error enums, which a contract may use solely by handing them to
//! `panic_with_error!`. Every other user-defined type is named by whatever
//! references it, so post-processing tools settle those by walking references
//! from the contract's functions, and the type needs no marker of its own.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip unreachable and unmarked specs from contractspecv0

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Path;

/// Generates a `SpecShakingMarker` impl for a type that emits a marker.
///
/// # Arguments
///
/// * `path` - The crate path (e.g., `soroban_sdk`)
/// * `ident` - The type identifier
/// * `spec_xdr` - The XDR bytes of the spec entry
/// * `gen_impl` - Optional generics impl tokens (e.g., `<T>`)
/// * `gen_types` - Optional generics type tokens (e.g., `<T>`)
/// * `gen_where` - Optional generics where clause
///
/// # Returns
///
/// A `TokenStream2` containing the `impl SpecShakingMarker for Type { ... }` block.
pub fn generate_marker_impl(
    path: &Path,
    ident: TokenStream2,
    spec_xdr: TokenStream2,
    gen_impl: Option<TokenStream2>,
    gen_types: Option<TokenStream2>,
    gen_where: Option<TokenStream2>,
) -> TokenStream2 {
    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    quote! {
        impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
            #[doc(hidden)]
            #[inline(always)]
            fn spec_shaking_marker() {
                #[cfg(target_family = "wasm")]
                {
                    // Marker in data section. Post-build tools can scan for "SpEcV1"
                    // patterns and match against specs in contractspecv0. Built from
                    // the same const-encoded XDR that is embedded in that section, so
                    // the two cannot drift apart.
                    static MARKER: [u8; 14] =
                        #path::reexports_for_macros::soroban_spec::shaking::generate_marker_for_xdr(&#spec_xdr);
                    // Volatile read prevents DCE of this function and keeps MARKER
                    // in the data section. We only read a single `u8` from the start
                    // of the array because merely taking a volatile reference to the
                    // symbol is sufficient; reading all bytes via
                    // `read_volatile::<[u8; 14]>()` would be redundant and
                    // could increase code size without any functional benefit.
                    let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
                }
            }
        }
    }
}
