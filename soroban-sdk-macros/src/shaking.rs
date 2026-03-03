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
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip unused specs from contractspecv0

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, Path, Type};

/// Generates the spec XDR static, `spec_xdr()` const fn, and optional
/// `SpecShakingMarker` impl for a contract type.
///
/// Returns `(spec_gen, spec_shaking_impl)` where:
/// - `spec_gen` contains the `__SPEC_XDR_TYPE_*` static and `spec_xdr()` method
/// - `spec_shaking_impl` contains the `SpecShakingMarker` impl (only when the
///   `experimental_spec_shaking_v2` feature is enabled)
///
/// Both are `None` when `spec_xdr` is `None` (i.e., spec generation is disabled).
pub fn generate_type_spec_and_marker(
    path: &Path,
    ident: &Ident,
    spec_xdr: &Option<Vec<u8>>,
    field_types: &[&Type],
) -> (Option<TokenStream2>, Option<TokenStream2>) {
    let spec_gen = spec_xdr.as_ref().map(|spec_xdr| {
        let spec_xdr_lit = proc_macro2::Literal::byte_string(spec_xdr.as_slice());
        let spec_xdr_len = spec_xdr.len();
        let spec_ident = format_ident!("__SPEC_XDR_TYPE_{}", ident.to_string().to_uppercase());
        quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #ident::spec_xdr();

            impl #ident {
                pub const fn spec_xdr() -> [u8; #spec_xdr_len] {
                    *#spec_xdr_lit
                }
            }
        }
    });

    let spec_shaking_impl = if cfg!(feature = "experimental_spec_shaking_v2") {
        spec_xdr.as_ref().map(|spec_xdr| {
            generate_marker_impl(
                path,
                quote!(#ident),
                spec_xdr,
                field_types.iter().copied(),
                None,
                None,
                None,
            )
        })
    } else {
        None
    };

    (spec_gen, spec_shaking_impl)
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
    spec_xdr: &[u8],
    field_types: I,
    gen_impl: Option<TokenStream2>,
    gen_types: Option<TokenStream2>,
    gen_where: Option<TokenStream2>,
) -> TokenStream2
where
    I: Iterator<Item = &'a Type>,
{
    let marker = soroban_spec::shaking::generate_marker_for_xdr(spec_xdr);
    let marker_lit = proc_macro2::Literal::byte_string(&marker);
    let marker_len = marker.len();

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
                    static MARKER: [u8; #marker_len] = *#marker_lit;
                    // Volatile read prevents DCE within live function.
                    let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
                }
            }
        }
    }
}
