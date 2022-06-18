use stellar_xdr::{SpecTypeDef, SpecTypeTuple, SpecTypeUdt};
use syn::{Type, TypePath, TypeTuple, Path, PathArguments, PathSegment};

// TODO: Remove user-defined types from SpecTypeDef and treat separately.

pub fn type_def_from_str(t: &Type) -> SpecTypeDef {
    match t {
        Type::Path(TypePath { qself: None, path: Path { segments { ident, arguments: PathArguments::None }, .. } }) => match path.segments.last() {
            Some()
            None => unimplemented!(),
        }
        Type::Tuple(TypeTuple { elems, .. }) => SpecTypeDef::Tuple(Box::new(SpecTypeTuple {
            value_types: elems
                .iter()
                .map(|e| type_def_from_str(e))
                .collect() // TODO: Implement conversion to VecM from iters to omit this collect.
                .try_into()
                .unwrap(),
        })),
        _ => unimplemented!(),
    }
    // match s.as_ref() {
    //     "u64" => SpecTypeDef::I64,
    //     "i64" => SpecTypeDef::I64,
    //     "u32" => SpecTypeDef::I32,
    //     "i32" => SpecTypeDef::I32,
    //     "bool" => SpecTypeDef::Bool,
    //     "Symbol" => SpecTypeDef::Symbol,
    //     "Bitset" => SpecTypeDef::Bitset,
    //     "Status" => SpecTypeDef::Status,
    //     "Binary" => SpecTypeDef::Binary, // TODO: Would this be a Vec<u8>?
    //     "Option<T>" => SpecTypeDef::Binary, // TODO: How do we piece apart the generics? Can we get more than a str? I think so!
    //     "Vec<T>" => SpecTypeDef::Binary, // TODO: How do we piece apart the generics? Can we get more than a str? I think so!
    //     "Map<K, V>" => SpecTypeDef::Binary, // TODO: How do we piece apart the generics? Can we get more than a str? I think so!
    //     "Set<T>" => SpecTypeDef::Binary, // TODO: How do we piece apart the generics? Can we get more than a str? I think so!
    //     "(...)" => SpecTypeDef::Binary, // TODO: How do we piece apart the generics? Can we get more than a str? I think so!
    //     s => SpecTypeDef::Udt(SpecTypeUdt {
    //         name: s.try_into().unwrap(), // TODO: Handle error.
    //     }),
    // }
}
