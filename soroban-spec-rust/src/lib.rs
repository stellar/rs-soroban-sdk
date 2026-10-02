mod syn_ext;
mod r#trait;
pub mod types;

use std::borrow::Cow;
use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use stellar_xdr::{ScMetaEntry, ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtUnionCaseV0};
use syn::Error;

use soroban_spec::read::{from_wasm, FromWasmError};

use types::{
    generate_enum_with_options, generate_error_enum_with_options, generate_event_with_options,
    generate_struct_with_options, generate_union_with_options,
};
pub use types::{GenerateError, GenerateOptions};

// IMPORTANT: The "docs" fields of spec entries are not output in Rust token
// streams as rustdocs, because rustdocs can contain Rust code, and that code
// will be executed. Generated code may be generated from untrusted Wasm
// containing untrusted spec docs.

/// The first major version of the soroban-sdk whose specs the error override
/// does not apply to, as those SDKs name a contract's own error enum distinctly
/// from `soroban_sdk::Error`.
const ERROR_UDT_OVERRIDE_BEFORE_SDK_MAJOR: u32 = 30;

#[derive(thiserror::Error, Debug)]
pub enum GenerateFromFileError {
    #[error("reading file: {0}")]
    Io(io::Error),
    #[error("sha256 does not match, expected: {expected}")]
    VerifySha256 { expected: String },
    #[error("parsing contract spec: {0}")]
    Parse(stellar_xdr::Error),
    #[error("getting contract spec: {0}")]
    GetSpec(FromWasmError),
    #[error("getting contract meta: {0}")]
    GetMeta(soroban_meta::read::FromWasmError),
    #[error("generating code: {0}")]
    Generate(GenerateError),
}

