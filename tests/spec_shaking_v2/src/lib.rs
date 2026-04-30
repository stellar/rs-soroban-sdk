#![no_std]
use soroban_sdk::{
    assert_with_error, contract, contracterror, contractevent, contractimpl, contracttype,
    panic_with_error, Env, Map, Symbol, Vec,
};

#[contract]
pub struct Contract;

// --- Used types: reachable from function specs or event-root markers ---

// Used as fn param (struct with nested type)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedParamStruct {
    pub a: u32,
    pub nested: UsedNestedInStruct,
}

// Used only as constructor param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedConstructorMeta {
    pub val: u32,
}

// Used as fn return (union enum)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UsedReturnEnum {
    A(u32),
    B(i64),
}

// Used as fn param (int enum)
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedParamIntEnum {
    X = 1,
    Y = 2,
}

// Used as fn error return
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedErrorEnum {
    NotFound = 1,
    Invalid = 2,
}

// Used only via panic_with_error! (never appears in a Result return).
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedPanicErrorEnum {
    Boom = 1,
}

// Used only via assert_with_error! (never appears in a Result return).
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedAssertErrorEnum {
    Bad = 1,
}

// Used as nested field of UsedParamStruct
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedNestedInStruct {
    pub val: i64,
}

// Used as Vec element in fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedVecElement {
    pub data: u32,
}

// Used as Map key in fn param
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedMapKey {
    K1 = 1,
    K2 = 2,
}

// Used as Map value in fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedMapVal {
    pub v: u32,
}

// Used as Option element in fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedOptionElement {
    pub data: u32,
}

// Used as Result Ok type in fn return
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedResultOk {
    pub data: u32,
}

// Used as published event (simple, primitive fields only)
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventSimple {
    #[topic]
    pub kind: Symbol,
    pub amount: i128,
}

