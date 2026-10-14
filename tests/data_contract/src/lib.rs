#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Map, Vec};
use test_data_elem_outer_lib::ElemOuter;
use test_data_enum_lib::Enum;
use test_data_field_elem_lib::FieldElem;
use test_data_int_enum_lib::IntEnum;
use test_data_map_elem_lib::{MapKey, MapVal};
use test_data_nested_lib::Outer;
use test_data_tuple_lib::Tuple;
use test_data_vec_elem_lib::VecElem;
use test_data_wrapped_lib::Wrapped;

#[contracttype]
pub struct Wrapper(pub Wrapped);

impl Wrapper {
    // An inherent fn with the same name as the SpecAnchor trait's fn, which the
    // generated conversions call, must not clash with it.
    pub fn spec_anchor() -> u32 {
        0
    }
}

// A type with a Vec field whose element type is used nowhere else, so that the
// element type's spec is linked in only through the field.
#[contracttype]
pub struct HasVecField(pub Vec<FieldElem>);

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn int_enum(v: IntEnum) -> IntEnum {
        v
    }

    pub fn enum_(v: Enum) -> Enum {
        v
    }

    pub fn tuple(v: Tuple) -> Tuple {
        v
    }

    pub fn outer(v: Outer) -> u32 {
        v.0 .0
    }

    pub fn wrapper(v: Wrapper) -> u32 {
        v.0 .0
    }

    // The conversions of a Vec and a Map to and from Val do not convert their
    // elements, so a type used only as an element type, with no element read
    // or written, has its spec anchor called only through the anchor of the
    // Vec or Map.
    pub fn vec_elem(v: Vec<VecElem>) -> u32 {
        v.len()
    }

    pub fn map_elem(v: Map<MapKey, MapVal>) -> u32 {
        v.len()
    }

    pub fn has_vec_field(v: HasVecField) -> u32 {
        v.0.len()
    }

    // A type used only as the element type of a Vec, with no element converted,
    // has its spec anchor called only through the anchor of the Vec, and so the
    // types of its fields are linked in only through its anchor.
    pub fn elem_field(v: Vec<ElemOuter>) -> u32 {
        v.len()
    }
}
