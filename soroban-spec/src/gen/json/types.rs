use serde::Serialize;
use stellar_xdr::{
    ScSpecEntry, ScSpecFunctionInputV0, ScSpecTypeDef, ScSpecUdtEnumCaseV0,
    ScSpecUdtErrorEnumCaseV0, ScSpecUdtStructFieldV0, ScSpecUdtUnionCaseV0,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructField {
    docs: String,
    name: String,
    value: Type,
}

impl From<&ScSpecUdtStructFieldV0> for StructField {
    fn from(f: &ScSpecUdtStructFieldV0) -> Self {
        StructField {
            docs: f.docs.to_string_lossy(),
            name: f.name.to_string_lossy(),
            value: (&f.type_).into(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInput {
    name: String,
    value: Type,
}

impl From<&ScSpecFunctionInputV0> for FunctionInput {
    fn from(f: &ScSpecFunctionInputV0) -> Self {
        FunctionInput {
            name: f.name.to_string_lossy(),
            value: (&f.type_).into(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnionCase {
    docs: String,
    name: String,
    values: Vec<Type>,
}

impl From<&ScSpecUdtUnionCaseV0> for UnionCase {
    fn from(c: &ScSpecUdtUnionCaseV0) -> Self {
        UnionCase {
            docs: c.docs.to_string_lossy(),
            name: c.name.to_string_lossy(),
            values: c.type_.as_ref().map(Type::from).into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumCase {
    docs: String,
    name: String,
    value: u32,
}

impl From<&ScSpecUdtEnumCaseV0> for EnumCase {
    fn from(c: &ScSpecUdtEnumCaseV0) -> Self {
        EnumCase {
            docs: c.docs.to_string_lossy(),
            name: c.name.to_string_lossy(),
            value: c.value,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEnumCase {
    docs: String,
    name: String,
    value: u32,
}

impl From<&ScSpecUdtErrorEnumCaseV0> for EnumCase {
    fn from(c: &ScSpecUdtErrorEnumCaseV0) -> Self {
        EnumCase {
            docs: c.docs.to_string_lossy(),
            name: c.name.to_string_lossy(),
            value: c.value,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Type {
    Val,
    U64,
    I64,
    U32,
    I32,
    U128,
    I128,
    Bool,
    Symbol,
    Bitset,
    Status,
    Bytes,
    Address,
    AccountId,
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
        docs: String,
        name: String,
        inputs: Vec<FunctionInput>,
        outputs: Vec<Type>,
    },
    Struct {
        docs: String,
        name: String,
        fields: Vec<StructField>,
    },
    Union {
        docs: String,
        name: String,
        cases: Vec<UnionCase>,
    },
    Enum {
        docs: String,
        name: String,
        cases: Vec<EnumCase>,
    },
    ErrorEnum {
        docs: String,
        name: String,
        cases: Vec<ErrorEnumCase>,
    },
}

impl From<&ScSpecTypeDef> for Type {
    fn from(spec: &ScSpecTypeDef) -> Self {
        match spec {
            ScSpecTypeDef::Map(map) => Type::Map {
                key: Box::new(Type::from(map.key_type.as_ref())),
                value: Box::new(Type::from(map.value_type.as_ref())),
            },
            ScSpecTypeDef::Option(opt) => Type::Option {
                value: Box::new(Type::from(opt.value_type.as_ref())),
            },
            ScSpecTypeDef::Result(res) => Type::Result {
                value: Box::new(Type::from(res.ok_type.as_ref())),
                error: Box::new(Type::from(res.error_type.as_ref())),
            },
            ScSpecTypeDef::Set(set) => Type::Set {
                element: Box::new(Type::from(set.element_type.as_ref())),
            },
            ScSpecTypeDef::Tuple(tuple) => Type::Tuple {
                elements: tuple.value_types.iter().map(Type::from).collect(),
            },
            ScSpecTypeDef::Vec(vec) => Type::Vec {
                element: Box::new(Type::from(vec.element_type.as_ref())),
            },
            ScSpecTypeDef::Udt(udt) => Type::Custom {
                name: udt.name.to_string_lossy(),
            },
            ScSpecTypeDef::BytesN(b) => Type::BytesN { n: b.n },
            ScSpecTypeDef::Val => Type::Val,
            ScSpecTypeDef::U64 => Type::U64,
            ScSpecTypeDef::I64 => Type::I64,
            ScSpecTypeDef::U32 => Type::U32,
            ScSpecTypeDef::I32 => Type::I32,
            ScSpecTypeDef::U128 => Type::U128,
            ScSpecTypeDef::I128 => Type::I128,
            ScSpecTypeDef::Bool => Type::Bool,
            ScSpecTypeDef::Symbol => Type::Symbol,
            ScSpecTypeDef::Bitset => Type::Bitset,
            ScSpecTypeDef::Status => Type::Status,
            ScSpecTypeDef::Bytes => Type::Bytes,
            ScSpecTypeDef::Invoker => Type::Address,
            ScSpecTypeDef::AccountId => Type::AccountId,
        }
    }
}

impl From<&ScSpecEntry> for Entry {
    fn from(spec: &ScSpecEntry) -> Self {
        match spec {
            ScSpecEntry::FunctionV0(f) => Entry::Function {
                docs: f.docs.to_string_lossy(),
                name: f.name.to_string_lossy(),
                inputs: f.inputs.iter().map(FunctionInput::from).collect(),
                outputs: f.outputs.iter().map(Type::from).collect(),
            },
            ScSpecEntry::UdtStructV0(s) => Entry::Struct {
                docs: s.docs.to_string_lossy(),
                name: s.name.to_string_lossy(),
                fields: s.fields.iter().map(StructField::from).collect(),
            },
            ScSpecEntry::UdtUnionV0(u) => Entry::Union {
                docs: u.docs.to_string_lossy(),
                name: u.name.to_string_lossy(),
                cases: u.cases.iter().map(UnionCase::from).collect(),
            },
            ScSpecEntry::UdtEnumV0(e) => Entry::Enum {
                docs: e.docs.to_string_lossy(),
                name: e.name.to_string_lossy(),
                cases: e.cases.iter().map(EnumCase::from).collect(),
            },
            ScSpecEntry::UdtErrorEnumV0(e) => Entry::Enum {
                docs: e.docs.to_string_lossy(),
                name: e.name.to_string_lossy(),
                cases: e.cases.iter().map(EnumCase::from).collect(),
            },
        }
    }
}
