mod syn_ext;
pub mod r#trait;
pub mod types;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::{fs, io};

use proc_macro2::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use stellar_xdr::{ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtUnionCaseV0, StringM};
use syn::Error;

use soroban_spec::read::{from_wasm, FromWasmError};
use soroban_spec::udt_id::canonical_id;

use types::{
    generate_enum_with_options, generate_error_enum_with_options, generate_event_with_options,
    generate_struct_with_options, generate_union_with_options,
};
pub use types::{GenerateError, GenerateOptions};

// IMPORTANT: The "docs" fields of spec entries are not output in Rust token
// streams as rustdocs, because rustdocs can contain Rust code, and that code
// will be executed. Generated code may be generated from untrusted Wasm
// containing untrusted spec docs.

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

pub fn generate_from_wasm(
    wasm: &[u8],
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<TokenStream, GenerateFromFileError> {
    generate_from_wasm_with_options(wasm, file, verify_sha256, &GenerateOptions::default())
}

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
    let code = generate_with_options(&spec, file, &sha256, opts)
        .map_err(GenerateFromFileError::Generate)?;
    Ok(code)
}

pub fn generate(
    specs: &[ScSpecEntry],
    file: &str,
    sha256: &str,
) -> Result<TokenStream, GenerateError> {
    generate_with_options(specs, file, sha256, &GenerateOptions::default())
}

pub fn generate_with_options(
    specs: &[ScSpecEntry],
    file: &str,
    sha256: &str,
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    let generated = generate_without_file_with_options(specs, opts)?;
    Ok(quote! {
        pub const WASM: &[u8] = soroban_sdk::contractfile!(file = #file, sha256 = #sha256);
        #generated
    })
}

pub fn generate_without_file(specs: &[ScSpecEntry]) -> Result<TokenStream, GenerateError> {
    generate_without_file_with_options(specs, &GenerateOptions::default())
}

pub fn generate_without_file_with_options(
    specs: &[ScSpecEntry],
    opts: &GenerateOptions,
) -> Result<TokenStream, GenerateError> {
    // Collected before the rewrites below, because the id a reference carries is
    // derived from the referenced entry's own contents.
    let names = udt_names(specs);
    let mut specs = apply_error_udt_override(specs).into_owned();
    resolve_udt_v2_names(&mut specs, &names);
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

/// The `#[contractimpl]` macro emits any type named `Error` in a contract's
/// function signatures as the built-in `ScSpecTypeDef::Error` in the spec,
/// regardless of whether the contract defined its own error enum named `Error`
/// or used `soroban_sdk::Error` directly. To let clients of contracts that
/// define their own `Error` enum see the user-defined type instead of
/// `soroban_sdk::Error`, this pass rewrites every `ScSpecTypeDef::Error`
/// reference in the spec to `Udt { name: "Error" }` whenever the spec also
/// contains a `UdtErrorEnumV0` named `Error`.
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
    if has_error_udt {
        let mut v = specs.to_vec();
        rewrite_error_to_udt(&mut v);
        Cow::Owned(v)
    } else {
        Cow::Borrowed(specs)
    }
}

/// Rewrites every `ScSpecTypeDef::Error` reference in the given entries to
/// `ScSpecTypeDef::Udt { name: "Error" }`. Called only when the spec contains
/// a user-defined error enum named `Error`, so the UDT reference resolves to
/// that enum during code generation.
fn rewrite_error_to_udt(entries: &mut [ScSpecEntry]) {
    for_each_type_def(entries, &mut |t| {
        if matches!(t, ScSpecTypeDef::Error) {
            *t = ScSpecTypeDef::Udt(ScSpecTypeUdt {
                name: "Error".try_into().unwrap(),
            });
        }
    });
}

/// Calls `f` on every type in `entries`, including the types nested inside the
/// parameterized ones.
fn for_each_type_def(entries: &mut [ScSpecEntry], f: &mut impl FnMut(&mut ScSpecTypeDef)) {
    fn visit(t: &mut ScSpecTypeDef, f: &mut impl FnMut(&mut ScSpecTypeDef)) {
        f(t);
        match t {
            ScSpecTypeDef::Option(o) => visit(&mut o.value_type, f),
            ScSpecTypeDef::Result(r) => {
                visit(&mut r.ok_type, f);
                visit(&mut r.error_type, f);
            }
            ScSpecTypeDef::Vec(v) => visit(&mut v.element_type, f),
            ScSpecTypeDef::Map(m) => {
                visit(&mut m.key_type, f);
                visit(&mut m.value_type, f);
            }
            ScSpecTypeDef::Tuple(tu) => {
                for vt in tu.value_types.iter_mut() {
                    visit(vt, f);
                }
            }
            _ => {}
        }
    }
    for entry in entries.iter_mut() {
        match entry {
            ScSpecEntry::FunctionV0(fun) => {
                for input in fun.inputs.iter_mut() {
                    visit(&mut input.type_, f);
                }
                for output in fun.outputs.iter_mut() {
                    visit(output, f);
                }
            }
            ScSpecEntry::UdtStructV0(s) => {
                for field in s.fields.iter_mut() {
                    visit(&mut field.type_, f);
                }
            }
            ScSpecEntry::UdtUnionV0(u) => {
                for case in u.cases.iter_mut() {
                    if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                        for ty in t.type_.iter_mut() {
                            visit(ty, f);
                        }
                    }
                }
            }
            ScSpecEntry::UdtEnumV0(_) | ScSpecEntry::UdtErrorEnumV0(_) => {}
            ScSpecEntry::EventV0(e) => {
                for p in e.params.iter_mut() {
                    visit(&mut p.type_, f);
                }
            }
        }
    }
}

