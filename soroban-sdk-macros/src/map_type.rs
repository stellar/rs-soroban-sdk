use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use stellar_xdr::{
    ScSpecEntry, ScSpecTypeBytesN, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption,
    ScSpecTypeResult, ScSpecTypeTuple, ScSpecTypeUdtv2, ScSpecTypeVec, WriteXdr,
};
use syn::{
    ext::IdentExt as _, spanned::Spanned, Error, Expr, ExprLit, GenericArgument, Ident, Lit, Path,
    PathArguments, PathSegment, Type, TypePath, TypeTuple,
};
use syn::{Generics, TypeReference};

use crate::syn_ext::ident_to_type;
use crate::DEFAULT_XDR_RW_LIMITS;

// These constants' values must match the definitions of the constants with the
// same names in soroban_sdk::crypto::bls12_381.
pub const FP_SERIALIZED_SIZE: u32 = 48;
pub const FP2_SERIALIZED_SIZE: u32 = FP_SERIALIZED_SIZE * 2;
pub const G1_SERIALIZED_SIZE: u32 = FP_SERIALIZED_SIZE * 2;
pub const G2_SERIALIZED_SIZE: u32 = FP2_SERIALIZED_SIZE * 2;

// BN254 constants - values must match soroban_sdk::crypto::bn254
pub const BN254_FP_SERIALIZED_SIZE: u32 = 32;
pub const BN254_G1_SERIALIZED_SIZE: u32 = BN254_FP_SERIALIZED_SIZE * 2; // 64
pub const BN254_G2_SERIALIZED_SIZE: u32 = BN254_G1_SERIALIZED_SIZE * 2; // 128

