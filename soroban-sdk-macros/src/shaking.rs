//! Generates the `SpecShakingMarker` impl for contract types.
//!
//! For each type, the macro emits:
//! - a module-scope `static` holding a [`MarkerNode`] with the type's 14-byte
//!   `SpEcV1…` marker and `*const MarkerNode` pointers to each field's
//!   `MARKER_NODE`;
//! - an `impl SpecShakingMarker` whose `MARKER_NODE` const returns a raw
//!   pointer to that static (via `&raw const`).
//!
//! Raw pointers are used throughout the graph (rather than `&'static` refs)
//! to sidestep rustc's const-evaluation cycle detector: CTFE of a `&STATIC`
//! expression triggers evaluation of `STATIC`'s initializer, which fails on
//! recursive user types like `struct S { v: Vec<S> }` (S's init depends on
//! Vec<S>'s init which depends on S's init). CTFE of a `*const STATIC`
//! (via `&raw const STATIC`) does not walk into the initializer. Runtime
//! behaviour is identical — addresses resolve at link time the same way.

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{visit_mut::VisitMut, Ident, Lifetime, Path, Type};

/// Replace every lifetime in field types with `'static`.
///
/// Rust does not allow the module-scope static we emit to reference
/// lifetimes declared on the type's generics (e.g. `<'a>` on an event
/// struct with `&'a Field` fields). Every `&'a T: SpecShakingMarker` impl
/// forwards to `T::MARKER_NODE`, so substituting `'static` for `'a` in the
/// field type produces an identical `MARKER_NODE` value while letting the
/// static typecheck at module scope.
struct LifetimeToStatic;
impl VisitMut for LifetimeToStatic {
    fn visit_lifetime_mut(&mut self, lt: &mut Lifetime) {
        lt.ident = Ident::new("static", lt.ident.span());
    }
}

/// Generates the `SpecShakingMarker` impl for a type.
///
/// # Arguments
///
/// * `path` - The crate path (e.g., `soroban_sdk`)
/// * `ident` - The type identifier
/// * `spec_xdr` - The XDR bytes of the spec entry
/// * `field_types` - Iterator of field types whose markers this type reaches
/// * `gen_impl` - Optional generics impl tokens (e.g., `<T>`)
/// * `gen_types` - Optional generics type tokens (e.g., `<T>`)
/// * `gen_where` - Optional generics where clause
///
/// # Returns
///
/// A `TokenStream2` containing a module-scope `static` plus the `impl
/// SpecShakingMarker for Type { ... }` block.
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

    // Substitute 'static for all lifetimes so the static can live at module scope.
    let field_type_markers: Vec<Type> = field_types
        .map(|t| {
            let mut t = t.clone();
            LifetimeToStatic.visit_type_mut(&mut t);
            t
        })
        .collect();

    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    // Derive a stable per-type static identifier from a hash of the marker
    // (which itself hashes the spec entry XDR). Keeps the name readable and
    // collision-unlikely across types in the same crate.
    let static_ident = format_ident!(
        "__SOROBAN_SDK_SPEC_MARKER_NODE_{}",
        marker[6..]
            .iter()
            .map(|b| format!("{b:02X}"))
            .collect::<String>()
    );

    quote! {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        static #static_ident: #path::MarkerNode = #path::MarkerNode {
            marker: *#marker_lit,
            children: &[
                #(<#field_type_markers as #path::SpecShakingMarker>::MARKER_NODE,)*
            ],
        };

        impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
            const MARKER_NODE: *const #path::MarkerNode = &raw const #static_ident;
        }
    }
}
