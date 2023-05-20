use crate::gen::json::types;

pub fn type_to_js_xdr(value: &types::Type) -> String {
    match value {
        types::Type::Val => todo!(),
        types::Type::U64 => "xdr.ScVal.scvU64(xdr.Uint64.fromString(i))".to_string(),
        types::Type::I64 => "xdr.ScVal.scvI64(xdr.Int64.fromString(i))".to_string(),
        types::Type::U32 => "xdr.ScVal.scvU32(i)".to_string(),
        types::Type::I32 => "xdr.ScVal.scvI32(i)".to_string(),
        types::Type::Bool => "xdr.ScVal.scvBool(i)".to_string(),
        types::Type::Symbol => "xdr.ScVal.scvSymbol(i)".to_string(),
        types::Type::Map { key, value } => format!(
            "xdr.ScVal.scvMap(Array.from(i.entries()).map(([key, value]) => {{
            return new xdr.ScMapEntry({{
                key: ((i)=>{})(key),
                val: ((i)=>{})(value)}})
          }}))",
            type_to_js_xdr(key),
            type_to_js_xdr(value)
        ),
        types::Type::Option { value } => format!(
            "(!i) ? {} : {}",
            type_to_js_xdr(&types::Type::Void),
            type_to_js_xdr(value)
        ),
        types::Type::Result { value, .. } => type_to_js_xdr(value),
        types::Type::Set { .. } => todo!(),
        types::Type::Vec { element } => format!("[...i].map({})", type_to_js_xdr(element)),
        types::Type::Tuple { .. } => {
            todo!()
            // elements.iter().map(|e| format!("[...i].map({})", type_to_js_xdr(element))
        }

        types::Type::Custom { name } => format!("{name}ToXDR(i)"),
        types::Type::Status => todo!(),
        types::Type::BytesN { .. } => "xdr.ScVal.scvBytes(i)".to_owned(),
        types::Type::Bytes => "xdr.ScVal.scvBytes(i)".to_owned(),
        types::Type::Address => "SorobanClient.Address.fromString(i).toScVal()".to_owned(),
        types::Type::Void => "xdr.ScVal.scvVoid()".to_owned(),
        types::Type::U128 => {
            "xdr.ScVal.scvU128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex'))".to_owned()
        }
        types::Type::I128 => {
            "xdr.ScVal.scvI128(xdr.Int128Parts.fromXDR(i.toString(16), 'hex'))".to_owned()
        }
        types::Type::I256 => todo!(),
        types::Type::U256 => todo!(),
        types::Type::String => "xdr.ScVal.scvString(i)".to_owned(),
        types::Type::Timepoint => todo!(),
        types::Type::Duration => todo!(),
    }
}