pub fn generate_from_file(
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<TokenStream, GenerateFromFileError> {
    // Read file.
    let wasm = fs::read(file).map_err(GenerateFromFileError::Io)?;

    // Generate code.
    let code = generate_from_wasm(&wasm, file, verify_sha256)?;
    Ok(code)
}

/// Generates the code for a contract from its wasm, recognising the version
/// of the soroban-sdk that built it from its `rssdkver` meta.
pub fn generate_from_wasm(
    wasm: &[u8],
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<TokenStream, GenerateFromFileError> {
    generate_from_wasm_with_options(wasm, file, verify_sha256, &GenerateOptions::default())
}

/// Generates the code for a contract from its wasm, with configurable options,
/// recognising the version of the soroban-sdk that built it from its
/// `rssdkver` meta.
pub fn generate_from_wasm_with_options(
    wasm: &[u8],
    file: &str,
    verify_sha256: Option<&str>,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateFromFileError> {
    let sha256 = Sha256::digest(wasm);
    let sha256 = format!("{:x}", sha256);
    if let Some(verify_sha256) = verify_sha256 {
        if verify_sha256 != sha256 {
            return Err(GenerateFromFileError::VerifySha256 { expected: sha256 });
        }
    }

    let spec = from_wasm(wasm).map_err(GenerateFromFileError::GetSpec)?;
    let meta = soroban_meta::read::from_wasm(wasm).map_err(GenerateFromFileError::GetMeta)?;
    let sdk_major_version = sdk_major_version_from_meta(&meta);
    let code = generate_with_options(&spec, file, &sha256, sdk_major_version, opts)
        .map_err(GenerateFromFileError::Generate)?;
    Ok(code)
}

/// Returns the major version of the soroban-sdk recorded in the contract's
/// `rssdkver` meta, such as `30` for `30.0.0#abc123`, or `None` if the meta has
/// no `rssdkver` entry or its version can't be read.
pub fn sdk_major_version_from_meta(meta: &[ScMetaEntry]) -> Option<u32> {
    meta.iter().find_map(|entry| match entry {
        ScMetaEntry::ScMetaV0(v0) if v0.key.as_slice() == b"rssdkver" => v0
            .val
            .to_utf8_string_lossy()
            .split('.')
            .next()
            .and_then(|major| major.parse().ok()),
        _ => None,
    })
}

/// Generates the code for a contract from its spec.
///
/// `sdk_major_version` is the major version of the soroban-sdk that built the
/// contract, as recorded in its `rssdkver` meta, or `None` if not known, which
/// is treated as version 30 or later.
pub fn generate(
    specs: &[ScSpecEntry],
    file: &str,
    sha256: &str,
    sdk_major_version: Option<u32>,
) -> Result<TokenStream, GenerateError> {
    generate_with_options(
        specs,
        file,
        sha256,
        sdk_major_version,
        &GenerateOptions::default(),
    )
}

/// Generates the code for a contract from its spec, with configurable options.
///
/// `sdk_major_version` is the major version of the soroban-sdk that built the
/// contract, as recorded in its `rssdkver` meta, or `None` if not known, which
/// is treated as version 30 or later.
pub fn generate_with_options(
    specs: &[ScSpecEntry],
    file: &str,
    sha256: &str,
    sdk_major_version: Option<u32>,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let generated = generate_without_file_with_options(specs, sdk_major_version, opts)?;
    Ok(quote! {
        pub const WASM: &[u8] = soroban_sdk::contractfile!(file = #file, sha256 = #sha256);
        #generated
    })
}

/// Generates the code for a contract from its spec, without the constant that
/// embeds its wasm.
///
/// `sdk_major_version` is the major version of the soroban-sdk that built the
/// contract, as recorded in its `rssdkver` meta, or `None` if not known, which
/// is treated as version 30 or later.
pub fn generate_without_file(
    specs: &[ScSpecEntry],
    sdk_major_version: Option<u32>,
) -> Result<TokenStream, GenerateError> {
    generate_without_file_with_options(specs, sdk_major_version, &GenerateOptions::default())
}

/// Generates the code for a contract from its spec, without the constant that
/// embeds its wasm, with configurable options.
///
/// `sdk_major_version` is the major version of the soroban-sdk that built the
/// contract, as recorded in its `rssdkver` meta, or `None` if not known, which
/// is treated as version 30 or later.
pub fn generate_without_file_with_options(
    specs: &[ScSpecEntry],
    sdk_major_version: Option<u32>,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    // The error override applies only to specs from SDKs prior to 30, which
    // name their error enum `Error` unqualified, so it runs before the names
    // are reduced.
    let specs = if sdk_major_version.is_some_and(|v| v < ERROR_UDT_OVERRIDE_BEFORE_SDK_MAJOR) {
        apply_error_udt_override(specs)
    } else {
        Cow::Borrowed(specs)
    };
    // The spec names each user-defined type by its fully qualified name
    // (`mycrate::mymod::MyType`), while the generated code names it by a bare
    // identifier, so the names are reduced to simple names before generation,
    // rewriting references to keep them matched up with the types they refer
    // to. Reducing an already-simple spec changes nothing, so a caller that
    // reduced first (to report on the renames) generates the same code.
    //
    // A reduced name can be up to the spec's full name limit, but a generated
    // type's own spec name is its name qualified by the module the code is
    // generated into, which is checked against the same limit. A type whose
    // simple name is within a module path's length of the limit therefore
    // generates code that fails to compile. This is a known limitation, as
    // type names that long are not expected.
    let specs: Vec<ScSpecEntry> = soroban_spec::reduce::reduce(&specs)?
        .into_entries()
        .collect();
    let specs: &[ScSpecEntry] = &specs;

    let mut spec_fns = Vec::new();
    let mut spec_structs = Vec::new();
    let mut spec_unions = Vec::new();
    let mut spec_enums = Vec::new();
    let mut spec_error_enums = Vec::new();
    let mut spec_events = Vec::new();
    for s in specs {
        match s {
            ScSpecEntry::FunctionV0(f) => spec_fns.push(f),
            ScSpecEntry::UdtStructV0(s) => spec_structs.push(s),
            ScSpecEntry::UdtUnionV0(u) => spec_unions.push(u),
            ScSpecEntry::UdtEnumV0(e) => spec_enums.push(e),
            ScSpecEntry::UdtErrorEnumV0(e) => spec_error_enums.push(e),
            ScSpecEntry::EventV0(e) => spec_events.push(e),
        }
    }

    let trait_name = "Contract";

    let trait_ = r#trait::generate_trait(trait_name, &spec_fns)?;
    let structs = spec_structs
        .iter()
        .map(|s| generate_struct_with_options(s, opts))
        .collect::<Result<Vec<_>, _>>()?;
    let unions = spec_unions
        .iter()
        .map(|s| generate_union_with_options(s, opts))
        .collect::<Result<Vec<_>, _>>()?;
    let enums = spec_enums
        .iter()
        .map(|s| generate_enum_with_options(s, opts))
        .collect::<Result<Vec<_>, _>>()?;
    let error_enums = spec_error_enums
        .iter()
        .map(|s| generate_error_enum_with_options(s, opts))
        .collect::<Result<Vec<_>, _>>()?;
    let events = spec_events
        .iter()
        .map(|s| generate_event_with_options(s, opts))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
        #[soroban_sdk::contractargs(name = "Args")]
        #[soroban_sdk::contractclient(name = "Client")]
        #trait_

        #(#structs)*
        #(#unions)*
        #(#enums)*
        #(#error_enums)*
        #(#events)*
    })
}

/// Contracts built with earlier SDKs emit any type named `Error` in their
/// function signatures as the built-in `ScSpecTypeDef::Error` in the spec,
/// regardless of whether the contract defined its own error enum named `Error`
/// or used `soroban_sdk::Error` directly. Current SDKs emit a contract's own
/// error enum as a reference to its fully qualified name, which this pass
/// leaves alone. To let clients of contracts that
/// define their own `Error` enum see the user-defined type instead of
/// `soroban_sdk::Error`, this pass rewrites every `ScSpecTypeDef::Error`
/// reference in the spec to `Udt { name: "Error" }` whenever the spec also
/// contains a `UdtErrorEnumV0` named `Error`.
///
/// A spec that refers to a type by the name `Error` is not from an earlier
/// SDK, as earlier SDKs never wrote the name as a reference, so its
/// `ScSpecTypeDef::Error` references are `soroban_sdk::Error` and are left
/// alone. Such specs come from tools that reduce the names of a current spec,
/// such as the stellar-cli, which leave a contract's own error enum named
/// `Error` and referred to by that name.
///
/// This keeps the on-the-wire spec format unchanged (so already-deployed
/// contracts benefit without redeployment) and shifts the resolution to the
/// client generator.
///
/// Returns a borrowed slice when no rewrite is needed, otherwise a
/// freshly-owned `Vec` with the rewrite applied.
fn apply_error_udt_override(specs: &[ScSpecEntry]) -> Cow<'_, [ScSpecEntry]> {
    let has_error_udt = specs.iter().any(|e| {
        matches!(
            e,
            ScSpecEntry::UdtErrorEnumV0(err) if err.name.to_utf8_string_lossy() == "Error"
        )
    });
    if has_error_udt && !refers_to_error_udt(specs) {
        let mut v = specs.to_vec();
        rewrite_error_to_udt(&mut v);
        Cow::Owned(v)
    } else {
        Cow::Borrowed(specs)
    }
}

/// Whether any entry refers to a user-defined type by the name `Error`.
fn refers_to_error_udt(entries: &[ScSpecEntry]) -> bool {
    fn refers(t: &ScSpecTypeDef) -> bool {
        match t {
            ScSpecTypeDef::Udt(u) => matches!(u.name.as_slice(), b"Error" | b"::Error"),
            ScSpecTypeDef::Option(o) => refers(&o.value_type),
            ScSpecTypeDef::Result(r) => refers(&r.ok_type) || refers(&r.error_type),
            ScSpecTypeDef::Vec(v) => refers(&v.element_type),
            ScSpecTypeDef::Map(m) => refers(&m.key_type) || refers(&m.value_type),
            ScSpecTypeDef::Tuple(tu) => tu.value_types.iter().any(refers),
            _ => false,
        }
    }
    entries.iter().any(|entry| match entry {
        ScSpecEntry::FunctionV0(f) => {
            f.inputs.iter().any(|i| refers(&i.type_)) || f.outputs.iter().any(refers)
        }
        ScSpecEntry::UdtStructV0(s) => s.fields.iter().any(|f| refers(&f.type_)),
        ScSpecEntry::UdtUnionV0(u) => u.cases.iter().any(|c| match c {
            ScSpecUdtUnionCaseV0::TupleV0(t) => t.type_.iter().any(refers),
            ScSpecUdtUnionCaseV0::VoidV0(_) => false,
        }),
        ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => false,
        ScSpecEntry::EventV0(e) => e.params.iter().any(|p| refers(&p.type_)),
    })
}

/// Rewrites every `ScSpecTypeDef::Error` reference in the given entries to
/// `ScSpecTypeDef::Udt { name: "Error" }`. Called only when the spec contains
/// a user-defined error enum named `Error`, so the UDT reference resolves to
/// that enum during code generation.
fn rewrite_error_to_udt(entries: &mut [ScSpecEntry]) {
    fn rewrite_ty(t: &mut ScSpecTypeDef) {
        match t {
            ScSpecTypeDef::Error => {
                *t = ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Error".try_into().unwrap(),
                });
            }
            ScSpecTypeDef::Option(o) => rewrite_ty(&mut o.value_type),
            ScSpecTypeDef::Result(r) => {
                rewrite_ty(&mut r.ok_type);
                rewrite_ty(&mut r.error_type);
            }
            ScSpecTypeDef::Vec(v) => rewrite_ty(&mut v.element_type),
            ScSpecTypeDef::Map(m) => {
                rewrite_ty(&mut m.key_type);
                rewrite_ty(&mut m.value_type);
            }
            ScSpecTypeDef::Tuple(tu) => {
                for vt in tu.value_types.iter_mut() {
                    rewrite_ty(vt);
                }
            }
            _ => {}
        }
    }
    for entry in entries.iter_mut() {
        match entry {
            ScSpecEntry::FunctionV0(f) => {
                for input in f.inputs.iter_mut() {
                    rewrite_ty(&mut input.type_);
                }
                for output in f.outputs.iter_mut() {
                    rewrite_ty(output);
                }
            }
            ScSpecEntry::UdtStructV0(s) => {
                for field in s.fields.iter_mut() {
                    rewrite_ty(&mut field.type_);
                }
            }
            ScSpecEntry::UdtUnionV0(u) => {
                for case in u.cases.iter_mut() {
                    if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                        for ty in t.type_.iter_mut() {
                            rewrite_ty(ty);
                        }
                    }
                }
            }
            ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
            ScSpecEntry::EventV0(e) => {
                for p in e.params.iter_mut() {
                    rewrite_ty(&mut p.type_);
                }
            }
        }
    }
}

