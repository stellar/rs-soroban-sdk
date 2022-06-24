use stellar_xdr::{
    SpecTypeDef, SpecTypeMap, SpecTypeOption, SpecTypeSet, SpecTypeTuple, SpecTypeUdt, SpecTypeVec,
};
use syn::{
    spanned::Spanned, Error, GenericArgument, Path, PathArguments, PathSegment, Type, TypePath,
    TypeTuple,
};

#[allow(clippy::too_many_lines)]
pub fn map_type(t: &Type) -> Result<SpecTypeDef, Error> {
    match t {
        Type::Path(TypePath {
            qself: None,
            path: Path { segments, .. },
        }) => match segments.last() {
            Some(PathSegment {
                ident,
                arguments: PathArguments::None,
            }) => match &ident.to_string()[..] {
                "u64" => Ok(SpecTypeDef::U64),
                "i64" => Ok(SpecTypeDef::I64),
                "u32" => Ok(SpecTypeDef::U32),
                "i32" => Ok(SpecTypeDef::I32),
                "bool" => Ok(SpecTypeDef::Bool),
                "Symbol" => Ok(SpecTypeDef::Symbol),
                "Bitset" => Ok(SpecTypeDef::Bitset),
                "Status" => Ok(SpecTypeDef::Status),
                "Binary" => Ok(SpecTypeDef::Binary),
                s => Ok(SpecTypeDef::Udt(Box::new(SpecTypeUdt {
                    name: s.try_into().map_err(|e| {
                        Error::new(
                            t.span(),
                            format!("Udt name {:?} cannot be used in XDR spec: {}", s, e),
                        )
                    })?,
                    udt_def: None,
                }))),
            },
            Some(PathSegment {
                ident,
                arguments: PathArguments::AngleBracketed(angle_bracketed),
            }) => {
                let args = angle_bracketed.args.iter().collect::<Vec<_>>();
                match &ident.to_string()[..] {
                    "Option" => {
                        let t = match args.as_slice() {
                            [GenericArgument::Type(t)] => t,
                            [..] => Err(Error::new(
                                t.span(),
                                "incorrect number of generic arguments",
                            ))?,
                        };
                        Ok(SpecTypeDef::Option(Box::new(SpecTypeOption {
                            value_type: Box::new(map_type(t)?),
                        })))
                    }
                    "Vec" => {
                        let t = match args.as_slice() {
                            [GenericArgument::Type(t)] => t,
                            [..] => Err(Error::new(
                                t.span(),
                                "incorrect number of generic arguments",
                            ))?,
                        };
                        Ok(SpecTypeDef::Vec(Box::new(SpecTypeVec {
                            element_type: Box::new(map_type(t)?),
                        })))
                    }
                    "Set" => {
                        let t = match args.as_slice() {
                            [GenericArgument::Type(t)] => t,
                            [..] => Err(Error::new(
                                t.span(),
                                "incorrect number of generic arguments",
                            ))?,
                        };
                        Ok(SpecTypeDef::Set(Box::new(SpecTypeSet {
                            element_type: Box::new(map_type(t)?),
                        })))
                    }
                    "Map" => {
                        let (k, v) = match args.as_slice() {
                            [GenericArgument::Type(k), GenericArgument::Type(v)] => (k, v),
                            [..] => Err(Error::new(
                                t.span(),
                                "incorrect number of generic arguments",
                            ))?,
                        };
                        Ok(SpecTypeDef::Map(Box::new(SpecTypeMap {
                            key_type: Box::new(map_type(k)?),
                            value_type: Box::new(map_type(v)?),
                        })))
                    }
                    _ => Err(Error::new(
                        angle_bracketed.span(),
                        "generics unsupported on user-defined types in contract functions",
                    ))?,
                }
            }
            _ => Err(Error::new(t.span(), "unsupported type"))?,
        },
        Type::Tuple(TypeTuple { elems, .. }) => Ok(SpecTypeDef::Tuple(Box::new(SpecTypeTuple {
            value_types: elems
                .iter()
                .map(map_type)
                .collect::<Result<Vec<SpecTypeDef>, Error>>()? // TODO: Implement conversion to VecM from iters to omit this collect.
                .try_into()
                .map_err(|e| {
                    Error::new(
                        t.span(),
                        format!("tuple values cannot be used in XDR spec: {}", e),
                    )
                })?,
        }))),
        _ => Err(Error::new(t.span(), "unsupported type"))?,
    }
}
