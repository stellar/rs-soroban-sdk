//! Generates root marker blocks for spec shaking.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! Markers are embedded at roots that cannot be derived from `contractspecv0`,
//! currently event publish methods and errors thrown via `panic_with_error!` or
//! `assert_with_error!`. Function input/output roots are derived from function
//! entries in `contractspecv0`, and UDT reachability is discovered by exact spec
//! IDs in removable sidecar graph records.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Keep marked entries and all functions
//! 4. Read the removable sidecar graph when present
//! 5. Walk UDT references from those roots
//! 6. Strip unused specs from contractspecv0 and drop the sidecar graph

use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use stellar_xdr::curr::ScSpecTypeDef;
use syn::{ext::IdentExt as _, GenericArgument, Ident, Path, PathArguments, Type, TypeReference};

use crate::map_type::map_type;

/// Generates a marker block for a spec entry root.
///
/// # Returns
///
/// A `TokenStream2` containing a marker static and volatile read.
pub fn generate_marker_block(spec_xdr: &[u8]) -> TokenStream2 {
    let marker = soroban_spec_markers::generate_marker_for_xdr(spec_xdr);
    let marker_lit = proc_macro2::Literal::byte_string(&marker);
    let marker_len = marker.len();

    quote! {
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; #marker_len] = *#marker_lit;
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}

/// Generates a call-site hook for an error spec root.
///
/// The hook emits the same `SpEcV1` marker block used by events. It roots only
/// the error entry; referenced types are retained through the spec graph.
pub fn generate_error_marker_impl(path: &Path, ident: &Ident, spec_xdr: &[u8]) -> TokenStream2 {
    let marker_block = generate_marker_block(spec_xdr);

    quote! {
        impl #path::spec_shaking::SpecShakingMarker for #ident {
            #[doc(hidden)]
            #[inline(always)]
            fn spec_shaking_marker() {
                #marker_block
            }
        }
    }
}

/// Generates an implementation of the hidden exact spec identity trait for a UDT.
pub fn generate_type_id_impl(path: &Path, ident: &Ident, spec_xdr: &[u8]) -> TokenStream2 {
    let spec_id = soroban_spec_markers::generate_spec_id_for_xdr(spec_xdr);
    let spec_id_lit = proc_macro2::Literal::byte_string(&spec_id);

    quote! {
        impl #path::spec_shaking::SpecTypeId for #ident {
            const SPEC_TYPE_ID: [u8; 32] = *#spec_id_lit;
        }
    }
}

/// Generates spec-shaking metadata for a UDT.
pub fn generate_udt_shaking(
    path: &Path,
    ident: &Ident,
    spec_xdr: &[u8],
    refs: Vec<TokenStream2>,
    spec_exported: bool,
    generate_marker: bool,
) -> Option<TokenStream2> {
    let type_id_impl = generate_type_id_impl(path, ident, spec_xdr);
    if !spec_exported {
        return Some(type_id_impl);
    }

    let marker_impl = if generate_marker {
        Some(generate_error_marker_impl(path, ident, spec_xdr))
    } else {
        None
    };

    let graph_ident = format_ident!(
        "__SPEC_GRAPH_TYPE_{}",
        ident.unraw().to_string().to_uppercase()
    );
    let graph_record = generate_graph_record(
        path,
        &graph_ident,
        soroban_spec_markers::SpecGraphEntryKind::Udt,
        spec_xdr,
        refs,
    );

    Some(quote! {
        #type_id_impl
        #marker_impl
        #graph_record
    })
}

/// Generates one removable graph record in `contractspecv0.rssdk.graphv0` at compile time.
///
/// This exists as a `const fn` rather than a fully baked byte literal because of how
/// the SDK macros emit graph records. When `derive_*` expands for type `Foo`,
/// it knows `Foo`'s own spec XDR and can hash it — but `Foo`'s referenced
/// UDTs (`Bar`, `Baz`, …) are defined elsewhere, possibly in another crate,
/// and the macro has no visibility into their tokens. So the macro emits each ref as a
/// trait-associated constant expression `<Bar as SpecTypeId>::SPEC_TYPE_ID` and lets
/// const-eval resolve the 32 bytes after all impls are in scope.
pub fn generate_graph_record(
    path: &Path,
    ident: &Ident,
    kind: soroban_spec_markers::SpecGraphEntryKind,
    spec_xdr: &[u8],
    refs: Vec<TokenStream2>,
) -> TokenStream2 {
    let spec_id = soroban_spec_markers::generate_spec_id_for_xdr(spec_xdr);
    let spec_id_lit = proc_macro2::Literal::byte_string(&spec_id);
    let section_lit = proc_macro2::Literal::string(soroban_spec_markers::GRAPH_SECTION);
    let kind_lit = proc_macro2::Literal::u16_unsuffixed(kind.to_u16());
    let ref_count = refs.len();
    assert!(
        ref_count <= u16::MAX as usize,
        "spec graph record cannot encode more than u16::MAX refs"
    );
    let record_len = soroban_spec_markers::graph_record_len(ref_count);

    quote! {
        #[cfg_attr(target_family = "wasm", link_section = #section_lit)]
        #[allow(non_upper_case_globals)]
        pub static #ident: [u8; #record_len] =
            #path::spec_shaking::encode_graph_record::<#record_len, #ref_count>(
                #kind_lit,
                *#spec_id_lit,
                [#(#refs),*],
            );
    }
}