/// Checks if an `ident` and `generics` input type maps to a user-defined type (UDT).
///
/// Returns Ok if the input will be parsed as a UDT, and returns an Err with a message if not.
///
/// When users defined types like with `#[contracttype]`, the type name must map to a UDT.
/// Otherwise, the type might get mapped to a built-in soroban_sdk type instead.
///
/// ### Errors
/// - If `ident` cannot be parsed as a Rust type
/// - If `ident` cannot be mapped to a type with [map_type]
/// - If the type mapped from `ident` is not a UDT
/// - If `generics` has any parameters, as UDTs don't support generics
pub fn is_mapped_type_udt(ident: &Ident, generics: &Generics) -> Result<(), Error> {
    // Wrap the Ident directly into a Type rather than stringifying and
    // re-parsing — the latter would fail for raw keyword idents like `r#type`
    // because the unraw'd form (`type`) isn't a valid Rust Type.
    let ty = ident_to_type(ident.clone());
    match map_type(&ty, false, false) {
        Ok(ScSpecTypeDef::Udt(_) | ScSpecTypeDef::UdtV2(_)) => {
            // `ty` does not contain the generics, so check manually here
            if generics.params.len() > 0 {
                Err(Error::new(
                    ident.span(),
                    format!("type `{}` contains generics `{}`, which are not supported for user-defined types", ident, generics.params.to_token_stream()),
                ))
            } else {
                Ok(())
            }
        }
        _ => {
            // Check if the error originated from the UDT-arm of `map_type`
            let name = ident.unraw().to_string();
            let _ = ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 {
                id: [0u8; 8],
                name: name.try_into().map_err(|e| {
                    Error::new(
                        ident.span(),
                        format!("type `{}` cannot be used in XDR spec: {}", ident, e),
                    )
                })?,
            });
            Err(Error::new(
                ident.span(),
                format!("type `{}` conflicts with a soroban_sdk type and cannot be used as a user-defined type", ident),
            ))
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn map_type(t: &Type, allow_ref: bool, allow_hash: bool) -> Result<ScSpecTypeDef, Error> {
    match t {
        Type::Reference(TypeReference { elem, .. }) => {
            if allow_ref {
                map_type(elem, allow_ref, allow_hash)
            } else {
                Err(Error::new(t.span(), "references unsupported"))
            }
        }
        Type::Path(TypePath {
            qself: None,
            path: Path { segments, .. },
        }) => {
            match segments.last() {
                Some(PathSegment {
                    ident,
                    arguments: PathArguments::None,
                }) => match &ident.unraw().to_string()[..] {
                    "Val" => Ok(ScSpecTypeDef::Val),
                    "u64" => Ok(ScSpecTypeDef::U64),
                    "i64" => Ok(ScSpecTypeDef::I64),
                    "u32" => Ok(ScSpecTypeDef::U32),
                    "i32" => Ok(ScSpecTypeDef::I32),
                    "u128" => Ok(ScSpecTypeDef::U128),
                    "i128" => Ok(ScSpecTypeDef::I128),
                    "U256" => Ok(ScSpecTypeDef::U256),
                    "I256" => Ok(ScSpecTypeDef::I256),
                    "bool" => Ok(ScSpecTypeDef::Bool),
                    "Symbol" => Ok(ScSpecTypeDef::Symbol),
                    "String" => Ok(ScSpecTypeDef::String),
                    "Error" => Ok(ScSpecTypeDef::Error),
                    "Bytes" => Ok(ScSpecTypeDef::Bytes),
                    "Address" => Ok(ScSpecTypeDef::Address),
                    "MuxedAddress" => Ok(ScSpecTypeDef::MuxedAddress),
                    "Timepoint" => Ok(ScSpecTypeDef::Timepoint),
                    "Duration" => Ok(ScSpecTypeDef::Duration),
                    // Check if types that require generics are being used without any path arguments
                    "Result" | "Option" | "Vec" | "Map" | "BytesN" | "Hash" => Err(Error::new(
                        ident.span(),
                        format!("type {} requires generic arguments", ident),
                    )),
                    // The BLS and BN types defined below are represented in the contract's
                    // interface by their underlying data types, i.e.
                    // Bls12381Fp/Bls12381Fp2/Bls12381G1Affine/Bls12381G2Affine => BytesN<N>,
                    // Bls12381Fr/Bn254Fr => U256. This approach simplifies integration with
                    // contract development tooling, as it avoids introducing new spec types
                    // for these constructs.
                    //
                    // While this is functionally sound because the types are
                    // essentially newtypes over their inner representations, it means
                    // that the specific semantic meaning of `Bls12381G1Affine`,
                    // `Bls12381G2Affine`, `Bls12381Fr`, or `Bn254Fr` is not directly visible
                    // in the compiled WASM interface. For example, a contract function
                    // expecting a `Bls12381G1Affine` will appear in the WASM interface as
                    // expecting a `BytesN<96>`.
                    //
                    // Future enhancements might allow the macro to automatically deduce
                    // and utilize the inner types for types defined using the New Type
                    // Idiom. For more details, see the tracking issue for supporting
                    // type aliases:
                    // https://github.com/stellar/rs-soroban-sdk/issues/1063

                    // These BLS12-381 unprefixed type names are deprecated.
                    // Use the Bls12381-prefixed names instead.
                    "Fp" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: FP_SERIALIZED_SIZE,
                    })),
                    "Fp2" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: FP2_SERIALIZED_SIZE,
                    })),
                    "G1Affine" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: G1_SERIALIZED_SIZE,
                    })),
                    "G2Affine" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: G2_SERIALIZED_SIZE,
                    })),
                    // Deprecated: `Fr` maps to BLS12-381 Fr for backward compat.
                    // Use `Bls12381Fr` or `Bn254Fr` instead.
                    "Fr" => Ok(ScSpecTypeDef::U256),
                    // BLS12-381 prefixed type names
                    "Bls12381Fp" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: FP_SERIALIZED_SIZE,
                    })),
                    "Bls12381Fp2" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: FP2_SERIALIZED_SIZE,
                    })),
                    "Bls12381G1Affine" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: G1_SERIALIZED_SIZE,
                    })),
                    "Bls12381G2Affine" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: G2_SERIALIZED_SIZE,
                    })),
                    "Bls12381Fr" => Ok(ScSpecTypeDef::U256),
                    // BN254 prefixed type names
                    "Bn254Fp" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: BN254_FP_SERIALIZED_SIZE,
                    })),
                    "Bn254G1Affine" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: BN254_G1_SERIALIZED_SIZE,
                    })),
                    "Bn254G2Affine" => Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN {
                        n: BN254_G2_SERIALIZED_SIZE,
                    })),
                    "Bn254Fr" => Ok(ScSpecTypeDef::U256),
                    // Deprecated alias for Bn254Fr
                    "BnScalar" => Ok(ScSpecTypeDef::U256),
                    s => Ok(ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 {
                        id: [0u8; 8],
                        name: s.try_into().map_err(|e| {
                            Error::new(
                                t.span(),
                                format!("type `{}` cannot be used in XDR spec: {}", s, e),
                            )
                        })?,
                    })),
                },
                Some(PathSegment {
                    ident,
                    arguments: PathArguments::AngleBracketed(angle_bracketed),
                }) => {
                    let args = angle_bracketed.args.iter().collect::<Vec<_>>();
                    match &ident.unraw().to_string()[..] {
                        "Result" => {
                            let (ok, err) = match args.as_slice() {
                                [GenericArgument::Type(ok), GenericArgument::Type(err)] => (ok, err),
                                [..] => Err(Error::new(
                                    t.span(),
                                    "incorrect number of generic arguments, expect two for Result<T, E>",
                                ))?,
                            };
                            Ok(ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                                ok_type: Box::new(map_type(ok, allow_ref, false)?),
                                error_type: Box::new(map_type(err, allow_ref, false)?),
                            })))
                        }
                        "Option" => {
                            let t = match args.as_slice() {
                            [GenericArgument::Type(t)] => t,
                            [..] => Err(Error::new(
                                t.span(),
                                "incorrect number of generic arguments, expect one for Option<T>",
                            ))?,
                        };
                            Ok(ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
                                value_type: Box::new(map_type(t, allow_ref, false)?),
                            })))
                        }
                        "Vec" => {
                            let t = match args.as_slice() {
                                [GenericArgument::Type(t)] => t,
                                [..] => Err(Error::new(
                                    t.span(),
                                    "incorrect number of generic arguments, expect one for Vec<T>",
                                ))?,
                            };
                            Ok(ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                                element_type: Box::new(map_type(t, allow_ref, false)?),
                            })))
                        }
                        "Map" => {
                            let (k, v) = match args.as_slice() {
                                [GenericArgument::Type(k), GenericArgument::Type(v)] => (k, v),
                                [..] => Err(Error::new(
                                    t.span(),
                                    "incorrect number of generic arguments, expect two for Map<K, V>",
                                ))?,
                            };
                            Ok(ScSpecTypeDef::Map(Box::new(ScSpecTypeMap {
                                key_type: Box::new(map_type(k, allow_ref, false)?),
                                value_type: Box::new(map_type(v, allow_ref, false)?),
                            })))
                        }
                        "BytesN" => {
                            let n = match args.as_slice() {
                                [GenericArgument::Const(Expr::Lit(ExprLit { lit: Lit::Int(int), .. }))] => int.base10_parse()?,
                                [..] => Err(Error::new(
                                    t.span(),
                                    "incorrect number of generic arguments, expect one for BytesN<N>",
                                ))?,
                            };
                            Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n }))
                        }
                        "Hash" => {
                            if allow_hash {
                                let n = match args.as_slice() {
                                    [GenericArgument::Const(Expr::Lit(ExprLit { lit: Lit::Int(int), .. }))] => int.base10_parse()?,
                                    [..] => Err(Error::new(
                                        t.span(),
                                        "incorrect number of generic arguments, expect one for Hash<N>",
                                    ))?,
                                };
                                Ok(ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n }))
                            } else {
                                Err(Error::new(
                                    t.span(),
                                    "Hash<N> can only be used in contexts where there is a guarantee that the hash has been sourced from a secure cryptographic hash function",
                                ))
                            }
                        }
                        _ => Err(Error::new(
                            angle_bracketed.span(),
                            "generics unsupported on user-defined types in contract functions",
                        ))?,
                    }
                }
                _ => Err(Error::new(t.span(), "unsupported type"))?,
            }
        }
        Type::Tuple(TypeTuple { elems, .. }) => {
            if elems.is_empty() {
                Ok(ScSpecTypeDef::Void)
            } else {
                let map_type_reject_hash =
                    |t: &Type| -> Result<ScSpecTypeDef, Error> { map_type(t, allow_ref, false) };
                Ok(ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                    value_types: elems
                        .iter()
                        .map(map_type_reject_hash)
                        .collect::<Result<Vec<ScSpecTypeDef>, Error>>()? // TODO: Implement conversion to VecM from iters to omit this collect.
                        .try_into()
                        .map_err(|e| {
                            Error::new(
                                t.span(),
                                format!("tuple values cannot be used in XDR spec: {}", e),
                            )
                        })?,
                })))
            }
        }
        _ => Err(Error::new(t.span(), "unsupported type"))?,
    }
}

