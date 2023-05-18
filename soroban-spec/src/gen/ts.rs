use std::{fs, io};

use super::json::types;

use itertools::Itertools;
use sha2::{Digest, Sha256};
use stellar_xdr::ScSpecEntry;

use types::Entry;

use crate::{
    gen::ts::wrapper::type_to_js_xdr,
    read::{from_wasm, FromWasmError},
};

pub mod boilerplate;
pub mod wrapper;

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
}

pub fn generate_from_file(
    file: &str,
    verify_sha256: Option<&str>,
) -> Result<String, GenerateFromFileError> {
    // Read file.
    let wasm = fs::read(file).map_err(GenerateFromFileError::Io)?;

    // Produce hash for file.
    let sha256 = Sha256::digest(&wasm);
    let sha256 = format!("{sha256:x}");

    if let Some(verify_sha256) = verify_sha256 {
        if verify_sha256 != sha256 {
            return Err(GenerateFromFileError::VerifySha256 { expected: sha256 });
        }
    }

    // Generate code.
    let json = generate_from_wasm(&wasm).map_err(GenerateFromFileError::GetSpec)?;
    Ok(json)
}

pub fn generate_from_wasm(wasm: &[u8]) -> Result<String, FromWasmError> {
    let spec = from_wasm(wasm)?;
    let json = generate(&spec);
    Ok(json)
}

pub fn generate(spec: &[ScSpecEntry]) -> String {
    let collected: Vec<_> = spec.iter().map(Entry::from).collect();
    collected.iter().map(entry_to_ts).join("\n")
}

fn doc_to_ts_doc(doc: &str) -> String {
    if doc.is_empty() {
        String::new()
    } else {
        let doc = doc.split('\n').join("\n * ");
        format!(
            r#"/**
 * {doc}
 */
"#,
        )
    }
}

pub fn entry_to_ts(entry: &Entry) -> String {
    match entry {
        Entry::Function {
            doc,
            name,
            inputs,
            outputs,
        } => {
            let sign_me = doc
                .contains("@signme")
                .then_some(r#""sign": true, "#)
                .unwrap_or_default();
            let args = inputs
                .iter()
                .map(|i| format!("((i) =>{})({})", type_to_js_xdr(&i.value), i.name))
                .join(", ");
            let input = (!inputs.is_empty())
                .then(|| {
                    format!(
                        "{{{}}}: {{{}}}",
                        inputs.iter().map(func_input_to_arg_name).join(", "),
                        inputs.iter().map(func_input_to_ts).join(", ")
                    )
                })
                .unwrap_or_default();
            let return_type = if outputs.is_empty() {
                "".to_owned()
            } else if outputs.len() == 1 {
                format!(": Promise<{}>", type_to_ts(&outputs[0]))
            } else {
                format!(
                    ": Promise<Tuple<{}>>",
                    outputs.iter().map(type_to_ts).join(", ")
                )
            };
            let ts_doc = doc_to_ts_doc(doc);

            // let output_parser = outputs.get(0).map(scVal_to_type).unwrap_or_default();
            let output = (!outputs.is_empty())
                .then(|| {
                    format!(
                        r#"
    return scValToJs(SorobanClient.xdr.ScVal.fromXDR(Buffer.from(response.xdr, 'base64'))) as {}
"#,
                        type_to_ts(&outputs[0])
                    )
                })
                .unwrap_or_default();
            format!(
                r#"{ts_doc}export async function {name}({input}){return_type} {{
    let invokeArgs: InvokeArgs = {{{sign_me}method: '{name}', args: [{args}]}};
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);{output}
    
}}
"#
            )
        }
        Entry::Struct { doc, name, fields } => {
            let docs = doc_to_ts_doc(doc);
            let fields = fields.iter().map(field_to_ts).join("\n  ");
            format!(
                r#"{docs}export interface {name} {{
  {fields}
}}
"#
            )
        }

        Entry::Union { doc, name, cases } => {
            String::new()
            // let doc = doc_to_ts_doc(doc);
            // let cases = cases.iter().map(case_to_ts).join(" | ");
            //             format!(
            //                 r#"{doc}export type {name} = {cases};
            // "#
            //             )
        }
        Entry::Enum { doc, name, cases } => {
            let doc = doc_to_ts_doc(doc);
            let cases = cases.iter().map(enum_case_to_ts).join("\n  ");
            format!(
                r#"{doc}export enum {name} {{
  {cases}
}}
"#
            )
        }
        Entry::ErrorEnum { .. } => todo!(),
    }
}

fn enum_case_to_ts(case: &types::EnumCase) -> String {
    let types::EnumCase { name, value, .. } = case;
    format!("{name} = {value},")
}

fn case_to_ts(case: &types::UnionCase) -> String {
    let types::UnionCase { name, .. } = case;
    name.to_string()
}

fn field_to_ts(field: &types::StructField) -> String {
    let types::StructField { doc, name, value } = field;
    let doc = doc_to_ts_doc(doc);
    let type_ = type_to_ts(value);
    format!("{doc}{name}: {type_};")
}

pub fn func_input_to_ts(input: &types::FunctionInput) -> String {
    let types::FunctionInput { name, value, .. } = input;
    let type_ = type_to_ts(value);
    format!("{name}: {type_}")
}

pub fn func_input_to_arg_name(input: &types::FunctionInput) -> String {
    let types::FunctionInput { name, .. } = input;
    name.to_string()
}

pub fn type_to_ts(value: &types::Type) -> String {
    match value {
        types::Type::Val => todo!(),
        types::Type::U64 => "u64".to_owned(),
        types::Type::I64 => "i64".to_owned(),
        types::Type::U128 => "u128".to_owned(),
        types::Type::I128 => "i128".to_owned(),
        types::Type::U32 => "u32".to_owned(),
        types::Type::I32 => "i32".to_owned(),
        types::Type::Bool => "boolean".to_owned(),
        types::Type::Symbol => "string".to_owned(),
        types::Type::Map { key, value } => {
            format!("Map<{}, {}>", type_to_ts(key), type_to_ts(value))
        }
        types::Type::Option { value } => format!("{}?", type_to_ts(value)),
        types::Type::Result { value, error } => {
            format!("Result<{}, {}>", type_to_ts(value), type_to_ts(error))
        }
        types::Type::Set { element } => format!("Set<{}>", type_to_ts(element)),
        types::Type::Vec { element } => format!("Array<{}>", type_to_ts(element)),
        types::Type::Tuple { elements } => {
            if elements.is_empty() {
                "void".to_owned()
            } else {
                format!("[{}]", elements.iter().map(type_to_ts).join(", "))
            }
        }
        types::Type::Custom { name } => name.to_owned(),
        types::Type::Status => todo!(),
        types::Type::Address => "Address".to_string(),
        types::Type::Bytes => "Buffer".to_string(),
        types::Type::BytesN { .. } => "".to_string(),
        types::Type::Void => "void".to_owned(),
        types::Type::U256 => todo!(),
        types::Type::I256 => todo!(),
        types::Type::String => todo!(),
        types::Type::Timepoint => todo!(),
        types::Type::Duration => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::generate;

    const EXAMPLE_WASM: &[u8] =
        include_bytes!("../../../../soroban-abundance-token/target/wasm32-unknown-unknown/release/abundance_token.wasm");

    #[test]
    fn example() {
        let entries = crate::read::from_wasm(EXAMPLE_WASM).unwrap();
        let ts = generate(&entries);
        println!("{ts}");
    }
}
