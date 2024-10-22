pub fn main() {
    if let Ok(version) = rustc_version::version() {
        if version.major == 1 && version.minor >= 82 {
            panic!("Rust compiler 1.82+ is unsupported by the Soroban Environment, because Wasm target features are enabled that are not yet supported and not easily disabled: reference-types, multi-value. Use Rust 1.81.");
        }
    }
}