/// Builds a Rust type identifier from a UDT type name, using a raw identifier
/// when the name is a keyword (e.g. `type` -> `r#type`).
fn type_ident(name: &str) -> Ident {
    syn::parse_str::<Ident>(name).unwrap_or_else(|_| Ident::new_raw(name, Span::call_site()))
}

/// The UDT type name a `Type::Path` maps to in the spec (its last path segment,
/// unraw'd), matching how [`map_type`] names user-defined types.
fn udt_ref_name(t: &Type) -> Option<String> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = t
    {
        segments.last().map(|s| s.ident.unraw().to_string())
    } else {
        None
    }
}

/// Collects the Rust types of every user-defined-type reference reachable from
/// `t`, in a form suitable for naming the referenced type in generated code
/// (e.g. `errcontract::Flag` for a `Vec<errcontract::Flag>` field). Mirrors the
/// structure [`map_type`] walks, reusing it to classify leaf paths as UDT or
/// built-in so the two never disagree.
pub(crate) fn collect_udt_ref_types(t: &Type, out: &mut Vec<Type>) {
    match t {
        Type::Reference(TypeReference { elem, .. }) => collect_udt_ref_types(elem, out),
        Type::Path(TypePath {
            qself: None,
            path: Path { segments, .. },
        }) => match segments.last() {
            Some(PathSegment {
                arguments: PathArguments::None,
                ..
            }) => {
                if let Ok(ScSpecTypeDef::UdtV2(_)) = map_type(t, true, true) {
                    out.push(t.clone());
                }
            }
            Some(PathSegment {
                ident,
                arguments: PathArguments::AngleBracketed(angle_bracketed),
            }) => {
                let args = angle_bracketed.args.iter();
                match &ident.unraw().to_string()[..] {
                    // Containers whose type arguments can themselves be UDTs, in
                    // the same order map_type serializes them.
                    "Result" | "Map" | "Option" | "Vec" => {
                        for a in args {
                            if let GenericArgument::Type(ty) = a {
                                collect_udt_ref_types(ty, out);
                            }
                        }
                    }
                    // BytesN<N>, Hash<N> and friends carry no UDT references.
                    _ => {}
                }
            }
            _ => {}
        },
        Type::Tuple(TypeTuple { elems, .. }) => {
            for e in elems {
                collect_udt_ref_types(e, out);
            }
        }
        _ => {}
    }
}