/// Implemented by types that can be converted into pretty formatted Strings of
/// Rust code.
pub trait ToFormattedString {
    /// Converts the value to a String that is pretty formatted. If there is any
    /// error parsing the token stream the raw String version of the code is
    /// returned instead.
    fn to_formatted_string(&self) -> Result<String, Error>;
}

impl ToFormattedString for TokenStream {
    fn to_formatted_string(&self) -> Result<String, Error> {
        let file = syn::parse2(self.clone())?;
        Ok(prettyplease::unparse(&file))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::{generate, ToFormattedString};
    use soroban_spec::read::from_wasm;

    const EXAMPLE_WASM: &[u8] = include_bytes!("../../target/wasm32v1-none/release/test_udt.wasm");

    #[test]
    fn example() {
        let entries = from_wasm(EXAMPLE_WASM).unwrap();
        let rust = generate(&entries, "<file>", "<sha256>", None)
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn add(env: soroban_sdk::Env, a: UdtEnum, b: UdtEnum) -> i64;
    fn recursive(env: soroban_sdk::Env, a: UdtRecursive) -> Option<UdtRecursive>;
    fn recursive_enum(
        env: soroban_sdk::Env,
        a: RecursiveEnum,
        key: u32,
    ) -> Result<Option<RecursiveEnum>, soroban_sdk::Error>;
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtTuple(pub i64, pub soroban_sdk::Vec<i64>);
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtStruct {
    pub a: i64,
    pub b: i64,
    pub c: soroban_sdk::Vec<i64>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtRecursive {
    pub a: soroban_sdk::Symbol,
    pub b: soroban_sdk::Vec<UdtRecursive>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RecursiveToEnum {
    pub a: soroban_sdk::Symbol,
    pub b: soroban_sdk::Map<u32, RecursiveEnum>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractExecutableRef {
    pub owner: soroban_sdk::Address,
    pub tag: soroban_sdk::String,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractContext {
    pub args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub contract: soroban_sdk::Address,
    pub fn_name: soroban_sdk::Symbol,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubContractInvocation {
    pub context: ContractContext,
    pub sub_invocations: soroban_sdk::Vec<InvokerContractAuthEntry>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractWithConstructorHostFnContext {
    pub constructor_args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
    UdtC(UdtEnum2),
    UdtD(UdtTuple),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RecursiveEnum {
    NotRecursive,
    Recursive(RecursiveToEnum),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ContractExecutable {
    Wasm(soroban_sdk::BytesN<32>),
    ExternalRef(ContractExecutableRef),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Context {
    Contract(ContractContext),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InvokerContractAuthEntry {
    Contract(SubContractInvocation),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Executable {
    Wasm(soroban_sdk::BytesN<32>),
    StellarAsset,
    Account,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UdtEnum2 {
    A = 10,
    B = 15,
}
"#,
        );
    }

    const ADD_U64_WASM: &[u8] =
        include_bytes!("../../target/wasm32v1-none/release/test_add_u64.wasm");

    /// Test that Result types with user-defined error types are generated correctly.
    /// This specifically tests that:
    /// - An error enum named `Error` generates `Result<u64, Error>` (not `Result<u64, soroban_sdk::Error>`)
    /// - An error enum named `MyError` generates `Result<u64, MyError>`
    #[test]
    fn test_add_u64_result_types() {
        let entries = from_wasm(ADD_U64_WASM).unwrap();
        let rust = generate(&entries, "<file>", "<sha256>", None)
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn add(env: soroban_sdk::Env, a: u64, b: u64) -> u64;
    fn safe_add(env: soroban_sdk::Env, a: u64, b: u64) -> Result<u64, Error>;
    fn safe_add_two(env: soroban_sdk::Env, a: u64, b: u64) -> Result<u64, MyError>;
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractExecutableRef {
    pub owner: soroban_sdk::Address,
    pub tag: soroban_sdk::String,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractContext {
    pub args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub contract: soroban_sdk::Address,
    pub fn_name: soroban_sdk::Symbol,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubContractInvocation {
    pub context: ContractContext,
    pub sub_invocations: soroban_sdk::Vec<InvokerContractAuthEntry>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractWithConstructorHostFnContext {
    pub constructor_args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ContractExecutable {
    Wasm(soroban_sdk::BytesN<32>),
    ExternalRef(ContractExecutableRef),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Context {
    Contract(ContractContext),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InvokerContractAuthEntry {
    Contract(SubContractInvocation),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Executable {
    Wasm(soroban_sdk::BytesN<32>),
    StellarAsset,
    Account,
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MyError {
    Overflow = 1,
}
"#,
        );
    }

    /// Test that shows the raw spec entries from the wasm.
    /// Verifies that a contract error enum is emitted as a UDT reference to
    /// its qualified name in function signatures, whether it is named `Error`
    /// or something else (`MyError`).
    #[test]
    fn test_add_u64_spec_entries() {
        use super::ScSpecEntry;
        use stellar_xdr::ScSpecTypeDef;

        let entries = from_wasm(ADD_U64_WASM).unwrap();

        // Find the safe_add function spec
        let safe_add_fn = entries
            .iter()
            .find_map(|e| match e {
                ScSpecEntry::FunctionV0(f) if f.name.to_utf8_string().unwrap() == "safe_add" => {
                    Some(f)
                }
                _ => None,
            })
            .expect("safe_add function not found");

        let output = safe_add_fn.outputs.to_option().expect("should have output");
        let ScSpecTypeDef::Result(r) = output else {
            panic!("output should be a Result type");
        };
        assert!(
            matches!(r.ok_type.as_ref(), ScSpecTypeDef::U64),
            "ok_type should be U64"
        );
        let ScSpecTypeDef::Udt(u) = r.error_type.as_ref() else {
            panic!(
                "error_type should be a UDT for Error, got {:?}",
                r.error_type
            );
        };
        assert_eq!(
            u.name.to_utf8_string().unwrap(),
            "::test_add_u64::Error",
            "error_type should be Error UDT"
        );

        // Find the safe_add_two function spec
        let safe_add_two_fn = entries
            .iter()
            .find_map(|e| match e {
                ScSpecEntry::FunctionV0(f)
                    if f.name.to_utf8_string().unwrap() == "safe_add_two" =>
                {
                    Some(f)
                }
                _ => None,
            })
            .expect("safe_add_two function not found");

        let output = safe_add_two_fn
            .outputs
            .to_option()
            .expect("should have output");
        let ScSpecTypeDef::Result(r) = output else {
            panic!("output should be a Result type");
        };
        assert!(
            matches!(r.ok_type.as_ref(), ScSpecTypeDef::U64),
            "ok_type should be U64"
        );
        let ScSpecTypeDef::Udt(u) = r.error_type.as_ref() else {
            panic!(
                "error_type should be a UDT for MyError, got {:?}",
                r.error_type
            );
        };
        assert_eq!(
            u.name.to_utf8_string().unwrap(),
            "::test_add_u64::MyError",
            "error_type should be MyError UDT"
        );
    }

    /// When the spec references `ScSpecTypeDef::Error` and contains no error
    /// enum named `Error`, the generator must leave it as `soroban_sdk::Error`.
    /// This covers contracts that use `soroban_sdk::Error` directly as their
    /// Result error type, including every contract compiled before the
    /// error-enum override was introduced.
    #[test]
    fn test_missing_error_udt_falls_back_to_sdk_error() {
        use super::ScSpecEntry;
        use stellar_xdr::{ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult};

        let func = ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "safe_add".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::U64),
                error_type: Box::new(ScSpecTypeDef::Error),
            }))]
            .try_into()
            .unwrap(),
        };
        let entries = [ScSpecEntry::FunctionV0(func)];
        let rust = generate(&entries, "<file>", "<sha256>", Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn safe_add(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
"#,
        );
    }

    /// When the spec contains a user-defined `Error` error enum, every
    /// `ScSpecTypeDef::Error` reference in the spec must be rewritten to
    /// reference that UDT instead of `soroban_sdk::Error`.
    #[test]
    fn test_error_udt_overrides_sdk_error() {
        use super::ScSpecEntry;
        use stellar_xdr::{
            ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecUdtErrorEnumCaseV0,
            ScSpecUdtErrorEnumV0,
        };

        let func = ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "safe_add".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::U64),
                error_type: Box::new(ScSpecTypeDef::Error),
            }))]
            .try_into()
            .unwrap(),
        };
        let error_enum = ScSpecUdtErrorEnumV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "Error".try_into().unwrap(),
            cases: [ScSpecUdtErrorEnumCaseV0 {
                doc: "".try_into().unwrap(),
                name: "Overflow".try_into().unwrap(),
                value: 1,
            }]
            .try_into()
            .unwrap(),
        };
        let entries = [
            ScSpecEntry::FunctionV0(func),
            ScSpecEntry::UdtErrorEnumV0(error_enum),
        ];
        let rust = generate(&entries, "<file>", "<sha256>", Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn safe_add(env: soroban_sdk::Env) -> Result<u64, Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// When the spec refers to its `Error` error enum by name, as a reduced
    /// spec from a current SDK does, its `ScSpecTypeDef::Error` references are
    /// `soroban_sdk::Error` and must not be rewritten.
    #[test]
    fn test_error_udt_referred_to_by_name_leaves_sdk_error() {
        use super::ScSpecEntry;
        use stellar_xdr::{
            ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let sdk_error_fn = ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "sdk_error".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::U64),
                error_type: Box::new(ScSpecTypeDef::Error),
            }))]
            .try_into()
            .unwrap(),
        };
        let own_error_fn = ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "own_error".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::U64),
                error_type: Box::new(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "Error".try_into().unwrap(),
                })),
            }))]
            .try_into()
            .unwrap(),
        };
        let error_enum = ScSpecUdtErrorEnumV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "Error".try_into().unwrap(),
            cases: [ScSpecUdtErrorEnumCaseV0 {
                doc: "".try_into().unwrap(),
                name: "Overflow".try_into().unwrap(),
                value: 1,
            }]
            .try_into()
            .unwrap(),
        };
        let entries = [
            ScSpecEntry::FunctionV0(sdk_error_fn),
            ScSpecEntry::FunctionV0(own_error_fn),
            ScSpecEntry::UdtErrorEnumV0(error_enum),
        ];
        let rust = generate(&entries, "<file>", "<sha256>", Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
    fn own_error(env: soroban_sdk::Env) -> Result<u64, Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// When the `Error` override applies, nested `ScSpecTypeDef::Error`
    /// references must be rewritten too.
    #[test]
    fn test_error_udt_override_rewrites_nested_vec() {
        use super::ScSpecEntry;
        use stellar_xdr::{
            ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeVec, ScSpecUdtErrorEnumCaseV0,
            ScSpecUdtErrorEnumV0,
        };

        let func = ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "errors".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                element_type: Box::new(ScSpecTypeDef::Error),
            }))]
            .try_into()
            .unwrap(),
        };
        let error_enum = ScSpecUdtErrorEnumV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "Error".try_into().unwrap(),
            cases: [ScSpecUdtErrorEnumCaseV0 {
                doc: "".try_into().unwrap(),
                name: "Overflow".try_into().unwrap(),
                value: 1,
            }]
            .try_into()
            .unwrap(),
        };
        let entries = [
            ScSpecEntry::FunctionV0(func),
            ScSpecEntry::UdtErrorEnumV0(error_enum),
        ];
        let rust = generate(&entries, "<file>", "<sha256>", Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"pub const WASM: &[u8] = soroban_sdk::contractfile!(file = "<file>", sha256 = "<sha256>");
#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn errors(env: soroban_sdk::Env) -> soroban_sdk::Vec<Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// Two user-defined error enums sharing the simple name `Error`
    /// (`a::Error`, `b::Error`), taken end-to-end through name
    /// reduction and code generation.
    #[test]
    fn test_two_error_enums_sharing_a_simple_name() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let error_enum = |name: &str| {
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: name.try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Failed".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            })
        };
        let func = |name: &str, error: ScSpecTypeDef| {
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: name.try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U32),
                    error_type: Box::new(error),
                }))]
                .try_into()
                .unwrap(),
            })
        };
        let entries = [
            error_enum("mycrate::a::Error"),
            error_enum("mycrate::b::Error"),
            func(
                "use_a",
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "mycrate::a::Error".try_into().unwrap(),
                }),
            ),
            func(
                "use_b",
                ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "mycrate::b::Error".try_into().unwrap(),
                }),
            ),
            func("use_builtin", ScSpecTypeDef::Error),
        ];

        let code = super::generate_without_file(&entries, None)
            .unwrap()
            .to_formatted_string()
            .unwrap();

        // Each enum is generated in full, so it is clear which one kept the
        // simple name (the first, `a::Error`) and which was numbered (the
        // second, `b::Error` -> `Error2`); both carry the same `Failed` case.
        assert!(
            code.contains("pub enum Error {\n    Failed = 1,\n}"),
            "{code}"
        );
        assert!(
            code.contains("pub enum Error2 {\n    Failed = 1,\n}"),
            "{code}"
        );
        // Each function is asserted by name so the reference is clearly tied to
        // its own type: `use_a` follows `a::Error` to `Error`, `use_b` follows
        // `b::Error` to `Error2`, and `use_builtin`'s built-in error stays
        // `soroban_sdk::Error`, as the enums have qualified names.
        assert!(
            code.contains("fn use_a(env: soroban_sdk::Env) -> Result<u32, Error>;"),
            "{code}"
        );
        assert!(
            code.contains("fn use_b(env: soroban_sdk::Env) -> Result<u32, Error2>;"),
            "{code}"
        );
        assert!(
            code.contains(
                "fn use_builtin(env: soroban_sdk::Env) -> Result<u32, soroban_sdk::Error>;"
            ),
            "{code}"
        );
    }

    // How the generated code refers to error types, for each kind of spec:
    // from an old SDK (prior to 30, before fully qualified names), from a new
    // SDK (30 or later) with its names unreduced, and from a new SDK with its
    // names reduced. Each spec has the contract's own error enum,
    // `soroban_sdk::Error`, or both.

    /// Old SDK, with the contract's own error enum named `Error`. Old SDKs wrote
    /// any type named `Error` as the built-in `ScSpecTypeDef::Error`, so the
    /// contract's own `Error` and `soroban_sdk::Error` look the same in the
    /// spec. Both are generated as the contract's own `Error`.
    #[test]
    fn test_error_old_sdk_own_error_named_error() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let entries = [
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: "Error".try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Overflow".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "own_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "sdk_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
        ];
        let rust = super::generate_without_file(&entries, Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn own_error(env: soroban_sdk::Env) -> Result<u64, Error>;
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// Old SDK, with the contract's own error enum named something other than
    /// `Error`. The own error enum is referred to by name, and
    /// `soroban_sdk::Error` stays `soroban_sdk::Error`.
    #[test]
    fn test_error_old_sdk_own_error_named_other() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let entries = [
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: "MyError".try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Overflow".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "own_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                        name: "MyError".try_into().unwrap(),
                    })),
                }))]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "sdk_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
        ];
        let rust = super::generate_without_file(&entries, Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn own_error(env: soroban_sdk::Env) -> Result<u64, MyError>;
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MyError {
    Overflow = 1,
}
"#,
        );
    }

    /// Old SDK, with no error enum of the contract's own. `soroban_sdk::Error`
    /// stays `soroban_sdk::Error`.
    #[test]
    fn test_error_old_sdk_no_own_error() {
        use stellar_xdr::{ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult};

        let entries = [ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "sdk_error".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::U64),
                error_type: Box::new(ScSpecTypeDef::Error),
            }))]
            .try_into()
            .unwrap(),
        })];
        let rust = super::generate_without_file(&entries, Some(22))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
