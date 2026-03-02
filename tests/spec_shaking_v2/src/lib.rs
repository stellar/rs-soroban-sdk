#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Env, Map, Symbol, Vec,
};

#[contract]
pub struct Contract;

// --- Used types: markers expected ---

// Used as fn param (struct with nested type)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsedParamStruct {
    pub a: u32,
    pub nested: UsedNestedInStruct,
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

// --- Non-pub used types: spec entries + markers expected with feature ---

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

// --- Imported types from contractimport: only used ones should have markers ---

mod imported {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/test_spec_contract_import_lib.wasm"
    );
}

// --- Unused types: no markers expected ---

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

// Non-pub unused struct: spec entry exists but no marker
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct UnusedNonPubStruct {
    pub x: u32,
}

// Non-pub unused error: spec entry exists but no marker
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum UnusedNonPubError {
    Bad = 1,
}

#[allow(private_interfaces)]
#[contractimpl]
impl Contract {
    pub fn with_param(_env: Env, _s: UsedParamStruct, _ie: UsedParamIntEnum) {}

    pub fn with_return(_env: Env) -> UsedReturnEnum {
        UsedReturnEnum::A(1)
    }

    pub fn with_error(_env: Env) -> Result<u32, UsedErrorEnum> {
        Ok(42)
    }

    pub fn with_vec(_env: Env, _v: Vec<UsedVecElement>) {}

    pub fn with_map(_env: Env, _m: Map<UsedMapKey, UsedMapVal>) {}

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

    pub fn with_imported(_env: Env, _s: imported::StructA) {}

    pub fn with_non_pub(_env: Env, _s: UsedNonPubStruct) {}

    pub fn with_non_pub_error(_env: Env) -> Result<u32, UsedNonPubError> {
        Ok(1)
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

#[cfg(test)]
mod test;
