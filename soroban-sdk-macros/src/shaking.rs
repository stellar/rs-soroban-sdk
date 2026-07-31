//! Generates the `spec_id` const fn and the `SpecShakingMarker` impl for
//! contract types.
//!
//! Every type gets a `spec_id()`, which is the first 64 bits of the SHA256 of
//! its spec entry's XDR. The marker is a byte array in the data section with a
//! distinctive pattern, built from that id:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: the type's `spec_id()`
//!
//! Markers are embedded in `spec_shaking_marker()` functions with a volatile read.
//! When the type is used, the function is called and the marker is included.
//! When the type is unused, the function is DCE'd along with its marker.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip unused specs from contractspecv0

use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{Path, Type};

/// Length of a spec id: the truncated SHA256 that a marker carries after its
/// magic prefix.
const ID_LEN: usize = 8;

/// Generates the `spec_id` const fn for a type.
///
/// The id is the first `ID_LEN` bytes of the SHA256 of the spec entry's XDR. It
/// is the value a marker carries after its magic prefix, and the value
/// post-processing tools match against the entries in `contractspecv0`.
///
/// The hash is taken at const evaluation time over the very bytes the spec
/// entry encodes to, because parts of an entry, such as the fully qualified
/// name of the type, are only known once const evaluation resolves them.
pub fn generate_spec_id(path: &Path) -> TokenStream2 {
    let id_len = ID_LEN;
    let hash_bytes = (0..ID_LEN).map(Literal::usize_unsuffixed);
    quote! {
        pub const fn spec_id() -> [u8; #id_len] {
            let xdr = Self::spec_xdr();
            let hash = #path::reexports_for_macros::sha2_const::Sha256::new()
                .update(&xdr)
                .finalize();
            [#(hash[#hash_bytes]),*]
        }
    }
}

/// Generates the `SpecShakingMarker` impl for a type.
///
/// # Arguments
///
/// * `path` - The crate path (e.g., `soroban_sdk`)
/// * `ident` - The type identifier
/// * `spec_xdr` - The XDR bytes of the spec entry
/// * `field_types` - Optional iterator of field types to include markers for nested types
/// * `gen_impl` - Optional generics impl tokens (e.g., `<T>`)
/// * `gen_types` - Optional generics type tokens (e.g., `<T>`)
/// * `gen_where` - Optional generics where clause
///
/// # Returns
///
/// A `TokenStream2` containing the `impl SpecShakingMarker for Type { ... }` block.
pub fn generate_marker_impl<'a, I>(
    path: &Path,
    ident: TokenStream2,
    field_types: I,
    gen_impl: Option<TokenStream2>,
    gen_types: Option<TokenStream2>,
    gen_where: Option<TokenStream2>,
) -> TokenStream2
where
    I: Iterator<Item = &'a Type>,
{
    // The marker's magic prefix, emitted a byte at a time so the marker can be
    // assembled from the id.
    let magic = soroban_spec::shaking::MAGIC.iter().map(|b| *b);
    let marker_len = soroban_spec::shaking::MAGIC.len() + ID_LEN;
    let id_bytes = (0..ID_LEN).map(Literal::usize_unsuffixed);

    let field_type_markers: Vec<_> = field_types.collect();
    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    quote! {
        impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
            #[doc(hidden)]
            #[inline(always)]
            fn spec_shaking_marker() {
                #(<#field_type_markers as #path::SpecShakingMarker>::spec_shaking_marker();)*
                #[cfg(target_family = "wasm")]
                {
                    // Marker in data section. Post-build tools can scan for "SpEcV1"
                    // patterns and match against specs in contractspecv0.
                    //
                    // The type is named rather than reached through `Self`
                    // because a static is an item, and an item does not inherit
                    // `Self` from the scope it is written in.
                    static MARKER: [u8; #marker_len] = {
                        let id = #ident::spec_id();
                        [#(#magic,)* #(id[#id_bytes]),*]
                    };
                    // Volatile read prevents DCE of this function and keeps MARKER
                    // in the data section. We only read a single `u8` from the start
                    // of the array because merely taking a volatile reference to the
                    // symbol is sufficient; reading all bytes via
                    // `read_volatile::<[u8; #marker_len]>()` would be redundant and
                    // could increase code size without any functional benefit.
                    let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
                }
            }
        }
    }
}
