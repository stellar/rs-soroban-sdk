use stellar_xdr::{
    ScSpecTypeBytesN, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption, ScSpecTypeResult,
    ScSpecTypeSet, ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec,
};
use syn::{
    spanned::Spanned, Error, Expr, ExprLit, GenericArgument, Lit, Path, PathArguments, PathSegment,
    Type, TypePath, TypeTuple,
};

#[allow(clippy::too_many_lines)]
pub fn map_type(t: &Type) -> Result<ScSpecTypeDef, Error> {
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
                    "RawVal" => Ok(ScSpecTypeDef::Val),
                    "u64" => Ok(ScSpecTypeDef::U64),
                    "i64" => Ok(ScSpecTypeDef::I64),
                    "u32" => Ok(ScSpecTypeDef::U32),
                    "i32" => Ok(ScSpecTypeDef::I32),
                    "u128" => Ok(ScSpecTypeDef::U128),
                    "i128" => Ok(ScSpecTypeDef::I128),
                    "bool" => Ok(ScSpecTypeDef::Bool),
                    "Symbol" => Ok(ScSpecTypeDef::Symbol),
                    "Bitset" => Ok(ScSpecTypeDef::Bitset),
                    "Status" => Ok(ScSpecTypeDef::Status),
                    "Bytes" => Ok(ScSpecTypeDef::Bytes),
                    "Address" => Ok(ScSpecTypeDef::Invoker),
                    "AccountId" => Ok(ScSpecTypeDef::AccountId),
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
                                ok_type: Box::new(map_type(ok)?),
                                error_type: Box::new(map_type(err)?),
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
                                value_type: Box::new(map_type(t)?),
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
                                element_type: Box::new(map_type(t)?),
                            })))
                        }
                        "Set" => {
                            let t = match args.as_slice() {
                                [GenericArgument::Type(t)] => t,
                                [..] => Err(Error::new(
                                    t.span(),
                                    "incorrect number of generic arguments, expect one for Set<T>",
                                ))?,
                            };
                            Ok(ScSpecTypeDef::Set(Box::new(ScSpecTypeSet {
                                element_type: Box::new(map_type(t)?),
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
                                key_type: Box::new(map_type(k)?),
                                value_type: Box::new(map_type(v)?),
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
            Ok(ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                value_types: elems
                    .iter()
                    .map(map_type)
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
