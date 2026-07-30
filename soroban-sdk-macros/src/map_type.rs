use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use stellar_xdr::{
    ScSpecEntry, ScSpecTypeBytesN, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption,
    ScSpecTypeResult, ScSpecTypeTuple, ScSpecTypeUdtv2, ScSpecTypeVec, ScSymbol, StringM,
};
use syn::{
    ext::IdentExt as _, spanned::Spanned, Error, Expr, ExprLit, GenericArgument, Ident, Lit, Path,
    PathArguments, PathSegment, Type, TypePath, TypeTuple,
};
use syn::{Generics, TypeReference};

use crate::syn_ext::ident_to_type;

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

// The limit on the `name` of every UDT definition entry in the contract spec
// XDR, i.e. the `name<60>` of SCSpecUDTStructV0 and its union, enum, and error
// enum counterparts.
const UDT_NAME_LIMIT: u32 = 60;

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
        Ok(ScSpecTypeDef::UdtV2(_)) => {
            // A reference carries only the id, so the name is bounded by the
            // limit on the name of the definition entry that the type gets.
            let name = ident.unraw().to_string();
            StringM::<UDT_NAME_LIMIT>::try_from(name).map_err(|e| {
                Error::new(
                    ident.span(),
                    format!("type `{}` cannot be used in XDR spec: {}", ident, e),
                )
            })?;
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
        _ => Err(Error::new(
            ident.span(),
            format!("type `{}` conflicts with a soroban_sdk type and cannot be used as a user-defined type", ident),
        )),
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
                    // A reference to a user-defined type. The id identifying the
                    // referenced type is not known here, and is filled in by
                    // [const_ref_type_def] from that type's `spec_type_id`.
                    _ => Ok(ScSpecTypeDef::UdtV2(ScSpecTypeUdtv2 { id: [0u8; 8] })),
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

/// Strips any outer references from a Rust type, so a `&Vec<T>` param descends
/// like the `Vec<T>` it maps to.
fn unref(t: &Type) -> &Type {
    match t {
        Type::Reference(TypeReference { elem, .. }) => unref(elem),
        _ => t,
    }
}

/// The Rust type arguments that line up, in order, with the type arguments of
/// the spec type [map_type] produced for `t`: the arguments of a container
/// (`Option<T>`, `Result<T, E>`, `Vec<T>`, `Map<K, V>`) or the elements of a
/// tuple. Empty for anything else, including the parameterized types whose
/// arguments are not types in the spec (`BytesN<N>`, `Hash<N>`).
fn type_args(t: &Type) -> Vec<&Type> {
    match unref(t) {
        Type::Tuple(TypeTuple { elems, .. }) => elems.iter().collect(),
        Type::Path(TypePath {
            qself: None,
            path: Path { segments, .. },
        }) => match segments.last() {
            Some(PathSegment {
                ident,
                arguments: PathArguments::AngleBracketed(args),
            }) if matches!(
                &ident.unraw().to_string()[..],
                "Option" | "Result" | "Vec" | "Map"
            ) =>
            {
                args.args
                    .iter()
                    .filter_map(|a| match a {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    })
                    .collect()
            }
            _ => Vec::new(),
        },
        _ => Vec::new(),
    }
}

/// Emits the `spec_type_id` const fn on a user-defined type: the 8-byte identity
/// that references to it carry, which is the truncated SHA256 of its own spec
/// entry. Emitted for every user-defined type, even one whose spec is not
/// exported, because a reference to it from anywhere needs the id.
pub fn spec_type_id_gen(ident: &Ident, entry: &ScSpecEntry) -> TokenStream2 {
    let id = soroban_spec::udt_id::canonical_id(entry);
    quote! {
        impl #ident {
            #[doc(hidden)]
            pub const fn spec_type_id() -> [u8; 8] {
                [#(#id),*]
            }
        }
    }
}

/// Renders a [ScSpecTypeDef] as a const expression of type
/// `#path::xdr::ScSpecTypeDefRef`, so the containing spec entry can be encoded
/// to XDR at compile time by the contract crate. A reference to a user-defined
/// type takes its id from that type's `spec_type_id`, resolving the reference at
/// const evaluation time even when the type lives in another crate.
///
/// `rust` is the Rust type `t` was mapped from, descended in step with `t` so a
/// reference is reached holding the type that names it, keeping any path
/// qualification (e.g. the `othercontract::Flag` in a `Vec<othercontract::Flag>`)
/// that the rendered expression needs to name it.
pub fn const_ref_type_def(path: &Path, t: &ScSpecTypeDef, rust: Option<&Type>) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let variant = format_ident!("{}", t.name());
    let args = rust.map(type_args).unwrap_or_default();
    let arg = |i: usize| args.get(i).copied();
    // Variants that hold a value. The recursive ones sit behind a reference in
    // the Ref type, matching the Box in the owned type.
    let value = match t {
        ScSpecTypeDef::Option(o) => {
            let value_type = const_ref_type_def(path, &o.value_type, arg(0));
            Some(quote!((&#xdr::ScSpecTypeOptionRef { value_type: &#value_type })))
        }
        ScSpecTypeDef::Result(r) => {
            let ok_type = const_ref_type_def(path, &r.ok_type, arg(0));
            let error_type = const_ref_type_def(path, &r.error_type, arg(1));
            Some(
                quote!((&#xdr::ScSpecTypeResultRef { ok_type: &#ok_type, error_type: &#error_type })),
            )
        }
        ScSpecTypeDef::Vec(v) => {
            let element_type = const_ref_type_def(path, &v.element_type, arg(0));
            Some(quote!((&#xdr::ScSpecTypeVecRef { element_type: &#element_type })))
        }
        ScSpecTypeDef::Map(m) => {
            let key_type = const_ref_type_def(path, &m.key_type, arg(0));
            let value_type = const_ref_type_def(path, &m.value_type, arg(1));
            Some(
                quote!((&#xdr::ScSpecTypeMapRef { key_type: &#key_type, value_type: &#value_type })),
            )
        }
        ScSpecTypeDef::Tuple(t) => {
            let value_types = t
                .value_types
                .iter()
                .enumerate()
                .map(|(i, t)| const_ref_type_def(path, t, arg(i)));
            Some(
                quote!((&#xdr::ScSpecTypeTupleRef { value_types: #xdr::VecMRef::new(&[#(#value_types),*]) })),
            )
        }
        ScSpecTypeDef::BytesN(b) => {
            let n = b.n;
            Some(quote!((#xdr::ScSpecTypeBytesN { n: #n })))
        }
        ScSpecTypeDef::Udt(u) => {
            let name = const_ref_string(path, &u.name);
            Some(quote!((#xdr::ScSpecTypeUdtRef { name: #name })))
        }
        ScSpecTypeDef::UdtV2(_) => {
            // The reference carries only the id, which is the referenced type's
            // own `spec_type_id`. Naming that type is the only way to reach it,
            // so the Rust type the reference was mapped from is required, named
            // as written with its path qualification and all.
            match rust.map(unref) {
                Some(ty) => Some(quote!((#xdr::ScSpecTypeUdtv2 { id: <#ty>::spec_type_id() }))),
                None => Some(quote!(
                    (compile_error!(
                        "user-defined type reference has no Rust type to take its id from"
                    ))
                )),
            }
        }
        // All remaining variants are void.
        _ => None,
    };
    quote!(#xdr::ScSpecTypeDefRef::#variant #value)
}

/// Renders a [StringM] as a const expression of type `#path::xdr::StringMRef`.
/// The `MAX` of the `StringMRef` is inferred from the field it is assigned to.
pub fn const_ref_string<const MAX: u32>(path: &Path, s: &StringM<MAX>) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let lit = Literal::byte_string(s.as_vec());
    quote!(#xdr::StringMRef::new(#lit))
}

/// Renders a [ScSymbol] as a const expression of type
/// `#path::xdr::ScSymbolRef`.
pub fn const_ref_symbol(path: &Path, s: &ScSymbol) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let s = const_ref_string(path, &s.0);
    quote!(#xdr::ScSymbolRef(#s))
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
