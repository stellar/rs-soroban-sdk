use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecTypeBytesN, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption, ScSpecTypeResult,
    ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec,
};
use syn::{
    spanned::Spanned, Error, Expr, ExprLit, GenericArgument, Ident, Lit, Path, PathArguments,
    PathSegment, Type, TypePath, TypeTuple,
};
use syn::{Generics, TypeReference};

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

/// Checks if a type corresponds to a user-defined type (UDT) that can be used in the XDR spec safely.
/// Returns without error if the type is OK to use as a UDT spec name.
///
/// This function is used to check if a user-defined type input maps to a UDT. Otherwise,
/// the identifier risks collision with a built-in spec type of the same name.
///
/// ### Errors
/// - If `generics` has any parameters, as UDTs don't support generics
/// - If `ident` cannot be parsed as a Rust type
/// - If `ident` cannot be mapped to a type with `map_type`
/// - If the type mapped from `ident` is not a UDT
pub fn is_input_type_spec_safe(ident: &Ident, generics: &Generics) -> Result<(), Error> {
    if generics.params.len() > 0 {
        return Err(Error::new(
            ident.span(),
            "generics unsupported on user-defined types",
        ));
    }
    let ty: Type = syn::parse_str(&ident.to_string()).map_err(|e| {
        Error::new(
            ident.span(),
            format!("type name {} cannot be used in XDR spec: {}", ident, e),
        )
    })?;
    match map_type(&ty, false, false) {
        Ok(ScSpecTypeDef::Udt(_)) => Ok(()),
        _ => Err(Error::new(
            ident.span(),
            format!("type name `{}` conflicts with a soroban_sdk type", ident),
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
                }) => match &ident.to_string()[..] {
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
                    // Fp/Fp2/G1Affine/G2Affine => BytesN<N>, Fr => U256. This approach
                    // simplifies integration with contract development tooling, as it
                    // avoids introducing new spec types for these constructs.
                    //
                    // While this is functionally sound because the types are
                    // essentially newtypes over their inner representations, it means
                    // that the specific semantic meaning of `G1Affine`, `G2Affine`, or
                    // `Fr` is not directly visible in the compiled WASM interface. For
                    // example, a contract function expecting a `G1Affine` will appear
                    // in the WASM interface as expecting a `BytesN<96>`.
                    //
                    // Future enhancements might allow the macro to automatically deduce
                    // and utilize the inner types for types defined using the New Type
                    // Idiom. For more details, see the tracking issue for supporting
                    // type aliases:
                    // https://github.com/stellar/rs-soroban-sdk/issues/1063

                    // These BLS12-381 unprefixed type names
                    // will be removed in a future release.
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
                    s => Ok(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                        name: s.try_into().map_err(|e| {
                            Error::new(
                                t.span(),
                                format!("Udt name {:?} cannot be used in XDR spec: {}", s, e),
                            )
                        })?,
                    })),
                },
                Some(PathSegment {
                    ident,
                    arguments: PathArguments::AngleBracketed(angle_bracketed),
                }) => {
                    let args = angle_bracketed.args.iter().collect::<Vec<_>>();
                    match &ident.to_string()[..] {
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
    fn test_is_input_type_spec_safe_non_udt_errors() {
        let input: DeriveInput = parse_quote!(
            struct Address {
                pub key: [u8; 32],
            }
        );
        is_input_type_spec_safe(&input.ident, &input.generics)
            .expect_err("type name `Address` conflicts with a soroban_sdk type");
    }

    #[test]
    fn test_is_input_type_spec_safe_unique_generic_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct GenericType<T> {
                pub key: T,
            }
        );
        is_input_type_spec_safe(&input.ident, &input.generics)
            .expect_err("generics unsupported on user-defined types");
    }

    #[test]
    fn test_is_input_type_spec_safe_generic_type_errors() {
        let input: DeriveInput = parse_quote!(
            struct BytesN<T> {
                pub key: T,
            }
        );
        is_input_type_spec_safe(&input.ident, &input.generics)
            .expect_err("generics unsupported on user-defined types");
    }

    #[test]
    fn test_is_input_type_spec_safe_generic_no_params_errors() {
        let input: DeriveInput = parse_quote!(
            struct BytesN {
                pub key: [u8; 32],
            }
        );
        is_input_type_spec_safe(&input.ident, &input.generics)
            .expect_err("type name `BytesN` conflicts with a soroban_sdk type");
    }

    #[test]
    fn test_is_input_type_spec_safe_unique_ok() {
        let input: DeriveInput = parse_quote!(
            struct MyType {
                pub key: [u8; 32],
            }
        );
        assert!(is_input_type_spec_safe(&input.ident, &input.generics).is_ok());
    }
}
