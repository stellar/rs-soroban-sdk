use std::{fs, io};

use super::json::types::{self, StructField, UnionCase};

use heck::ToLowerCamelCase;
use itertools::Itertools;
use sha2::{Digest, Sha256};
use stellar_xdr::ScSpecEntry;

use types::Entry;

use crate::{
    gen::{json::types::Type, typescript::wrapper::type_to_js_xdr},
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
            let args = inputs
                .iter()
                .map(|i| format!("((i) => {})({})", type_to_js_xdr(&i.value), i.name))
                .join(",\n        ");
            let input = (!inputs.is_empty())
                .then(|| {
                    format!(
                        "{{{}}}: {{{}}},",
                        inputs.iter().map(func_input_to_arg_name).join(", "),
                        inputs.iter().map(func_input_to_ts).join(", ")
                    )
                })
                .unwrap_or_default();
            let mut is_result = false;
            let mut inner_return_type = String::new();
            let return_type = if outputs.is_empty() {
                ": Promise<void>".to_owned()
            } else if outputs.len() == 1 {
                inner_return_type = type_to_ts(&outputs[0]);
                is_result = inner_return_type.starts_with("Result<");
                format!(": Promise<{inner_return_type}>")
            } else {
                format!(
                    ": Promise<[{}]>>",
                    outputs.iter().map(type_to_ts).join(", ")
                )
            };
            let ts_doc = doc_to_ts_doc(doc);

            // let output_parser = outputs.get(0).map(scVal_to_type).unwrap_or_default();
            if is_result {
                inner_return_type = inner_return_type
                    .strip_prefix("Result<")
                    .unwrap()
                    .strip_suffix('>')
                    .unwrap()
                    .to_owned();
            }

            let mut output = outputs
                .get(0)
                .map(|type_| match type_ {
                    Type::Custom { name } => format!("{name}FromXdr(response.xdr)"),
                    _ => format!("scValStrToJs(response.xdr) as {inner_return_type}"),
                })
                .unwrap_or_default();
            if is_result {
                output = format!("new Ok({output})");
            }
            let mut output = format!(
                r#"
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return {output};"#
            );
            if is_result {
                output = format!(
                    r#"
    try {{
        {output}
    }} catch (e) {{
        //@ts-ignore
        let err = getError(e.message);
        if (err) {{
            return err;
        }} else {{
            throw e;
        }}
    }}"#
                );
            }
            let args = (!inputs.is_empty())
                .then(|| format!("args: [{args}], "))
                .unwrap_or_default();
            format!(
                r#"{ts_doc}export async function {name}({input} {{signAndSend, fee}}: {{signAndSend?: boolean, fee?: number}} = {{signAndSend: false, fee: 100}}){return_type} {{
    let invokeArgs: InvokeArgs = {{
        signAndSend,
        fee,
        method: '{name}', 
        {args}
    }};
    {output}
}}
"#
            )
        }
        Entry::Struct { doc, name, fields } => {
            let docs = doc_to_ts_doc(doc);
            let arg_name = name.to_lower_camel_case();
            let encoded_fields = js_to_xdr_fields(&arg_name, fields);
            let decoded_fields = js_from_xdr_fields(fields);
            let fields = fields.iter().map(field_to_ts).join("\n  ");
            let void = type_to_js_xdr(&Type::Void);
            format!(
                r#"{docs}export interface {name} {{
  {fields}
}}

function {name}ToXDR({arg_name}?: {name}): xdr.ScVal {{
    if (!{arg_name}) {{
        return {void};
    }}
    let arr = [
        {encoded_fields}
        ];
    return xdr.ScVal.scvMap(arr);
}}


