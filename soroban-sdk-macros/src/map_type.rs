use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use stellar_xdr::{
    ScSpecTypeBytesN, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption, ScSpecTypeResult,
    ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec, ScSymbol, StringM,
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
        Ok(ScSpecTypeDef::Udt(_)) => {
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
            let _ = ScSpecTypeDef::Udt(ScSpecTypeUdt {
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
                    s => Ok(ScSpecTypeDef::Udt(ScSpecTypeUdt {
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

/// Renders a [ScSpecTypeDef] as a const expression of type
/// `#path::xdr::r#const::ScSpecTypeDef`, so the containing spec entry can be encoded
/// to XDR at compile time by the contract crate.
pub fn const_view_type_def(path: &Path, t: &ScSpecTypeDef) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let variant = format_ident!("{}", t.name());
    // Variants that hold a value. The recursive ones sit behind a reference in
    // the const type, matching the Box in the owned type.
    let value = match t {
        ScSpecTypeDef::Option(o) => {
            let value_type = const_view_type_def(path, &o.value_type);
            Some(quote!((&#xdr::r#const::ScSpecTypeOption { value_type: &#value_type })))
        }
        ScSpecTypeDef::Result(r) => {
            let ok_type = const_view_type_def(path, &r.ok_type);
            let error_type = const_view_type_def(path, &r.error_type);
            Some(
                quote!((&#xdr::r#const::ScSpecTypeResult { ok_type: &#ok_type, error_type: &#error_type })),
            )
        }
        ScSpecTypeDef::Vec(v) => {
            let element_type = const_view_type_def(path, &v.element_type);
            Some(quote!((&#xdr::r#const::ScSpecTypeVec { element_type: &#element_type })))
        }
        ScSpecTypeDef::Map(m) => {
            let key_type = const_view_type_def(path, &m.key_type);
            let value_type = const_view_type_def(path, &m.value_type);
            Some(
                quote!((&#xdr::r#const::ScSpecTypeMap { key_type: &#key_type, value_type: &#value_type })),
            )
        }
        ScSpecTypeDef::Tuple(t) => {
            let value_types = t.value_types.iter().map(|t| const_view_type_def(path, t));
            Some(
                quote!((&#xdr::r#const::ScSpecTypeTuple { value_types: #xdr::r#const::VecM::try_from_slice_or_panic(&[#(#value_types),*]) })),
            )
        }
        ScSpecTypeDef::BytesN(b) => {
            let n = b.n;
            Some(quote!((#xdr::r#const::ScSpecTypeBytesN { n: #n })))
        }
        ScSpecTypeDef::Udt(u) => {
            let name = const_view_string(path, &u.name);
            Some(quote!((#xdr::r#const::ScSpecTypeUdt { name: #name })))
        }
        ScSpecTypeDef::Val
        | ScSpecTypeDef::Bool
        | ScSpecTypeDef::Void
        | ScSpecTypeDef::Error
        | ScSpecTypeDef::U32
        | ScSpecTypeDef::I32
        | ScSpecTypeDef::U64
        | ScSpecTypeDef::I64
        | ScSpecTypeDef::Timepoint
        | ScSpecTypeDef::Duration
        | ScSpecTypeDef::U128
        | ScSpecTypeDef::I128
        | ScSpecTypeDef::U256
        | ScSpecTypeDef::I256
        | ScSpecTypeDef::Bytes
        | ScSpecTypeDef::String
        | ScSpecTypeDef::Symbol
        | ScSpecTypeDef::Address
        | ScSpecTypeDef::MuxedAddress => None,
    };
    quote!(#xdr::r#const::ScSpecTypeDef::#variant #value)
}

/// Renders a [StringM] as a const expression of type `#path::xdr::r#const::StringM`.
/// The `MAX` of the `const::StringM` is inferred from the field it is assigned to.
pub fn const_view_string<const MAX: u32>(path: &Path, s: &StringM<MAX>) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let lit = Literal::byte_string(s.as_vec());
    quote!(#xdr::r#const::StringM::try_from_slice_or_panic(#lit))
}

/// Renders a [ScSymbol] as a const expression of type
/// `#path::xdr::r#const::ScSymbol`.
pub fn const_view_symbol(path: &Path, s: &ScSymbol) -> TokenStream2 {
    let xdr = quote!(#path::xdr);
    let s = const_view_string(path, &s.0);
    quote!(#xdr::r#const::ScSymbol(#s))
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;
    use stellar_xdr::SC_SPEC_TYPE_NAME_LIMIT;
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
        // A name longer than the XDR spec's type name limit.
        let name = "A".repeat(SC_SPEC_TYPE_NAME_LIMIT as usize + 1);
        let ident = Ident::new(&name, Span::call_site());
        let err = is_mapped_type_udt(&ident, &Generics::default()).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("type `{name}` cannot be used in XDR spec: xdr value max length exceeded")
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

#[cfg(test)]
mod test_const_view {
    use super::*;
    use stellar_xdr::ScSpecType;
    use syn::parse_quote;

    fn path() -> Path {
        parse_quote!(soroban_sdk)
    }

    /// Compares token streams by their rendering, so spacing is not asserted.
    fn assert_tokens(actual: TokenStream2, expected: TokenStream2) {
        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[test]
    fn test_every_variant_renders_with_its_own_name() {
        let p = path();
        for t in ScSpecType::VARIANTS {
            let def = match t {
                ScSpecType::Val => ScSpecTypeDef::Val,
                ScSpecType::Bool => ScSpecTypeDef::Bool,
                ScSpecType::Void => ScSpecTypeDef::Void,
                ScSpecType::Error => ScSpecTypeDef::Error,
                ScSpecType::U32 => ScSpecTypeDef::U32,
                ScSpecType::I32 => ScSpecTypeDef::I32,
                ScSpecType::U64 => ScSpecTypeDef::U64,
                ScSpecType::I64 => ScSpecTypeDef::I64,
                ScSpecType::Timepoint => ScSpecTypeDef::Timepoint,
                ScSpecType::Duration => ScSpecTypeDef::Duration,
                ScSpecType::U128 => ScSpecTypeDef::U128,
                ScSpecType::I128 => ScSpecTypeDef::I128,
                ScSpecType::U256 => ScSpecTypeDef::U256,
                ScSpecType::I256 => ScSpecTypeDef::I256,
                ScSpecType::Bytes => ScSpecTypeDef::Bytes,
                ScSpecType::String => ScSpecTypeDef::String,
                ScSpecType::Symbol => ScSpecTypeDef::Symbol,
                ScSpecType::Address => ScSpecTypeDef::Address,
                ScSpecType::MuxedAddress => ScSpecTypeDef::MuxedAddress,
                ScSpecType::Option => ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
                    value_type: Box::new(ScSpecTypeDef::U32),
                })),
                ScSpecType::Result => ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U32),
                    error_type: Box::new(ScSpecTypeDef::Error),
                })),
                ScSpecType::Vec => ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                    element_type: Box::new(ScSpecTypeDef::U32),
                })),
                ScSpecType::Map => ScSpecTypeDef::Map(Box::new(ScSpecTypeMap {
                    key_type: Box::new(ScSpecTypeDef::Symbol),
                    value_type: Box::new(ScSpecTypeDef::U32),
                })),
                ScSpecType::Tuple => ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                    value_types: vec![ScSpecTypeDef::U32].try_into().unwrap(),
                })),
                ScSpecType::BytesN => ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n: 32 }),
                ScSpecType::Udt => ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Foo".try_into().unwrap(),
                }),
            };
            // The variants that render a value. Every other must render as a
            // bare path with no trailing group.
            let holds_value = matches!(
                t,
                ScSpecType::Option
                    | ScSpecType::Result
                    | ScSpecType::Vec
                    | ScSpecType::Map
                    | ScSpecType::Tuple
                    | ScSpecType::BytesN
                    | ScSpecType::Udt
            );
            let rendered = const_view_type_def(&p, &def).to_string();
            // The variant named in the output is the one the value reports, so
            // no variant is rendered as another.
            let expect_prefix = format!(
                "soroban_sdk :: xdr :: r#const :: ScSpecTypeDef :: {}",
                def.name()
            );
            assert!(
                rendered.starts_with(&expect_prefix),
                "variant {t:?} rendered as {rendered}, expected prefix {expect_prefix}"
            );
            // A value shows up as a trailing group; void variants have none.
            assert_eq!(
                rendered.len() > expect_prefix.len(),
                holds_value,
                "variant {t:?} value presence mismatch, rendered {rendered}"
            );
        }
    }

    #[test]
    fn test_void_variant() {
        assert_tokens(
            const_view_type_def(&path(), &ScSpecTypeDef::Void),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Void),
        );
    }

    #[test]
    fn test_bytes_n() {
        // The const module re-exports the types that need no const-specific
        // form, so the const path names this one too.
        assert_tokens(
            const_view_type_def(&path(), &ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n: 32 })),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 32u32 }
            )),
        );
    }

    #[test]
    fn test_bytes_n_boundaries() {
        for n in [0u32, 1, u32::MAX] {
            assert_tokens(
                const_view_type_def(&path(), &ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n })),
                quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                    soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: #n }
                )),
            );
        }
    }

    #[test]
    fn test_option() {
        let def = ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
            value_type: Box::new(ScSpecTypeDef::U32),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Option(
                &soroban_sdk::xdr::r#const::ScSpecTypeOption {
                    value_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::U32
                }
            )),
        );
    }

    #[test]
    fn test_result() {
        let def = ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
            ok_type: Box::new(ScSpecTypeDef::U32),
            error_type: Box::new(ScSpecTypeDef::Error),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Result(
                &soroban_sdk::xdr::r#const::ScSpecTypeResult {
                    ok_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                    error_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Error
                }
            )),
        );
    }

    #[test]
    fn test_vec() {
        let def = ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
            element_type: Box::new(ScSpecTypeDef::Bool),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                    element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Bool
                }
            )),
        );
    }

    #[test]
    fn test_map() {
        // Key and value are distinct types, so a swap of the two would fail.
        let def = ScSpecTypeDef::Map(Box::new(ScSpecTypeMap {
            key_type: Box::new(ScSpecTypeDef::Symbol),
            value_type: Box::new(ScSpecTypeDef::I128),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Map(
                &soroban_sdk::xdr::r#const::ScSpecTypeMap {
                    key_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Symbol,
                    value_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::I128
                }
            )),
        );
    }

    #[test]
    fn test_tuple_empty() {
        let def = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![].try_into().unwrap(),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Tuple(
                &soroban_sdk::xdr::r#const::ScSpecTypeTuple {
                    value_types: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[])
                }
            )),
        );
    }

    #[test]
    fn test_tuple_single() {
        let def = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![ScSpecTypeDef::U32].try_into().unwrap(),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Tuple(
                &soroban_sdk::xdr::r#const::ScSpecTypeTuple {
                    value_types: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecTypeDef::U32
                    ])
                }
            )),
        );
    }

    #[test]
    fn test_tuple_preserves_order() {
        // Asserted in both orders, so a reversal would not pass.
        let forward = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![ScSpecTypeDef::U32, ScSpecTypeDef::I64]
                .try_into()
                .unwrap(),
        }));
        assert_tokens(
            const_view_type_def(&path(), &forward),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Tuple(
                &soroban_sdk::xdr::r#const::ScSpecTypeTuple {
                    value_types: soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(&[
                        soroban_sdk::xdr::r#const::ScSpecTypeDef::U32,
                        soroban_sdk::xdr::r#const::ScSpecTypeDef::I64
                    ])
                }
            )),
        );
        let reverse = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![ScSpecTypeDef::I64, ScSpecTypeDef::U32]
                .try_into()
                .unwrap(),
        }));
        assert_ne!(
            const_view_type_def(&path(), &forward).to_string(),
            const_view_type_def(&path(), &reverse).to_string(),
        );
    }

    #[test]
    fn test_udt() {
        let def = ScSpecTypeDef::Udt(ScSpecTypeUdt {
            name: "MyType".try_into().unwrap(),
        });
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"MyType")
                }
            )),
        );
    }

    #[test]
    fn test_udt_qualified_name() {
        // Qualified names, as produced for fully qualified UDTs, carry colons
        // that must survive as bytes rather than being parsed as tokens.
        let def = ScSpecTypeDef::Udt(ScSpecTypeUdt {
            name: "::my_crate::MyType".try_into().unwrap(),
        });
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                    name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                        b"::my_crate::MyType"
                    )
                }
            )),
        );
    }

    #[test]
    fn test_nested_recursion() {
        // Vec<Option<Map<Symbol, Tuple<(BytesN<4>, Udt)>>>>, so every recursive
        // arm is exercised at depth and each level must be reference-wrapped.
        let inner_tuple = ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
            value_types: vec![
                ScSpecTypeDef::BytesN(ScSpecTypeBytesN { n: 4 }),
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "U".try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        }));
        let map = ScSpecTypeDef::Map(Box::new(ScSpecTypeMap {
            key_type: Box::new(ScSpecTypeDef::Symbol),
            value_type: Box::new(inner_tuple),
        }));
        let option = ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
            value_type: Box::new(map),
        }));
        let def = ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
            element_type: Box::new(option),
        }));
        assert_tokens(
            const_view_type_def(&path(), &def),
            quote!(soroban_sdk::xdr::r#const::ScSpecTypeDef::Vec(
                &soroban_sdk::xdr::r#const::ScSpecTypeVec {
                    element_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Option(
                        &soroban_sdk::xdr::r#const::ScSpecTypeOption {
                            value_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Map(
                                &soroban_sdk::xdr::r#const::ScSpecTypeMap {
                                    key_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Symbol,
                                    value_type: &soroban_sdk::xdr::r#const::ScSpecTypeDef::Tuple(
                                        &soroban_sdk::xdr::r#const::ScSpecTypeTuple {
                                            value_types:
                                                soroban_sdk::xdr::r#const::VecM::try_from_slice_or_panic(
                                                    &[
                                                        soroban_sdk::xdr::r#const::ScSpecTypeDef::BytesN(
                                                            soroban_sdk::xdr::r#const::ScSpecTypeBytesN { n: 4u32 }
                                                        ),
                                                        soroban_sdk::xdr::r#const::ScSpecTypeDef::Udt(
                                                            soroban_sdk::xdr::r#const::ScSpecTypeUdt {
                                                                name: soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"U")
                                                            }
                                                        )
                                                    ]
                                                )
                                        }
                                    )
                                }
                            )
                        }
                    )
                }
            )),
        );
    }

    #[test]
    fn test_path_is_used_verbatim() {
        // Every generated path is rooted at the caller's path, including a
        // leading-colon path and a `crate` path.
        let def = ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
            value_type: Box::new(ScSpecTypeDef::U32),
        }));
        let p: Path = parse_quote!(::soroban_sdk);
        assert_tokens(
            const_view_type_def(&p, &def),
            quote!(::soroban_sdk::xdr::r#const::ScSpecTypeDef::Option(
                &::soroban_sdk::xdr::r#const::ScSpecTypeOption {
                    value_type: &::soroban_sdk::xdr::r#const::ScSpecTypeDef::U32
                }
            )),
        );
        let p: Path = parse_quote!(crate);
        assert_tokens(
            const_view_type_def(&p, &ScSpecTypeDef::U32),
            quote!(crate::xdr::r#const::ScSpecTypeDef::U32),
        );
    }

    #[test]
    fn test_string_empty() {
        assert_tokens(
            const_view_string(&path(), &StringM::<60>::try_from("").unwrap()),
            quote!(soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b""
            )),
        );
    }

    #[test]
    fn test_string_ascii() {
        assert_tokens(
            const_view_string(&path(), &StringM::<60>::try_from("hello").unwrap()),
            quote!(soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"hello"
            )),
        );
    }

    #[test]
    fn test_string_max_is_not_rendered() {
        // The const StringM's MAX is inferred at the assignment site, so the
        // same bytes render identically whatever the source MAX is.
        let narrow =
            const_view_string(&path(), &StringM::<4>::try_from("abcd").unwrap()).to_string();
        let wide =
            const_view_string(&path(), &StringM::<1024>::try_from("abcd").unwrap()).to_string();
        assert_eq!(narrow, wide);
        assert_eq!(
            narrow,
            quote!(soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(
                b"abcd"
            ))
            .to_string()
        );
    }

    #[test]
    fn test_symbol() {
        assert_tokens(
            const_view_symbol(&path(), &ScSymbol("transfer".try_into().unwrap())),
            quote!(soroban_sdk::xdr::r#const::ScSymbol(
                soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"transfer")
            )),
        );
    }

    #[test]
    fn test_symbol_empty() {
        assert_tokens(
            const_view_symbol(&path(), &ScSymbol("".try_into().unwrap())),
            quote!(soroban_sdk::xdr::r#const::ScSymbol(
                soroban_sdk::xdr::r#const::StringM::try_from_slice_or_panic(b"")
            )),
        );
    }

    #[test]
    fn test_symbol_wraps_the_string_rendering() {
        // The symbol is exactly its inner string rendering, wrapped once.
        let p = path();
        let sym = ScSymbol("abc".try_into().unwrap());
        let inner = const_view_string(&p, &sym.0);
        assert_tokens(
            const_view_symbol(&p, &sym),
            quote!(soroban_sdk::xdr::r#const::ScSymbol(#inner)),
        );
    }

    #[test]
    fn test_symbol_path_is_used_verbatim() {
        let p: Path = parse_quote!(crate);
        assert_tokens(
            const_view_symbol(&p, &ScSymbol("s".try_into().unwrap())),
            quote!(crate::xdr::r#const::ScSymbol(
                crate::xdr::r#const::StringM::try_from_slice_or_panic(b"s")
            )),
        );
    }
}
