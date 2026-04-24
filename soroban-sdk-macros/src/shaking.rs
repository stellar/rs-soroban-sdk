//! Generates the `SpecShakingMarker` impl for contract types.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! The marker is embedded in a module-level `static` tuple that lives next to
//! the type. The first tuple element holds the marker bytes; the remaining
//! elements are references to the marker refs of each field type (via the
//! trait's `SPEC_SHAKING_MARKER_REF` associated constant). Keeping the tuple
//! live therefore transitively keeps all reachable field markers live.
//!
//! The type's `spec_shaking_marker()` fn does a single volatile read through
//! a pointer to the tuple — no recursive fn calls into field types are
//! needed, because the tuple already references them. When the type is
//! unused at a boundary, the fn is DCE'd, nothing references the tuple, and
//! the linker strips the entire chain.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip unused specs from contractspecv0

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{visit_mut::VisitMut, Ident, Lifetime, Path, Type, TypeReference};

/// Rewrites all lifetimes in a type to `'static`, including elided lifetimes
/// on `&T` references. The resulting type is suitable for use at module scope
/// — where generic lifetime parameters (like `'a` from an `impl<'a>`) aren't
/// in scope. The associated constant value being referenced
/// (`SPEC_SHAKING_MARKER_REF`) is lifetime-invariant, so this substitution
/// doesn't affect the resolved const.
struct LifetimesToStatic;
impl VisitMut for LifetimesToStatic {
    fn visit_lifetime_mut(&mut self, lt: &mut Lifetime) {
        lt.ident = Ident::new("static", lt.ident.span());
    }
    fn visit_type_reference_mut(&mut self, tr: &mut TypeReference) {
        if tr.lifetime.is_none() {
            tr.lifetime = Some(Lifetime::new("'static", proc_macro2::Span::call_site()));
        }
        syn::visit_mut::visit_type_reference_mut(self, tr);
    }
}

fn strip_lifetimes(ty: &Type) -> Type {
    let mut ty = ty.clone();
    LifetimesToStatic.visit_type_mut(&mut ty);
    ty
}

/// Generates the `SpecShakingMarker` impl for a type, along with its
/// module-level marker static tuple.
///
/// # Arguments
///
/// * `path` - The crate path (e.g., `soroban_sdk`)
/// * `ident` - The type identifier
/// * `spec_xdr` - The XDR bytes of the spec entry
/// * `field_types` - Iterator of field types to include marker refs for
/// * `gen_impl` - Optional generics impl tokens (e.g., `<'a>`)
/// * `gen_types` - Optional generics type tokens (e.g., `<'a>`)
/// * `gen_where` - Optional generics where clause
///
/// # Returns
///
/// A `TokenStream2` containing the module-level marker static plus the
/// `impl SpecShakingMarker for Type { ... }` block.
pub fn generate_marker_impl<'a, I>(
    path: &Path,
    ident: &Ident,
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

    let static_ident = format_ident!("__SPEC_SHAKING_MARKER_{}", ident.to_string().to_uppercase());

    let field_marker_refs: Vec<_> = field_types
        .map(|ty| {
            let stripped = strip_lifetimes(ty);
            quote! { <#stripped as #path::SpecShakingMarker>::SPEC_SHAKING_MARKER_REF }
        })
        .collect();

    let field_marker_ref_types = field_marker_refs.iter().map(|_| quote!(&'static [u8]));

    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    quote! {
        #[doc(hidden)]
        pub static #static_ident: (
            [u8; #marker_len],
            #( #field_marker_ref_types, )*
        ) = (
            *#marker_lit,
            #( #field_marker_refs, )*
        );

        impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
            const SPEC_SHAKING_MARKER_REF: &'static [u8] = &#static_ident.0;

            #[doc(hidden)]
            #[inline(always)]
            fn spec_shaking_marker() {
                #[cfg(target_family = "wasm")]
                {
                    // Volatile read prevents DCE of this function and keeps
                    // the marker tuple (and its transitively-referenced
                    // markers) in the data section.
                    let _ = unsafe {
                        ::core::ptr::read_volatile(
                            &#static_ident as *const _ as *const u8,
                        )
                    };
                }
            }
        }
    }
}
