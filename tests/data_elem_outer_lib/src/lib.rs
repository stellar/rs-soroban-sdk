//! A data library providing only a tuple struct, with a field of a type from
//! another data library, for a contract to use as the element type of a `Vec`,
//! without converting any element, so that the contract links the other crate in
//! only through this crate's spec anchor.
#![no_std]
use soroban_sdk::contracttype;
use test_data_elem_inner_lib::ElemInner;

#[contracttype]
pub struct ElemOuter(pub ElemInner);
