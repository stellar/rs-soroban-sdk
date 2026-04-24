//! Generates the `SpecShakingMarker` impl for contract types.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! Each type gets two module-level `#[repr(packed)]` statics next to the
//! type: a `BYTES` static holding the marker bytes plus a back-ref to a
//! `DEPS` static, and a `DEPS` static holding an array of references to
//! each field type's marker (via the trait's `SPEC_SHAKING_MARKER_REF`
//! associated constant). Keeping `BYTES` live transitively keeps `DEPS`
//! live, which keeps each field type's marker live, and so on.
//!
//! Two statics are needed to support self-recursive contract types (e.g.
//! `struct T { children: Vec<T> }`). See `generate_marker_impl` for more.
//!
//! The type's `spec_shaking_marker()` fn does a single volatile read through
//! a pointer to the `BYTES` static — no recursive fn calls into field types
//! are needed, because the statics already reference them. When the type is
//! unused at a boundary, the fn is DCE'd, nothing references the statics,
//! and the linker strips the entire chain.
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
/// module-level marker static.
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
/// A `TokenStream2` containing the marker struct type, the module-level
/// marker static, and the `impl SpecShakingMarker for Type { ... }` block.
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

    let ident_str = ident.to_string();
    let bytes_static_ident = format_ident!("__SPEC_SHAKING_MARKER_{}", ident_str.to_uppercase());
    let deps_static_ident =
        format_ident!("__SPEC_SHAKING_MARKER_DEPS_{}", ident_str.to_uppercase());
    let bytes_struct_ident = format_ident!("__SpecShakingMarkerOf{}", ident_str);
    let deps_struct_ident = format_ident!("__SpecShakingMarkerDepsOf{}", ident_str);

    let field_marker_refs: Vec<_> = field_types
        .map(|ty| {
            let stripped = strip_lifetimes(ty);
            quote! { <#stripped as #path::SpecShakingMarker>::SPEC_SHAKING_MARKER_REF }
        })
        .collect();

    let field_count = field_marker_refs.len();

    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    // Why two statics?
    //
    // Naively we'd emit one `static` per type holding both the marker bytes
    // and an array of refs to each field type's marker. But for self-
    // recursive contract types (e.g. `struct T { children: Vec<T> }`), the
    // array needs `<Vec<T> as SpecShakingMarker>::SPEC_SHAKING_MARKER_REF`,
    // which resolves to `<T>::SPEC_SHAKING_MARKER_REF`, defined as a ref
    // into the same static — a const-eval cycle.
    //
    // To support recursive types we always split into two statics with
    // mutual references:
    //
    //   `BYTES`: marker bytes + back-ref to `DEPS`.
    //   `DEPS`:  array of field marker refs.
    //
    // The assoc const is `&BYTES.marker`. When evaluating `DEPS`'s init,
    // any inner `<T>::SPEC_SHAKING_MARKER_REF` resolves to a ref into
    // `BYTES` (a different static), so no self-cycle.
    //
    // For DCE transitivity: another type's `DEPS` array contains
    // `&BYTES.marker` for this type. Keeping that ref live keeps `BYTES`
    // live; `BYTES`'s back-ref to `DEPS` keeps this type's own `DEPS` live;
    // its array's refs keep the next layer's `BYTES` live; and so on.
    quote! {
        #[doc(hidden)]
        #[repr(packed)]
        pub struct #bytes_struct_ident {
            pub marker: [u8; #marker_len],
            pub deps: &'static #deps_struct_ident,
        }

        #[doc(hidden)]
        #[repr(packed)]
        pub struct #deps_struct_ident {
            pub fields: [&'static [u8]; #field_count],
        }

        #[doc(hidden)]
        pub static #bytes_static_ident: #bytes_struct_ident = #bytes_struct_ident {
            marker: *#marker_lit,
            deps: &#deps_static_ident,
        };

        #[doc(hidden)]
        pub static #deps_static_ident: #deps_struct_ident = #deps_struct_ident {
            fields: [ #( #field_marker_refs, )* ],
        };

        impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
            const SPEC_SHAKING_MARKER_REF: &'static [u8] = &#bytes_static_ident.marker;

            #[doc(hidden)]
            #[inline(always)]
            fn spec_shaking_marker() {
                #[cfg(target_family = "wasm")]
                {
                    // Volatile read prevents DCE of this function and keeps
                    // the bytes static live. The bytes static's `deps`
                    // back-ref keeps the deps static live, which transitively
                    // keeps every referenced field marker live.
                    let _ = unsafe {
                        ::core::ptr::read_volatile(
                            &#bytes_static_ident as *const _ as *const u8,
                        )
                    };
                }
            }
        }
    }
}
