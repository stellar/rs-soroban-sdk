//! Generates root marker blocks for spec shaking.
//!
//! The marker is a byte array in the data section with a distinctive pattern:
//! - 6 bytes: "SpEcV1" prefix
//! - 8 bytes: first 64 bits of SHA256 hash of the spec entry XDR
//!
//! Markers are embedded at roots that cannot be derived from `contractspecv0`,
//! currently event publish methods. Function input/output roots are derived from
//! function entries in `contractspecv0`, and UDT reachability is discovered by
//! exact spec IDs in removable sidecar graph records.
//!
//! Post-processing tools (e.g. stellar-cli) can:
//! 1. Scan the WASM data section for "SpEcV1" patterns
//! 2. Extract the hash from each marker
//! 3. Keep marked events and all functions
//! 4. Read the removable sidecar graph when present
//! 5. Walk UDT references from those roots
//! 6. Strip unused specs from contractspecv0 and drop the sidecar graph

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{GenericArgument, Ident, Path, PathArguments, Type, TypeReference};

/// Generates a marker block for a spec entry root.
///
/// # Returns
///
/// A `TokenStream2` containing a marker static and volatile read.
pub fn generate_marker_block(spec_xdr: &[u8]) -> TokenStream2 {
    let marker = soroban_spec::shaking::generate_marker_for_xdr(spec_xdr);
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

/// Generates an implementation of the hidden exact spec identity trait for a UDT.
pub fn generate_type_id_impl(path: &Path, ident: &Ident, spec_xdr: &[u8]) -> TokenStream2 {
    let spec_id = soroban_spec::shaking::generate_spec_id_for_xdr(spec_xdr);
    let spec_id_lit = proc_macro2::Literal::byte_string(&spec_id);

    quote! {
        impl #path::spec_shaking::SpecTypeId for #ident {
            const SPEC_TYPE_ID: [u8; 32] = *#spec_id_lit;
        }
    }
}

/// Generates one removable graph record in `contractspecv0.rssdk.graphv0`.
pub fn generate_graph_record(
    path: &Path,
    ident: &Ident,
    kind: TokenStream2,
    spec_xdr: &[u8],
    refs: Vec<TokenStream2>,
) -> TokenStream2 {
    let spec_id = soroban_spec::shaking::generate_spec_id_for_xdr(spec_xdr);
    let spec_id_lit = proc_macro2::Literal::byte_string(&spec_id);
    let ref_count = refs.len();

    quote! {
        #[cfg_attr(target_family = "wasm", link_section = "contractspecv0.rssdk.graphv0")]
        #[allow(non_upper_case_globals)]
        pub static #ident: #path::spec_shaking::SpecGraphRecord<#ref_count> =
            #path::spec_shaking::SpecGraphRecord::new(#kind, *#spec_id_lit, [#(#refs),*]);
    }
}

/// Generates exact UDT spec identity expressions referenced by a Rust type.
pub fn type_id_refs(path: &Path, ty: &Type) -> Vec<TokenStream2> {
    match ty {
        Type::Reference(TypeReference { elem, .. }) => type_id_refs(path, elem),
        Type::Tuple(tuple) => tuple
            .elems
            .iter()
            .flat_map(|ty| type_id_refs(path, ty))
            .collect(),
        Type::Path(type_path) => {
            let Some(segment) = type_path.path.segments.last() else {
                return Vec::new();
            };
            let ident = segment.ident.to_string();
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
                ident if is_builtin_type_ident(ident) => Vec::new(),
                _ => vec![quote! { <#ty as #path::spec_shaking::SpecTypeId>::SPEC_TYPE_ID }],
            }
        }
        _ => Vec::new(),
    }
}

fn generic_type_args(segment: &syn::PathSegment) -> Vec<&Type> {
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

fn is_builtin_type_ident(ident: &str) -> bool {
    matches!(
        ident,
        "Val"
            | "u64"
            | "i64"
            | "u32"
            | "i32"
            | "u128"
            | "i128"
            | "U256"
            | "I256"
            | "bool"
            | "Symbol"
            | "String"
            | "Error"
            | "Bytes"
            | "BytesN"
            | "Address"
            | "MuxedAddress"
            | "Timepoint"
            | "Duration"
            | "Context"
            | "Hash"
            | "Fp"
            | "Fp2"
            | "G1Affine"
            | "G2Affine"
            | "Fr"
            | "Bls12381Fp"
            | "Bls12381Fp2"
            | "Bls12381G1Affine"
            | "Bls12381G2Affine"
            | "Bls12381Fr"
            | "Bn254Fp"
            | "Bn254G1Affine"
            | "Bn254G2Affine"
            | "Bn254Fr"
            | "BnScalar"
    )
}
