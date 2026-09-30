//! Names shared by the macros that write contract specs and the tools that
//! read them.

/// The names of the soroban_sdk types that a user-defined type cannot take,
/// so that a reference to a user-defined type can never be mistaken for one of
/// them by a reader of the contract.
///
/// This matters in particular for contracts built with older SDKs that import
/// contracts built with this one. Older SDKs map a type named like an SDK
/// type, such as `Address`, to the SDK type rather than to a user-defined
/// type of the same name, so a contract must not export user-defined types
/// with these names, or older importers would misread its interface.
///
/// Tools that generate code from a spec, such as `contractimport!`, rename a
/// type with one of these names, so that the generated type does not take it
/// either.
pub const RESERVED_NAMES: &[&str] = &[
    "Val",
    "bool",
    "u32",
    "i32",
    "u64",
    "i64",
    "u128",
    "i128",
    "U256",
    "I256",
    "Timepoint",
    "Duration",
    "Bytes",
    "BytesN",
    "Hash",
    "String",
    "Symbol",
    "Address",
    "MuxedAddress",
    "Vec",
    "Map",
    "Option",
    "Result",
    "Fp",
    "Fp2",
    "Fr",
    "G1Affine",
    "G2Affine",
    "Bls12381Fp",
    "Bls12381Fp2",
    "Bls12381Fr",
    "Bls12381G1Affine",
    "Bls12381G2Affine",
    "Bn254Fp",
    "Bn254Fr",
    "Bn254G1Affine",
    "Bn254G2Affine",
    "BnScalar",
];
