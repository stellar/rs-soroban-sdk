//! Generates the `SpecShakingMarker` impl for contract types.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! The marker is embedded in a module-level `static` of a per-type
//! `#[repr(packed)]` struct that lives next to the type. The struct's first
//! field holds the marker bytes; subsequent fields are references to the
//! marker refs of each field type (via the trait's `SPEC_SHAKING_MARKER_REF`
//! associated constant). Keeping the static live therefore transitively
//! keeps all reachable field markers live.
//!
//! The type's `spec_shaking_marker()` fn does a single volatile read through
//! a pointer to the static — no recursive fn calls into field types are
//! needed, because the static already references them. When the type is
//! unused at a boundary, the fn is DCE'd, nothing references the static,
//! and the linker strips the entire chain.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Match against specs in contractspecv0 section (by hashing each spec)
//! 4. Strip unused specs from contractspecv0

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{visit::Visit, visit_mut::VisitMut, Ident, Lifetime, Path, Type, TypeReference};

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

/// Returns true if any field type syntactically references `ident`. Catches
/// direct self-recursion like `struct T { children: Vec<T> }`. Used to decide
/// whether to emit the recursion-safe two-static layout for this type.
fn has_self_reference<'a, I: IntoIterator<Item = &'a Type>>(ident: &Ident, field_types: I) -> bool {
    struct RefSearch<'a> {
        target: &'a Ident,
        found: bool,
    }
    impl<'ast> Visit<'ast> for RefSearch<'_> {
        fn visit_ident(&mut self, i: &'ast Ident) {
            if i == self.target {
                self.found = true;
            }
        }
    }
    let mut s = RefSearch {
        target: ident,
        found: false,
    };
    for ty in field_types {
        s.visit_type(ty);
        if s.found {
            return true;
        }
    }
    false
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
    let static_ident = format_ident!("__SPEC_SHAKING_MARKER_{}", ident_str.to_uppercase());
    let struct_ident = format_ident!("__SpecShakingMarkerOf{}", ident_str);

    // Materialize field types up-front so we can both detect self-reference
    // and emit marker ref expressions without consuming the iterator twice.
    let field_tys: Vec<Type> = field_types.cloned().collect();
    let self_recursive = has_self_reference(ident, field_tys.iter());

    let field_marker_refs: Vec<_> = field_tys
        .iter()
        .map(|ty| {
            let stripped = strip_lifetimes(ty);
            quote! { <#stripped as #path::SpecShakingMarker>::SPEC_SHAKING_MARKER_REF }
        })
        .collect();

    let field_count = field_marker_refs.len();

    let gen_impl = gen_impl.unwrap_or_default();
    let gen_types = gen_types.unwrap_or_default();
    let gen_where = gen_where.unwrap_or_default();

    // For non-recursive types we emit a single static per type:
    //
    //   ```
    //   static MARKER = Struct { marker: [...], fields: [refs...] };
    //   impl Trait for T { const REF = &MARKER.marker; }
    //   ```
    //
    // For self-recursive types (e.g. `struct T { children: Vec<T> }`), the
    // single-static form creates a const-eval cycle: evaluating the
    // static's `fields` array needs `<Vec<T>>::SPEC_SHAKING_MARKER_REF`,
    // which resolves to `<T>::SPEC_SHAKING_MARKER_REF`, which is defined
    // as a ref into that same static. To break the cycle we split into
    // two statics with mutual references:
    //
    //   `BYTES`: marker bytes + back-ref to `DEPS`.
    //   `DEPS`:  the array of field marker refs.
    //
    // The assoc const is `&BYTES.marker`. When evaluating `DEPS`'s init,
    // inner `<T>::SPEC_SHAKING_MARKER_REF` resolves to a ref into `BYTES`
    // (a different static), so no self-cycle.
    //
    // The split costs an extra 4-byte back-ref + an extra static symbol
    // per recursive type; most contract types are non-recursive and use
    // the compact single-static form.
    if self_recursive {
        let deps_static_ident =
            format_ident!("__SPEC_SHAKING_MARKER_DEPS_{}", ident_str.to_uppercase());
        let deps_struct_ident = format_ident!("__SpecShakingMarkerDepsOf{}", ident_str);
        quote! {
            #[doc(hidden)]
            #[repr(packed)]
            pub struct #struct_ident {
                pub marker: [u8; #marker_len],
                pub deps: &'static #deps_struct_ident,
            }

            #[doc(hidden)]
            #[repr(packed)]
            pub struct #deps_struct_ident {
                pub fields: [&'static [u8]; #field_count],
            }

            #[doc(hidden)]
            pub static #static_ident: #struct_ident = #struct_ident {
                marker: *#marker_lit,
                deps: &#deps_static_ident,
            };

            #[doc(hidden)]
            pub static #deps_static_ident: #deps_struct_ident = #deps_struct_ident {
                fields: [ #( #field_marker_refs, )* ],
            };

            impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
                const SPEC_SHAKING_MARKER_REF: &'static [u8] = &#static_ident.marker;

                #[doc(hidden)]
                #[inline(always)]
                fn spec_shaking_marker() {
                    #[cfg(target_family = "wasm")]
                    {
                        let _ = unsafe {
                            ::core::ptr::read_volatile(
                                &#static_ident as *const _ as *const u8,
                            )
                        };
                    }
                }
            }
        }
    } else {
        quote! {
            #[doc(hidden)]
            #[repr(packed)]
            pub struct #struct_ident {
                pub marker: [u8; #marker_len],
                pub fields: [&'static [u8]; #field_count],
            }

            #[doc(hidden)]
            pub static #static_ident: #struct_ident = #struct_ident {
                marker: *#marker_lit,
                fields: [ #( #field_marker_refs, )* ],
            };

            impl #gen_impl #path::SpecShakingMarker for #ident #gen_types #gen_where {
                const SPEC_SHAKING_MARKER_REF: &'static [u8] = &#static_ident.marker;

                #[doc(hidden)]
                #[inline(always)]
                fn spec_shaking_marker() {
                    #[cfg(target_family = "wasm")]
                    {
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
}