// Used as published event with custom type in topic
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedEventTopicType {
    Transfer = 1,
    Mint = 2,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventWithTopicType {
    #[topic]
    pub kind: UsedEventTopicType,
    pub amount: i128,
}

// Used as published event with custom type in data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventDataType {
    pub x: u32,
    pub y: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventWithDataType {
    #[topic]
    pub kind: Symbol,
    pub payload: UsedEventDataType,
}

// Used as published event with nested custom type in topic
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventTopicOuter {
    pub inner: UsedEventTopicInner,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventTopicInner {
    pub val: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventWithNestedTopic {
    #[topic]
    pub info: UsedEventTopicOuter,
    pub amount: i128,
}

// Used as published event with nested custom type in data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventDataOuter {
    pub inner: UsedEventDataInner,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventDataInner {
    pub val: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventWithNestedData {
    #[topic]
    pub kind: Symbol,
    pub payload: UsedEventDataOuter,
}

// Used as published event with ref fields containing custom types
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UsedRefTopicType {
    Send = 1,
    Recv = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedRefDataType {
    pub nested: UsedRefDataInner,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedRefDataInner {
    pub val: u32,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedEventWithRefs<'a> {
    #[topic]
    pub kind: &'a UsedRefTopicType,
    pub payload: &'a UsedRefDataType,
}

// Used as element in tuple fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedTupleElement {
    pub val: u32,
}

// Used as element in tuple fn return
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedTupleReturnElement {
    pub val: u32,
}

// Used as nested type in Vec element in fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedVecInnerVecElement {
    pub val: u32,
}

// Used as nested type in Vec element in fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedVecInnerElement {
    pub val: u32,
}

// Used as type in Vec element in fn param containing custom types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedVecElementNested {
    pub val: u32,
    pub inner: UsedVecInnerElement,
    pub vec_inner: Vec<UsedVecInnerVecElement>,
}

// --- Non-pub used types: spec entries expected with feature ---

// Non-pub struct used as fn param
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct UsedNonPubStruct {
    pub val: u32,
}

// Non-pub error enum used as fn error return
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UsedNonPubError {
    Fail = 1,
}

// --- Recursive types: graph pruning should include all reachable entries ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedRecursiveRoot {
    pub val: UsedRecursiveNode,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UsedRecursiveNode {
    NotRecursive(UsedLeaf),
    Recursive(UsedRecursiveLeaf),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedRecursiveLeaf {
    pub val: Vec<UsedRecursiveRoot>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedLeaf {
    pub val: u32,
}

// --- Lib-imported types (Rust crate dep): rlib statics linked into cdylib ---
// Only StructC is used in a contract fn; other spec_lib types have spec entries
// but are not rooted by any function or published event.

// --- WASM-imported types (contractimport!): only used ones should survive pruning ---

mod wasm_imported {
    soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/test_spec_import.wasm");
}

// --- Unused types: not rooted by function specs or published events ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnusedStruct {
    pub x: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnusedEnum {
    A,
    B(i64),
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnusedIntEnum {
    U1 = 1,
    U2 = 2,
}

#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnusedEvent {
    #[topic]
    pub kind: Symbol,
    pub data: u32,
}

// A pub #[contracterror] enum that is never referenced anywhere — neither in a
// Result return type, nor in panic_with_error! / assert_with_error!. Confirms
// that an error enum that is not actually used is shaken out of the spec.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnusedPubError {
    Nope = 1,
}

// Used only in a non-contractimpl fn: spec entry exists but no marker
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnusedNonContractFnParam {
    pub x: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnusedNonContractFnReturn {
    pub x: u32,
}

// Non-pub unused struct: spec entry exists but is not rooted.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct UnusedNonPubStruct {
    pub x: u32,
}

// Non-pub unused error: spec entry exists but is not rooted.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UnusedNonPubError {
    Bad = 1,
}

#[allow(private_interfaces)]
#[contractimpl]
impl Contract {
    pub fn __constructor(_env: Env, _meta: UsedConstructorMeta) {}

    pub fn with_param(_env: Env, _s: UsedParamStruct, _ie: UsedParamIntEnum) {}

    pub fn with_return(_env: Env) -> UsedReturnEnum {
        UsedReturnEnum::A(1)
    }

    pub fn with_error(_env: Env) -> Result<u32, UsedErrorEnum> {
        Ok(42)
    }

    pub fn with_panic_error(env: Env, fail: bool) {
        if fail {
            panic_with_error!(&env, UsedPanicErrorEnum::Boom);
        }
    }

    pub fn with_assert_error(env: Env, ok: bool) {
        assert_with_error!(&env, ok, UsedAssertErrorEnum::Bad);
    }

    // Confirms that a raw soroban_sdk::Error still works as the argument to
    // panic_with_error! after the SpecShakingMarker bound was added to
    // Env::panic_with_error.
    pub fn with_panic_raw_error(env: Env, fail: bool) {
        if fail {
            panic_with_error!(&env, soroban_sdk::Error::from_contract_error(7));
        }
    }

    pub fn with_vec(_env: Env, _v: Vec<UsedVecElement>) {}

    pub fn with_vec_nested(_env: Env, _v: Vec<UsedVecElementNested>) {}

    pub fn with_map(_env: Env, _m: Map<UsedMapKey, UsedMapVal>) {}

    pub fn with_option(_env: Env, _o: Option<UsedOptionElement>) {}

    pub fn with_result(_env: Env) -> Result<UsedResultOk, UsedErrorEnum> {
        Ok(UsedResultOk { data: 1 })
    }

    pub fn with_recursion(_env: Env, _r: UsedRecursiveRoot) {}

    pub fn publish_simple(env: Env) {
        UsedEventSimple {
            kind: Symbol::new(&env, "transfer"),
            amount: 100,
        }
        .publish(&env);
    }

    pub fn publish_topic_type(env: Env) {
        UsedEventWithTopicType {
            kind: UsedEventTopicType::Transfer,
            amount: 100,
        }
        .publish(&env);
    }

    pub fn publish_data_type(env: Env) {
        UsedEventWithDataType {
            kind: Symbol::new(&env, "coords"),
            payload: UsedEventDataType { x: 1, y: 2 },
        }
        .publish(&env);
    }

    pub fn publish_nested_topic(env: Env) {
        UsedEventWithNestedTopic {
            info: UsedEventTopicOuter {
                inner: UsedEventTopicInner { val: 42 },
            },
            amount: 100,
        }
        .publish(&env);
    }

    pub fn publish_nested_data(env: Env) {
        UsedEventWithNestedData {
            kind: Symbol::new(&env, "nested"),
            payload: UsedEventDataOuter {
                inner: UsedEventDataInner { val: 42 },
            },
        }
        .publish(&env);
    }

    pub fn with_lib_struct(_env: Env, _s: test_spec_lib::StructC) {}

    pub fn with_wasm_imported(_env: Env, _s: wasm_imported::StructA) {}

    pub fn with_non_pub(_env: Env, _s: UsedNonPubStruct) {}

    pub fn with_non_pub_error(_env: Env) -> Result<u32, UsedNonPubError> {
        Ok(1)
    }

    pub fn with_tuple(_env: Env, _t: (UsedTupleElement, u32)) {}

    pub fn with_tuple_return(_env: Env) -> (UsedTupleReturnElement, u32) {
        (UsedTupleReturnElement { val: 1 }, 2)
    }

    pub fn publish_ref_event(env: Env) {
        let kind = UsedRefTopicType::Send;
        let payload = UsedRefDataType {
            nested: UsedRefDataInner { val: 99 },
        };
        UsedEventWithRefs {
            kind: &kind,
            payload: &payload,
        }
        .publish(&env);
    }
}

// Non-contractimpl function: types used here should not be kept since they are
// not at a contract boundary.
#[allow(dead_code)]
fn non_contract_fn(_s: UnusedNonContractFnParam) -> UnusedNonContractFnReturn {
    UnusedNonContractFnReturn { x: 1 }
}

#[cfg(test)]
mod test;
