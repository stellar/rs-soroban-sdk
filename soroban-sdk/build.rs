pub fn main() {
    // Inform the compiler that the soroban_sdk_internal_no_rssdkver_meta cfg is valid.
    // The cfg is used when building the test vectors in this repository, to disable the embedding
    // of the rssdkver meta to increase the stability of the build wasms and therefore their wasm
    // hash.
    println!("cargo::rustc-check-cfg=cfg(soroban_sdk_internal_no_rssdkver_meta)");

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    if let Ok(version) = rustc_version::version() {
        if version.major == 1 && version.minor >= 82 {
            panic!("Rust compiler 1.82+ with target 'wasm32-unknown-unknown' is unsupported by the Soroban Environment, because the 'wasm32-unknown-unknown' target in Rust 1.82+ has features enabled that are not yet supported and not easily disabled: reference-types, multi-value. Use Rust 1.81 to build for the 'wasm32-unknown-unknown' target.");
        }
    }

    if let Ok(rustc_version) = rustc_version::version() {
        println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");
    }

    // Warn if building for WASM without spec optimization support from the build system (Stellar
    // CLI). The cfg flag indicates the build system supports spec optimization.
    println!("cargo::rustc-check-cfg=cfg(soroban_sdk_build_system_supports_optimising_specs_using_data_markers)");
    let target_family = std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    if target_family == "wasm" {
        let rustflags = std::env::var("RUSTFLAGS").unwrap_or_default();
        let cargo_encoded_rustflags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
        let cfg_name = "soroban_sdk_build_system_supports_optimising_specs_using_data_markers";
        let has_cfg = rustflags.contains(cfg_name) || cargo_encoded_rustflags.contains(cfg_name);
        if !has_cfg {
            println!("cargo:warning=Building without a Soroban-aware build system. The contract will be larger than necessary. Use stellar-cli v26 or newer to build.");
        }
    }

    crate_git_revision::init();
}