function {name}FromXDR(base64Xdr: string): {name} {{
    let scVal = xdr.ScVal.fromXDR(Buffer.from(base64Xdr, 'base64'));
    let obj: [string, any][] = scVal.map().map(e => [e.key().str() as string, e.val()]);
    let map = new Map<string, any>(obj);
    if (!obj) {{
        throw new Error('Invalid XDR');
    }}
    return {{
        {decoded_fields}
    }};
}}
"#
            )
        }

        Entry::Union { name, doc, cases } => {
            let doc = doc_to_ts_doc(doc);
            let arg_name = name.to_lower_camel_case();
            let encoded_cases = js_to_xdr_union_cases(&arg_name, cases);
            let cases = cases.iter().map(case_to_ts).join(" | ");
            let void = type_to_js_xdr(&Type::Void);

            format!(
                r#"{doc}export type {name} = {cases};

function {name}ToXDR({arg_name}?: {name}): xdr.ScVal {{
    if (!{arg_name}) {{
        return {void};
    }}
    let res: xdr.ScVal[] = [];
    switch ({arg_name}.tag) {{
        {encoded_cases}  
    }}
    return xdr.ScVal.scvVec(res);
}}
"#
            )
        }
        Entry::Enum { doc, name, cases } => {
            if name == "Error" {
                let cases = cases
                    .iter()
                    .map(|c| format!("{{message:\"{}\"}}", c.doc))
                    .join(",\n  ");
                return format!(
                    r#"const Errors = [ 
  {cases}
]"#
                );
            }
            let doc = doc_to_ts_doc(doc);
            let cases = cases.iter().map(enum_case_to_ts).join("\n  ");
            let name = (name == "Error")
                .then(|| format!("{name}s"))
                .unwrap_or(name.to_string());
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

fn js_to_xdr_fields(struct_name: &str, f: &[StructField]) -> String {
    f.iter()
        .map(|StructField {  name, value , .. }| {
            format!(
                r#"new xdr.ScMapEntry({{key: ((i)=>{})("{name}"), val: ((i)=>{})({struct_name}.{name})}})"#,
                type_to_js_xdr(&Type::Symbol),
                type_to_js_xdr(value),
            )
        })
        .join(",\n        ")
}

fn js_to_xdr_union_cases(arg_name: &str, f: &[UnionCase]) -> String {
    f.iter()
        .map(|UnionCase { name, values, .. }| {
            let mut rhs = format!(
                "res.push(((i) => {})(\"{name}\"))",
                type_to_js_xdr(&Type::Symbol)
            );
            if !values.is_empty() {
                rhs = format!(
                    "{rhs};\n            res.push(...((i) => {})({arg_name}.values))",
                    type_to_js_xdr(&Type::Tuple {
                        elements: values.clone()
                    })
                );
            };
            format!("case \"{name}\":\n            {rhs};\n            break;")
        })
        .join("\n    ")
}

fn enum_case_to_ts(case: &types::EnumCase) -> String {
    let types::EnumCase { name, value, .. } = case;
    format!("{name} = {value},")
}

fn case_to_ts(case: &types::UnionCase) -> String {
    let types::UnionCase { name, values, .. } = case;
    let mut inner: String = format!("tag: \"{name}\"");
    if !values.is_empty() {
        inner = format!(
            "{inner}, values: {}",
            type_to_ts(&Type::Tuple {
                elements: values.clone(),
            })
        );
    };
    format!("{{{inner}}}")
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
        types::Type::String => "string".to_owned(),
        types::Type::Map { key, value } => {
            format!("Map<{}, {}>", type_to_ts(key), type_to_ts(value))
        }
        types::Type::Option { value } => format!("Option<{}>", type_to_ts(value)),
        types::Type::Result { value, .. } => {
            format!("Result<{}>", type_to_ts(value))
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
        types::Type::BytesN { .. } => "Buffer".to_string(),
        types::Type::Void => "void".to_owned(),
        types::Type::U256 => todo!(),
        types::Type::I256 => todo!(),
        types::Type::Timepoint => todo!(),
        types::Type::Duration => todo!(),
    }
}

fn js_from_xdr_fields(f: &[StructField]) -> String {
    f.iter()
        .map(|StructField { name, value, .. }| {
            format!(
                r#"{name}: scValToJs(map.get("{name}")) as unknown as {}"#,
                type_to_ts(value)
            )
        })
        .join(",\n        ")
}