"#,
        );
    }

    /// New SDK, spec not reduced, with the contract's own error enum named
    /// `Error` and used. The own error enum is referred to by its fully
    /// qualified name, which is reduced to `Error` during generation, and
    /// `soroban_sdk::Error` stays `soroban_sdk::Error`.
    #[test]
    fn test_error_new_sdk_unreduced_own_error_used() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let entries = [
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: "::mycontract::Error".try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Overflow".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "own_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                        name: "::mycontract::Error".try_into().unwrap(),
                    })),
                }))]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "sdk_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
        ];
        let rust = super::generate_without_file(&entries, Some(30))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn own_error(env: soroban_sdk::Env) -> Result<u64, Error>;
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// New SDK, spec not reduced, with the contract's own error enum named
    /// `Error` but not used by any function. `soroban_sdk::Error` stays
    /// `soroban_sdk::Error`.
    #[test]
    fn test_error_new_sdk_unreduced_own_error_unused() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let entries = [
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: "::mycontract::Error".try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Overflow".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "sdk_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
        ];
        let rust = super::generate_without_file(&entries, Some(30))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// New SDK, spec reduced (as the stellar-cli does when building), with the
    /// contract's own error enum named `Error` and used. The own error enum is
    /// referred to by its reduced name `Error`, and `soroban_sdk::Error` stays
    /// `soroban_sdk::Error`. Same as the unreduced spec.
    #[test]
    fn test_error_new_sdk_reduced_own_error_used() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult, ScSpecTypeUdt,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let entries = [
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: "Error".try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Overflow".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "own_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                        name: "Error".try_into().unwrap(),
                    })),
                }))]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "sdk_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
        ];
        let rust = super::generate_without_file(&entries, Some(30))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn own_error(env: soroban_sdk::Env) -> Result<u64, Error>;
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }

    /// New SDK, spec reduced (as the stellar-cli does when building), with the
    /// contract's own error enum named `Error` but not used by any function.
    /// The spec looks like an old SDK's, but the old SDK override applies only
    /// to specs from SDKs prior to 30, so `soroban_sdk::Error` stays
    /// `soroban_sdk::Error`. Same as the unreduced spec.
    #[test]
    fn test_error_new_sdk_reduced_own_error_unused() {
        use stellar_xdr::{
            ScSpecEntry, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeResult,
            ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        };

        let entries = [
            ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: "Error".try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Overflow".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "".try_into().unwrap(),
                name: "sdk_error".try_into().unwrap(),
                inputs: [].try_into().unwrap(),
                outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                    ok_type: Box::new(ScSpecTypeDef::U64),
                    error_type: Box::new(ScSpecTypeDef::Error),
                }))]
                .try_into()
                .unwrap(),
            }),
        ];
        let rust = super::generate_without_file(&entries, Some(30))
            .unwrap()
            .to_formatted_string()
            .unwrap();
        assert_eq!(
            rust,
            r#"#[soroban_sdk::contractargs(name = "Args")]
#[soroban_sdk::contractclient(name = "Client")]
pub trait Contract {
    fn sdk_error(env: soroban_sdk::Env) -> Result<u64, soroban_sdk::Error>;
}
#[soroban_sdk::contracterror]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }
}
