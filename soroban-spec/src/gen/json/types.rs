use serde::Serialize;
use soroban_env_host::xdr::{
    Error, ScSpecEntry, ScSpecFunctionInputV0, ScSpecTypeDef, ScSpecUdtStructFieldV0,
    ScSpecUdtUnionCaseV0,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    name: String,
    value: Type,
}

impl TryFrom<&ScSpecUdtStructFieldV0> for StructField {
    type Error = Error;

    fn try_from(f: &ScSpecUdtStructFieldV0) -> Result<Self, Self::Error> {
        Ok(StructField {
            name: f.name.to_string()?,
            value: Type::try_from(&f.type_)?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInput {
    name: String,
    value: Type,
}

impl TryFrom<&ScSpecFunctionInputV0> for FunctionInput {
    type Error = Error;

    fn try_from(f: &ScSpecFunctionInputV0) -> Result<Self, Self::Error> {
        Ok(FunctionInput {
            name: f.name.to_string()?,
            value: Type::try_from(&f.type_)?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnionCase {
    name: String,
    values: Vec<Type>,
}

impl TryFrom<&ScSpecUdtUnionCaseV0> for UnionCase {
    type Error = Error;

    fn try_from(c: &ScSpecUdtUnionCaseV0) -> Result<Self, Self::Error> {
        Ok(UnionCase {
            name: c.name.to_string()?,
            values: c
                .type_
                .as_ref()
                .map(Type::try_from)
                .transpose()?
                .into_iter()
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Type {
    U64,
    I64,
    U32,
    I32,
    Bool,
    Symbol,
    Bitset,
    Status,
    Bytes,
    BigInt,
    Map { key: Box<Type>, value: Box<Type> },
    Option { value: Box<Type> },
    Result { value: Box<Type>, error: Box<Type> },
    Set { element: Box<Type> },
    Vec { element: Box<Type> },
    BytesN { n: u32 },
    Tuple { elements: Vec<Type> },
    Custom { name: String },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Entry {
    Function {
        name: String,
        inputs: Vec<FunctionInput>,
        outputs: Vec<Type>,
    },
    Struct {
        name: String,
        fields: Vec<StructField>,
    },
    Union {
        name: String,
        cases: Vec<UnionCase>,
    },
}

impl TryFrom<&ScSpecTypeDef> for Type {
    type Error = Error;

    fn try_from(spec: &ScSpecTypeDef) -> Result<Self, Self::Error> {
        match spec {
            ScSpecTypeDef::Map(map) => Ok(Type::Map {
                key: Box::new(Type::try_from(map.key_type.as_ref())?),
                value: Box::new(Type::try_from(map.value_type.as_ref())?),
            }),
            ScSpecTypeDef::Option(opt) => Ok(Type::Option {
                value: Box::new(Type::try_from(opt.value_type.as_ref())?),
            }),
            ScSpecTypeDef::Result(res) => Ok(Type::Result {
                value: Box::new(Type::try_from(res.ok_type.as_ref())?),
                error: Box::new(Type::try_from(res.error_type.as_ref())?),
            }),
            ScSpecTypeDef::Set(set) => Ok(Type::Set {
                element: Box::new(Type::try_from(set.element_type.as_ref())?),
            }),
            ScSpecTypeDef::Tuple(tuple) => Ok(Type::Tuple {
                elements: tuple
                    .value_types
                    .iter()
                    .map(Type::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
            ScSpecTypeDef::Vec(vec) => Ok(Type::Vec {
                element: Box::new(Type::try_from(vec.element_type.as_ref())?),
            }),
            ScSpecTypeDef::Udt(udt) => Ok(Type::Custom {
                name: udt.name.to_string()?,
            }),
            ScSpecTypeDef::BytesN(b) => Ok(Type::BytesN { n: b.n }),
            ScSpecTypeDef::U64 => Ok(Type::U64),
            ScSpecTypeDef::I64 => Ok(Type::I64),
            ScSpecTypeDef::U32 => Ok(Type::U32),
            ScSpecTypeDef::I32 => Ok(Type::I32),
            ScSpecTypeDef::Bool => Ok(Type::Bool),
            ScSpecTypeDef::Symbol => Ok(Type::Symbol),
            ScSpecTypeDef::Bitset => Ok(Type::Bitset),
            ScSpecTypeDef::Status => Ok(Type::Status),
            ScSpecTypeDef::Bytes => Ok(Type::Bytes),
            ScSpecTypeDef::BigInt => Ok(Type::BigInt),
        }
    }
}

impl TryFrom<&ScSpecEntry> for Entry {
    type Error = Error;

    fn try_from(spec: &ScSpecEntry) -> Result<Self, Self::Error> {
        match spec {
            ScSpecEntry::FunctionV0(f) => Ok(Entry::Function {
                name: f.name.to_string()?,
                inputs: f
                    .inputs
                    .iter()
                    .map(FunctionInput::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
                outputs: f
                    .outputs
                    .iter()
                    .map(Type::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
            ScSpecEntry::UdtStructV0(s) => Ok(Entry::Struct {
                name: s.name.to_string()?,
                fields: s
                    .fields
                    .iter()
                    .map(StructField::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
            ScSpecEntry::UdtUnionV0(u) => Ok(Entry::Union {
                name: u.name.to_string()?,
                cases: u
                    .cases
                    .iter()
                    .map(UnionCase::try_from)
                    .collect::<Result<Vec<_>, Error>>()?,
            }),
        }
    }
}