/// Given a spec `entry` whose `ScSpecTypeUdtv2` references carry placeholder
/// (zeroed) ids, and the Rust types of the UDTs those references point at,
/// returns the byte length of the entry's XDR and the body of a `const fn` that
/// reproduces that XDR with every reference's id patched in from the referenced
/// type's `SPEC_XDR_ID` associated const. This resolves each reference to the id
/// of the type it points at, which is only known once that type is compiled.
///
/// `ref_types` are matched to references by name; a reference whose name is not
/// found falls back to naming the type by its (bare) name.
pub(crate) fn spec_xdr_const_fn(entry: &ScSpecEntry, ref_types: &[Type]) -> (usize, TokenStream2) {
    let by_name: std::collections::BTreeMap<String, &Type> = ref_types
        .iter()
        .filter_map(|t| udt_ref_name(t).map(|n| (n, t)))
        .collect();
    let canonical = entry.to_xdr(DEFAULT_XDR_RW_LIMITS).unwrap();
    let len = canonical.len();
    let lit = proc_macro2::Literal::byte_string(&canonical);
    let sites = soroban_spec::udt_id::udt_ref_sites(entry);
    let body = if sites.is_empty() {
        quote! { { *#lit } }
    } else {
        let patches = sites.into_iter().map(|site| {
            let off = site.id_offset;
            let ty: TokenStream2 = match by_name.get(&site.name) {
                Some(t) => quote! { #t },
                None => type_ident(&site.name).into_token_stream(),
            };
            quote! {
                {
                    let id = <#ty>::SPEC_XDR_ID;
                    let mut i = 0usize;
                    while i < 8 {
                        bytes[#off + i] = id[i];
                        i += 1;
                    }
                }
            }
        });
        quote! {
            {
                let mut bytes = *#lit;
                #(#patches)*
                bytes
            }
        }
    };
    (len, body)
}

/// Returns tokens for the 8-byte `SPEC_XDR_ID` array literal identifying the
/// type defined by `entry` (the value carried by references to it).
pub(crate) fn spec_xdr_id_literal(entry: &ScSpecEntry) -> TokenStream2 {
    let id: Vec<u8> = soroban_spec::udt_id::canonical_id(entry).to_vec();
    quote! { [#(#id),*] }
}

/// Generates the spec code for a referenceable user-defined type (struct,
/// tuple-struct, union, enum, or error enum) named `ident`.
///
/// The `SPEC_XDR_ID` identity const is always emitted — references to this type
/// (from anywhere) need it, even when the type's own spec is not exported. The
/// `spec_xdr` fn and its exported `contractspecv0` static are emitted only when
/// `spec` is set. `ref_types` are the Rust types of the UDTs `entry` references,
/// used to resolve their ids (see [`spec_xdr_const_fn`]).
pub(crate) fn referenceable_spec_gen(
    entry: &ScSpecEntry,
    ref_types: &[Type],
    ident: &Ident,
    spec: bool,
) -> TokenStream2 {
    let spec_xdr_id = spec_xdr_id_literal(entry);
    let id_impl = quote! {
        impl #ident {
            #[doc(hidden)]
            pub const SPEC_XDR_ID: [u8; 8] = #spec_xdr_id;
        }
    };
    if spec {
        let (spec_xdr_len, spec_xdr_body) = spec_xdr_const_fn(entry, ref_types);
        let spec_ident = format_ident!(
            "__SPEC_XDR_TYPE_{}",
            ident.unraw().to_string().to_uppercase()
        );
        quote! {
            #[cfg_attr(target_family = "wasm", link_section = "contractspecv0")]
            pub static #spec_ident: [u8; #spec_xdr_len] = #ident::spec_xdr();

            impl #ident {
                pub const fn spec_xdr() -> [u8; #spec_xdr_len] #spec_xdr_body
            }

            #id_impl
        }
    } else {
        id_impl
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn test_path() {
        let ty = syn::Type::Path(parse_quote!(u32));
        let res = map_type(&ty, false, false);
        assert_eq!(res.unwrap(), ScSpecTypeDef::U32);
    }

    #[test]
    fn test_ref() {
        let ty = Type::Reference(parse_quote!(&u32));
        let res = map_type(&ty, true, false);
        assert_eq!(res.unwrap(), ScSpecTypeDef::U32);
    }

    #[test]
    fn test_ref_error_when_ref_not_allowed() {
        let ty = Type::Reference(parse_quote!(&u32));
        let res = map_type(&ty, false, false);
        assert!(res.is_err());
    }

    #[test]
    fn test_unit_type() {
        let ty: Type = parse_quote!(());
        let res = map_type(&ty, false, false);
        assert_eq!(res.unwrap(), ScSpecTypeDef::Void);
    }

    #[test]
    fn test_tuple_single_element() {
        let ty: Type = parse_quote!((u32,));
        let res = map_type(&ty, false, false);
        assert_eq!(
            res.unwrap(),
            ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                value_types: vec![ScSpecTypeDef::U32].try_into().unwrap(),
            }))
        );
    }

    #[test]
    fn test_tuple_two_elements() {
        let ty: Type = parse_quote!((u32, i64));
        let res = map_type(&ty, false, false);
        assert_eq!(
            res.unwrap(),
            ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                value_types: vec![ScSpecTypeDef::U32, ScSpecTypeDef::I64]
                    .try_into()
                    .unwrap(),
            }))
        );
    }

    #[test]
    fn test_generic_type() {
        let ty: Type = parse_quote!(Vec<u32>);
        let res = map_type(&ty, false, false);
        assert_eq!(
            res.unwrap(),
            ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                element_type: Box::new(ScSpecTypeDef::U32),
            }))
        );
    }

    #[test]
    fn test_generic_type_multiple_params() {
        let ty: Type = parse_quote!(Result<u32, i64>);
        let res = map_type(&ty, false, false);
        assert_eq!(
            res.unwrap(),
            ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::U32),
                error_type: Box::new(ScSpecTypeDef::I64),
            }))
        );
    }

    #[test]
    fn test_generic_type_without_params_errors() {
        let ty: Type = parse_quote!(Vec);
        assert!(map_type(&ty, false, false).is_err());
    }

    #[test]
    fn test_generic_type_incorrect_params_errors() {
        let ty: Type = parse_quote!(Result<u32>);
        assert!(map_type(&ty, false, false).is_err());
    }

    #[test]
    fn test_is_mapped_type_udt_sdk_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct Address {
                pub key: [u8; 32],
            }
        );
        let err = is_mapped_type_udt(&input.ident, &input.generics).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `Address` conflicts with a soroban_sdk type and cannot be used as a user-defined type"
        );
    }

    #[test]
    fn test_is_mapped_type_udt_unique_generic_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct GenericType<A, B> {
                pub key: T,
            }
        );
        let err = is_mapped_type_udt(&input.ident, &input.generics).unwrap_err();
        assert_eq!(err.to_string(), "type `GenericType` contains generics `A , B`, which are not supported for user-defined types");
    }

    #[test]
    fn test_is_mapped_type_udt_sdk_generic_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct BytesN<T> {
                pub key: T,
            }
        );
        let err = is_mapped_type_udt(&input.ident, &input.generics).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `BytesN` conflicts with a soroban_sdk type and cannot be used as a user-defined type"
        );
    }

    #[test]
    fn test_is_mapped_type_udt_sdk_generic_no_params_errors() {
        let input: DeriveInput = parse_quote!(
            struct BytesN {
                pub key: [u8; 32],
            }
        );
        let err = is_mapped_type_udt(&input.ident, &input.generics).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `BytesN` conflicts with a soroban_sdk type and cannot be used as a user-defined type"
        );
    }

    #[test]
    fn test_is_mapped_type_udt_unique_xdr_error() {
        let input: DeriveInput = parse_quote!(
            struct MyTypeIsOverSixtyCharactersLongAndShouldFailToCompileDueToThat {
                pub key: [u8; 32],
            }
        );
        let err = is_mapped_type_udt(&input.ident, &input.generics).unwrap_err();
        assert_eq!(
            err.to_string(),
            "type `MyTypeIsOverSixtyCharactersLongAndShouldFailToCompileDueToThat` cannot be used in XDR spec: xdr value max length exceeded"
        );
    }

    #[test]
    fn test_is_mapped_type_udt_unique_ok() {
        let input: DeriveInput = parse_quote!(
            struct MyType {
                pub key: [u8; 32],
            }
        );
        assert!(is_mapped_type_udt(&input.ident, &input.generics).is_ok());
    }
}
