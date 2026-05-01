//! Generates the emit-side artifacts used by spec shaking v2.
//!
//! SDK macros use these helpers to emit root markers for events and thrown
//! errors, exact `SpecTypeId` implementations for UDTs, and removable graph
//! records in the private `contractspecv0.rssdk.graphv0` sidecar section.
//!
//! See `soroban-spec-markers` for the marker and graph wire formats, and
//! `soroban-spec` for the post-build reachability filter and Wasm rewriting.

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
/// Emits a `pub static [u8; LEN]` initialized by `encode_graph_record`, wired into
/// `contractspecv0.rssdk.graphv0` via `link_section`.
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
    use soroban_spec::shaking::{filter, SpecGraph, SpecGraphEntryKind};
    use std::collections::HashSet;
    use syn::{parse_quote, Type};

    use stellar_xdr::curr::{
        ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeMap, ScSpecTypeResult,
        ScSpecTypeUdt, ScSpecTypeVec, ScSpecUdtStructV0, StringM, VecM,
    };

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

    #[test]
    fn type_id_refs_match_strict_spec_graph_validation() {
        let path: Path = parse_quote!(soroban_sdk);
        let rust_ty: Type = parse_quote!(Result<Vec<Foo>, Map<u32, Bar>>);
        let macro_ref_names = type_id_refs(&path, &rust_ty)
            .into_iter()
            .map(type_id_ref_name)
            .collect::<Vec<_>>();
        assert_eq!(macro_ref_names, vec!["Foo", "Bar"]);

        let function = make_function(
            "foo",
            ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                    element_type: Box::new(udt("Foo")),
                }))),
                error_type: Box::new(ScSpecTypeDef::Map(Box::new(ScSpecTypeMap {
                    key_type: Box::new(ScSpecTypeDef::U32),
                    value_type: Box::new(udt("Bar")),
                }))),
            })),
        );
        let foo = make_struct("Foo");
        let bar = make_struct("Bar");
        let entries = vec![function.clone(), foo.clone(), bar.clone()];
        let graph_refs = macro_ref_names
            .iter()
            .map(|name| match name.as_str() {
                "Foo" => &foo,
                "Bar" => &bar,
                _ => panic!("unexpected macro ref {name}"),
            })
            .collect::<Vec<_>>();
        let graph = SpecGraph::from_records([
            (&function, SpecGraphEntryKind::Function, graph_refs),
            (&foo, SpecGraphEntryKind::Udt, vec![]),
            (&bar, SpecGraphEntryKind::Udt, vec![]),
        ]);

        let filtered = filter(entries.clone(), &HashSet::new(), &graph)
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(filtered, entries);
    }

    fn type_id_ref_name(tokens: TokenStream2) -> String {
        let tokens = tokens.to_string();
        tokens
            .split(" as ")
            .next()
            .expect("type id ref should contain a type")
            .trim()
            .trim_start_matches('<')
            .trim()
            .replace(' ', "")
    }

    fn udt(name: &str) -> ScSpecTypeDef {
        ScSpecTypeDef::Udt(ScSpecTypeUdt {
            name: name.try_into().unwrap(),
        })
    }

    fn make_function(name: &str, input_type: ScSpecTypeDef) -> ScSpecEntry {
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: StringM::default(),
            name: name.try_into().unwrap(),
            inputs: vec![ScSpecFunctionInputV0 {
                doc: StringM::default(),
                name: "arg0".try_into().unwrap(),
                type_: input_type,
            }]
            .try_into()
            .unwrap(),
            outputs: VecM::default(),
        })
    }

    fn make_struct(name: &str) -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: StringM::default(),
            lib: StringM::default(),
            name: name.try_into().unwrap(),
            fields: VecM::default(),
        })
    }
}
