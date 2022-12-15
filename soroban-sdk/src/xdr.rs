/// XDR contains types for building and generating XDR values.

// XDR types needed by macros.
#[doc(hidden)]
pub use super::env::xdr::HostFunction;

// XDR generic types and traits.
#[cfg(not(target_family = "wasm"))]
pub use super::env::xdr::ReadXdrIter;
pub use super::env::xdr::{Error, ReadXdr, Validate, VecM, WriteXdr};

// XDR contract val types.
pub use super::env::xdr::{
    ScMap, ScMapEntry, ScObject, ScObjectType, ScStatic, ScStatus, ScStatusType, ScSymbol, ScVal,
    ScValType, ScVec,
};

// XDR contract error codes.
pub use super::env::xdr::{
    ScHostContextErrorCode, ScHostFnErrorCode, ScHostObjErrorCode, ScHostStorageErrorCode,
    ScHostValErrorCode, ScUnknownErrorCode, ScVmErrorCode,
};

// XDR contract types.
pub use super::env::xdr::{
    ContractEvent, ContractEventBody, ContractEventType, ContractEventV0, ExtensionPoint,
};

// XDR contract env meta types.
pub use super::env::xdr::{ScEnvMetaEntry, ScEnvMetaKind};

// XDR contract spec types.
pub use super::env::xdr::{
    ScSpecEntry, ScSpecEntryKind, ScSpecFunctionV0, ScSpecType, ScSpecTypeDef, ScSpecTypeMap,
    ScSpecTypeOption, ScSpecTypeResult, ScSpecTypeSet, ScSpecTypeTuple, ScSpecTypeUdt,
    ScSpecTypeVec, ScSpecUdtStructFieldV0, ScSpecUdtStructV0, ScSpecUdtUnionCaseV0,
    ScSpecUdtUnionV0,
};

// XDR for ledger entries.
pub use super::env::xdr::{AccountEntry, AccountId, Asset};