/// Generates exact UDT spec identity expressions referenced by a Rust type.
pub fn type_id_refs(path: &Path, ty: &Type) -> Vec<TokenStream2> {
    // Keep this traversal in sync with `soroban-spec/src/shaking.rs::add_type_def_udt_names`.
    // The post-build validator mirrors these macro-emitted graph refs for each spec container.
    match ty {
        Type::Reference(TypeReference { elem, .. }) => type_id_refs(path, elem),
        Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .flat_map(|ty| type_id_refs(path, ty))
            .collect(),
        Type::Path(type_path) => {
            // Excludes malformed or empty paths. Without a terminal path segment,
            // there is no type constructor to classify or generic arguments to walk.
            let Some(segment) = type_path.path.segments.last() else {
                return Vec::new();
            };
            let ident = segment.ident.unraw().to_string();
            match ident.as_str() {
                "Option" | "Vec" => generic_type_args(segment)
                    .into_iter()
                    .take(1)
                    .flat_map(|ty| type_id_refs(path, ty))
                    .collect(),
                "Map" | "Result" => generic_type_args(segment)
                    .into_iter()
                    .take(2)
                    .flat_map(|ty| type_id_refs(path, ty))
                    .collect(),
                _ if should_emit_type_id_ref(ty) => {
                    vec![quote! { <#ty as #path::spec_shaking::SpecTypeId>::SPEC_TYPE_ID }]
                }
                _ => Vec::new(),
            }
        }
        // Excludes non-path type syntax such as arrays, slices, bare function
        // pointers, `impl Trait`, and inferred types. Contract specs either do
        // not accept these forms here or reject them through `map_type` when the
        // surrounding spec entry is built.
        _ => Vec::new(),
    }
}

fn should_emit_type_id_ref(ty: &Type) -> bool {
    // `map_type` is the canonical Rust-type-to-spec-type mapper. Only UDT-shaped
    // spec types need an exact graph ref; all SDK/builtin mappings return a
    // concrete non-UDT spec type. Under v2, even `export = false` contract types
    // get a hidden `SpecTypeId`, so SDK boundary types such as `auth::Context`
    // and user-defined UDTs named `Context` can be handled uniformly.
    matches!(map_type(ty, true, true), Ok(ScSpecTypeDef::Udt(_)))
}

fn generic_type_args(segment: &syn::PathSegment) -> Vec<&Type> {
    // Excludes non-generic path segments. Plain UDTs and concrete builtin types
    // have no child type arguments for the graph walker to recurse into.
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return Vec::new();
    };
    args.args
        .iter()
        .filter_map(|arg| {
            if let GenericArgument::Type(ty) = arg {
                Some(ty)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::{parse_quote, Type};

    fn refs_for(ty: Type) -> Vec<String> {
        let path: Path = parse_quote!(soroban_sdk);
        type_id_refs(&path, &ty)
            .into_iter()
            .map(|tokens| tokens.to_string())
            .collect()
    }

    #[test]
    fn type_id_refs_skip_types_mapped_by_map_type_to_builtins() {
        for ty in [
            parse_quote!(Val),
            parse_quote!(u64),
            parse_quote!(i64),
            parse_quote!(u32),
            parse_quote!(i32),
            parse_quote!(u128),
            parse_quote!(i128),
            parse_quote!(U256),
            parse_quote!(I256),
            parse_quote!(bool),
            parse_quote!(Symbol),
            parse_quote!(String),
            parse_quote!(Error),
            parse_quote!(Bytes),
            parse_quote!(Address),
            parse_quote!(MuxedAddress),
            parse_quote!(Timepoint),
            parse_quote!(Duration),
            parse_quote!(BytesN<32>),
            parse_quote!(Hash<32>),
            parse_quote!(Fp),
            parse_quote!(Fp2),
            parse_quote!(G1Affine),
            parse_quote!(G2Affine),
            parse_quote!(Fr),
            parse_quote!(Bls12381Fp),
            parse_quote!(Bls12381Fp2),
            parse_quote!(Bls12381G1Affine),
            parse_quote!(Bls12381G2Affine),
            parse_quote!(Bls12381Fr),
            parse_quote!(Bn254Fp),
            parse_quote!(Bn254G1Affine),
            parse_quote!(Bn254G2Affine),
            parse_quote!(Bn254Fr),
            parse_quote!(BnScalar),
            parse_quote!(Vec<Bn254G2Affine>),
        ] {
            assert!(refs_for(ty).is_empty());
        }
    }

    #[test]
    fn type_id_refs_emit_context_like_udts() {
        let refs = refs_for(parse_quote!(Vec<Context>));

        assert_eq!(refs.len(), 1);
        assert!(refs[0].contains("Context"));
        assert!(refs[0].contains("SpecTypeId"));
    }

    #[test]
    fn type_id_refs_emit_udts_recursively() {
        let refs = refs_for(parse_quote!(Map<Address, Option<MyType>>));

        assert_eq!(refs.len(), 1);
        assert!(refs[0].contains("MyType"));
        assert!(refs[0].contains("SpecTypeId"));
    }
}
