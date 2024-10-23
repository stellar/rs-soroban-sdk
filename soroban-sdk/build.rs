pub fn main() {
    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    if let Ok(version) = rustc_version::version() {
        if version.major == 1 && version.minor >= 82 {
            panic!("Rust compiler 1.82+ with target 'wasm32-unknown-unknown' is unsupported by the Soroban Environment, because the 'wasm32-unknown-unknown' target in Rust 1.82+ has features enabled that are not yet supported and not easily disabled: reference-types, multi-value. Use Rust 1.81 to build for the 'wasm32-unknown-unknown' target.");
        }
    }
}
