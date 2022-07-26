#![cfg_attr(target_family = "wasm", no_std)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![allow(dead_code)]

// This requires running on unstable.
//#![feature(alloc)]
//mod alloc;

#[cfg(target_family = "wasm")]
use stellar_contract_env_panic_handler_wasm32_unreachable as _;

#[cfg_attr(target_family = "wasm", link_section = "contractenvmetav0")]
static ENV_META_XDR: [u8; env::meta::XDR.len()] = env::meta::XDR;

pub use stellar_contract_macros::{contractimpl, contracttype, ContractType};

mod env;

/// XDR contains types for building and generating XDR values.
pub mod xdr {
    // XDR types needed by macros.
    #[doc(hidden)]
    pub use super::env::xdr::HostFunction;

    // XDR generic types and traits.
    #[cfg(not(target_family = "wasm"))]
    pub use super::env::xdr::ReadXdrIter;
    pub use super::env::xdr::{Error, ReadXdr, Validate, VecM, WriteXdr};

    // XDR contract specific types.
    pub use super::env::xdr::{
        ScBigInt, ScEnvMetaEntry, ScEnvMetaKind, ScHash, ScHashType, ScHostContextErrorCode,
        ScHostFnErrorCode, ScHostObjErrorCode, ScHostStorageErrorCode, ScHostValErrorCode, ScMap,
        ScMapEntry, ScNumSign, ScObject, ScObjectType, ScSpecEntry, ScSpecEntryKind,
        ScSpecFunctionV0, ScSpecType, ScSpecTypeDef, ScSpecTypeMap, ScSpecTypeOption,
        ScSpecTypeResult, ScSpecTypeSet, ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec,
        ScSpecUdtStructFieldV0, ScSpecUdtStructV0, ScSpecUdtUnionCaseV0, ScSpecUdtUnionV0,
        ScStatic, ScStatus, ScStatusType, ScSymbol, ScUnknownErrorCode, ScVal, ScValType, ScVec,
        ScVmErrorCode,
    };
}

pub use env::BitSet;
pub use env::ConversionError;
pub use env::Env;
#[doc(hidden)]
pub use env::EnvType;
pub use env::EnvVal;
#[doc(hidden)]
pub use env::IntoEnvVal;
pub use env::IntoVal;
#[doc(hidden)]
pub use env::Object;
pub use env::RawVal;
pub use env::Status;
pub use env::Symbol;
pub use env::TryFromVal;
#[doc(hidden)]
pub use env::TryIntoEnvVal;
pub use env::TryIntoVal;
use env::*;

mod bigint;
mod binary;
pub mod iter;
mod map;
mod vec;
pub use bigint::BigInt;
pub use binary::{Binary, FixedBinary};
pub use map::Map;
pub use vec::Vec;

pub mod serde;

pub mod testutils;
