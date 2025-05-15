use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    ScSpecTypeBytesN, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption, ScSpecTypeResult,
    ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec,
};
use syn::{
    spanned::Spanned, Error, Expr, ExprLit, GenericArgument, Lit, Path, PathArguments, PathSegment,
    Type, TypePath, TypeTuple,
};

// These constants' values must match the definitions of the constants with the
// same names in soroban_sdk::crypto::bls12_381.
pub const FP_SERIALIZED_SIZE: u32 = 48;
pub const FP2_SERIALIZED_SIZE: u32 = FP_SERIALIZED_SIZE * 2;
pub const G1_SERIALIZED_SIZE: u32 = FP_SERIALIZED_SIZE * 2;
pub const G2_SERIALIZED_SIZE: u32 = FP2_SERIALIZED_SIZE * 2;

#[allow(clippy::too_many_lines)]
pub fn map_type(t: &Type, allow_hash: bool) -> Result<ScSpecTypeDef, Error> {
    match t {
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
                    "Timepoint" => Ok(ScSpecTypeDef::Timepoint),
                    "Duration" => Ok(ScSpecTypeDef::Duration),
                    // The BLS types defined below are represented in the contract's
                    // interface by their underlying data types, i.e.
                    // Fp/Fp2/G1Affine/G2Affine => BytesN<N>, Fr => U256. This approach
                    // simplifies integration with contract development tooling, as it
                    // avoids introducing new spec types for these BLS constructs.
                    //
                    // While this is functionally sound because the BLS types are
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
                                ok_type: Box::new(map_type(ok, false)?),
                                error_type: Box::new(map_type(err, false)?),
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
                                value_type: Box::new(map_type(t, false)?),
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
                                element_type: Box::new(map_type(t, false)?),
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
                                key_type: Box::new(map_type(k, false)?),
                                value_type: Box::new(map_type(v, false)?),
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
            let map_type_reject_hash =
                |t: &Type| -> Result<ScSpecTypeDef, Error> { map_type(t, false) };
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
        _ => Err(Error::new(t.span(), "unsupported type"))?,
    }
}
