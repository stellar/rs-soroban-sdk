use stellar_xdr::ScSpecTypeDef;

pub fn generate_type_ident(spec: &ScSpecTypeDef) -> String {
    match spec {
        // TODO: Better number handling types
        ScSpecTypeDef::U64 | ScSpecTypeDef::I64 | ScSpecTypeDef::U128 | ScSpecTypeDef::I128 => {
            "bigint".to_string()
        }
        ScSpecTypeDef::U32 | ScSpecTypeDef::I32 => "number | bigint".to_string(),
        ScSpecTypeDef::Bool => "boolean".to_string(),
        ScSpecTypeDef::Symbol => "string".to_string(),
        ScSpecTypeDef::Bytes => "Buffer | Uint8Array".to_string(),
        ScSpecTypeDef::Address => "string | SorobanClient.Address".to_string(),
        ScSpecTypeDef::Option(o) => format!("null | {}", generate_type_ident(&o.value_type)),
        ScSpecTypeDef::Result(_r) => todo!("generate_type_ident(Result)"),
        ScSpecTypeDef::Vec(v) => {
            let element_ident = generate_type_ident(&v.element_type);
            format!("Array<{element_ident}>")
        }
        ScSpecTypeDef::Map(m) => {
            let key_ident = generate_type_ident(&m.key_type);
            let value_ident = generate_type_ident(&m.value_type);
            format!("Record<{key_ident}, {value_ident}>")
        }
        ScSpecTypeDef::Set(s) => {
            let element_ident = generate_type_ident(&s.element_type);
            // TODO: Is there a better type than array here?
            format!("Array<{element_ident}>")
        }
        ScSpecTypeDef::Tuple(t) => {
            let type_idents = t
                .value_types
                .iter()
                .map(generate_type_ident)
                .collect::<Vec<String>>()
                .join(", ");
            format!("[{}]", type_idents)
        }
        ScSpecTypeDef::BytesN(_b) => "Buffer | Uint8Array".to_string(),
        ScSpecTypeDef::Udt(_u) => todo!("generate_type_ident(Udt)"),
        // TODO: Figure these out
        ScSpecTypeDef::Val | ScSpecTypeDef::Bitset | ScSpecTypeDef::Status => "unknown".to_string(),
    }
}

pub fn generate_type_parser(spec: &ScSpecTypeDef, name: &str) -> String {
    match spec {
        // TODO: Better number handling types
        ScSpecTypeDef::U64 => {
            format!("xdr.ScVal.scvObject(xdr.ScObject.scoU64({name}))").to_string()
        }
        ScSpecTypeDef::I64 => {
            format!("xdr.ScVal.scvObject(xdr.ScObject.scoI64({name}))").to_string()
        }
        // TODO: Implement U128 and I128 conversion somewhere properly
        ScSpecTypeDef::U128 => format!("bigintToU128({name})").to_string(),
        ScSpecTypeDef::I128 => format!("bigintToI128({name})").to_string(),
        ScSpecTypeDef::U32 => format!("xdr.ScVal.scvU32({name})").to_string(),
        ScSpecTypeDef::I32 => format!("xdr.ScVal.scvI32({name})").to_string(),
        ScSpecTypeDef::Bool => {
            format!("xdr.ScVal.scvStatic({name} ? xdr.ScStatic.scsTrue() : xdr.ScStatic.scsFalse())")
                .to_string()
        }
        ScSpecTypeDef::Symbol => format!("xdr.ScVal.scvSymbol({name})").to_string(),
        ScSpecTypeDef::Bytes | ScSpecTypeDef::BytesN(_) => {
            format!("xdr.ScVal.scvObject(xdr.ScObject.scoBytes(Buffer.from({name})))").to_string()
        }
        ScSpecTypeDef::Address => format!(
            "(typeof {name} === \"string\" ? new SorobanClient.Address({name}) : {name}).toScVal()"
        )
        .to_string(),
        ScSpecTypeDef::Option(o) => format!(
            "{name} === null ? xdr.ScVal.scvStatic(xdr.ScStatic.scsVoid()) : {}",
            generate_type_parser(&o.value_type, name)
        )
        .to_string(),
        ScSpecTypeDef::Result(_r) => todo!("generate_type_parser(Result)"),
        ScSpecTypeDef::Vec(v) => format!(
            "xdr.ScVal.scvObject(xdr.ScObject.scoVec({name}.map(elem => {})))",
            generate_type_parser(&v.element_type, "elem")
        ),
        ScSpecTypeDef::Map(_m) => todo!("generate_type_parser(Map)"),
        ScSpecTypeDef::Set(_s) => todo!("generate_type_parser(Set)"),
        ScSpecTypeDef::Tuple(_t) => todo!("generate_type_parser(Tuple)"),
        ScSpecTypeDef::Udt(_u) => todo!("generate_type_parser(Udt)"),
        ScSpecTypeDef::Val => todo!(),
        ScSpecTypeDef::Bitset => todo!(),
        ScSpecTypeDef::Status => todo!(),
    }
}
