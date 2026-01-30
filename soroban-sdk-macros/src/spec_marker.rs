//! Generates the `IncludeSpecMarker` impl for contract types.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 4 bytes: "SpEc" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! Markers are embedded in `include_spec_marker()` functions with a volatile read.
//! When the type is used, the function is called and the marker is included.
//! When the type is unused, the function is DCE'd along with its marker.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEc" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip unused specs from contractspecv0

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Path, Type};

/// Generates the `IncludeSpecMarker` impl for a type.
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
/// A `TokenStream2` containing the `impl IncludeSpecMarker for Type { ... }` block.
pub fn generate_include_spec_marker_impl<'a, I>(
    path: &Path,
    ident: TokenStream2,
    spec_xdr: &[u8],
    field_types: I,
    gen_impl: Option<TokenStream2>,
    gen_types: Option<TokenStream2>,
    gen_where: Option<TokenStream2>,
) -> TokenStream2
where
    I: Iterator<Item = &'a Type>,
{
    let marker = soroban_spec::marker::generate_for_xdr(spec_xdr);
    let marker_lit = proc_macro2::Literal::byte_string(&marker);
    let marker_len = marker.len();

    let field_type_markers: Vec<_> = field_types.collect();
    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    quote! {
        impl #gen_impl #path::IncludeSpecMarker for #ident #gen_types #gen_where {
            #[doc(hidden)]
            #[inline(always)]
            fn include_spec_marker() {
                #(<#field_type_markers as #path::IncludeSpecMarker>::include_spec_marker();)*
                #[cfg(target_family = "wasm")]
                {
                    // Marker in data section. Post-build tools can scan for "SpEc"
                    // patterns and match against specs in contractspecv0.
                    static MARKER: [u8; #marker_len] = *#marker_lit;
                    // Volatile read prevents DCE within live function.
                    let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
                }
            }
        }
    }
}
