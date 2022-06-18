use stellar_xdr::SpecTypeDef;

// TODO: Remove user-defined types from SpecTypeDef and treat separately.

pub fn type_def_from_str(s: impl AsRef<str>) -> SpecTypeDef {
    let s: &str = s.as_ref();
    match s {
        "i32" => SpecTypeDef::I32,
        "i64" => SpecTypeDef::I64,
        "Symbol" => SpecTypeDef::Symbol,
        _ => SpecTypeDef::Unit, // TODO: Treat as a opaque named user-defined type.
    }
}