/// Rewrites every `ScSpecTypeDef::UdtV2` reference in `entries`, which
/// identifies the type it references only by an id, to the `ScSpecTypeDef::Udt`
/// naming that type, so that generation has a name to build an ident from.
///
/// `names` maps the id of every user-defined type the spec defines to its name,
/// and must be built from the entries as they were read, because an entry's id
/// is derived from its own contents.
fn resolve_udt_v2_names(entries: &mut [ScSpecEntry], names: &BTreeMap<[u8; 8], StringM<60>>) {
    for_each_type_def(entries, &mut |t| {
        if let ScSpecTypeDef::UdtV2(u) = t {
            if let Some(name) = names.get(&u.id) {
                *t = ScSpecTypeDef::Udt(ScSpecTypeUdt { name: name.clone() });
            }
        }
    });
}

/// The name of every user-defined type the spec defines, keyed by the id that
/// references to it carry.
fn udt_names(entries: &[ScSpecEntry]) -> BTreeMap<[u8; 8], StringM<60>> {
    entries
        .iter()
        .filter_map(|entry| {
            let name = match entry {
                ScSpecEntry::UdtStructV0(s) => &s.name,
                ScSpecEntry::UdtUnionV0(u) => &u.name,
                ScSpecEntry::UdtEnumV0(e) => &e.name,
                ScSpecEntry::UdtErrorEnumV0(e) => &e.name,
                ScSpecEntry::FunctionV0(_) | ScSpecEntry::EventV0(_) => return None,
            };
            Some((canonical_id(entry), name.clone()))
        })
        .collect()
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
        let rust = generate(&entries, "<file>", "<sha256>")
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
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtTuple(pub i64, pub soroban_sdk::Vec<i64>);
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtStruct {
    pub a: i64,
    pub b: i64,
    pub c: soroban_sdk::Vec<i64>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct UdtRecursive {
    pub a: soroban_sdk::Symbol,
    pub b: soroban_sdk::Vec<UdtRecursive>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RecursiveToEnum {
    pub a: soroban_sdk::Symbol,
    pub b: soroban_sdk::Map<u32, RecursiveEnum>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractContext {
    pub args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub contract: soroban_sdk::Address,
    pub fn_name: soroban_sdk::Symbol,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubContractInvocation {
    pub context: ContractContext,
    pub sub_invocations: soroban_sdk::Vec<InvokerContractAuthEntry>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractWithConstructorHostFnContext {
    pub constructor_args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
    UdtC(UdtEnum2),
    UdtD(UdtTuple),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RecursiveEnum {
    NotRecursive,
    Recursive(RecursiveToEnum),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Context {
    Contract(ContractContext),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ContractExecutable {
    Wasm(soroban_sdk::BytesN<32>),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InvokerContractAuthEntry {
    Contract(SubContractInvocation),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Executable {
    Wasm(soroban_sdk::BytesN<32>),
    StellarAsset,
    Account,
}
#[soroban_sdk::contracttype(export = false)]
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
        let rust = generate(&entries, "<file>", "<sha256>")
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
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ContractContext {
    pub args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub contract: soroban_sdk::Address,
    pub fn_name: soroban_sdk::Symbol,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SubContractInvocation {
    pub context: ContractContext,
    pub sub_invocations: soroban_sdk::Vec<InvokerContractAuthEntry>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractHostFnContext {
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct CreateContractWithConstructorHostFnContext {
    pub constructor_args: soroban_sdk::Vec<soroban_sdk::Val>,
    pub executable: ContractExecutable,
    pub salt: soroban_sdk::BytesN<32>,
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Context {
    Contract(ContractContext),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ContractExecutable {
    Wasm(soroban_sdk::BytesN<32>),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum InvokerContractAuthEntry {
    Contract(SubContractInvocation),
    CreateContractHostFn(CreateContractHostFnContext),
    CreateContractWithCtorHostFn(CreateContractWithConstructorHostFnContext),
}
#[soroban_sdk::contracttype(export = false)]
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Executable {
    Wasm(soroban_sdk::BytesN<32>),
    StellarAsset,
    Account,
}
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MyError {
    Overflow = 1,
}
"#,
        );
    }

    /// Test that shows the raw spec entries from the wasm.
    /// Verifies that the on-the-wire spec format is unchanged: a contract
    /// error enum named `Error` is still emitted as the built-in
    /// `ScSpecTypeDef::Error` in function signatures (the user-defined-vs-SDK
    /// disambiguation happens at client generation time, not here). A
    /// differently-named error enum (`MyError`) is emitted as a UDT reference.
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
        assert!(
            matches!(r.error_type.as_ref(), ScSpecTypeDef::Error),
            "error_type should be the built-in Error in the wasm spec, got {:?}",
            r.error_type
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
        let ScSpecTypeDef::UdtV2(u) = r.error_type.as_ref() else {
            panic!(
                "error_type should be a UDT for MyError, got {:?}",
                r.error_type
            );
        };
        // The reference carries only an id, so it is the id of the MyError
        // entry in the same spec that identifies it as the referenced type.
        let my_error = entries
            .iter()
            .find(|e| {
                matches!(e, ScSpecEntry::UdtErrorEnumV0(err)
                    if err.name.to_utf8_string().unwrap() == "MyError")
            })
            .expect("MyError error enum not found");
        assert_eq!(
            u.id,
            super::canonical_id(my_error),
            "error_type should reference the MyError UDT"
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
        let rust = generate(&entries, "<file>", "<sha256>")
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
        let rust = generate(&entries, "<file>", "<sha256>")
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
#[soroban_sdk::contracterror(export = false)]
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
        let rust = generate(&entries, "<file>", "<sha256>")
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
#[soroban_sdk::contracterror(export = false)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    Overflow = 1,
}
"#,
        );
    }
}
