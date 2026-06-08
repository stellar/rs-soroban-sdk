pub fn main() {
    // Inform the compiler that the soroban_sdk_internal_no_rssdkver_meta cfg is valid.
    // The cfg is used when building the test vectors in this repository, to disable the embedding
    // of the rssdkver meta to increase the stability of the build wasms and therefore their wasm
    // hash.
    println!("cargo::rustc-check-cfg=cfg(soroban_sdk_internal_no_rssdkver_meta)");

    // Check if we're building for wasm32-unknown-unknown target (cross-compilation safe)
    println!("cargo::rerun-if-env-changed=SOROBAN_SDK_ALLOW_WASM32_UNKNOWN_UNKNOWN");
    if std::env::var("CARGO_CFG_TARGET_FAMILY").as_deref() == Ok("wasm")
        && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("unknown")
    {
        if let Ok(version) = rustc_version::version() {
            if version.major == 1 && version.minor >= 82 {
                if std::env::var("SOROBAN_SDK_ALLOW_WASM32_UNKNOWN_UNKNOWN").is_ok() {
                    println!("cargo:warning=Building for 'wasm32-unknown-unknown' with Rust 1.82+ via SOROBAN_SDK_ALLOW_WASM32_UNKNOWN_UNKNOWN. The produced wasm uses features (reference-types, multi-value) that are not supported by the Soroban Environment and MUST NOT be deployed on-chain. This is only suitable for purposes such as static analysis. To produce deployable contracts, use the 'wasm32v1-none' target available with Rust 1.84+.");
                } else {
                    panic!("Rust compiler 1.82+ with target 'wasm32-unknown-unknown' is unsupported by the Soroban Environment, use 'wasm32v1-none' available with Rust 1.84+. The 'wasm32-unknown-unknown' target in Rust 1.82+ has features enabled that are not yet supported and not easily disabled: reference-types, multi-value. If you must build for the 'wasm32-unknown-unknown' use Rust 1.81 or earlier. If you only need to compile (not deploy) the contract, e.g. for static analysis, set the SOROBAN_SDK_ALLOW_WASM32_UNKNOWN_UNKNOWN environment variable to bypass this check.");
                }
            }
        }
    }

    if let Ok(rustc_version) = rustc_version::version() {
        println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");
    }

    // When the experimental_spec_shaking_v2 feature is enabled on a wasm target, check for an
    // env var from the build system (Stellar CLI) that indicates it supports spec optimization
    // using markers.
    if std::env::var("CARGO_FEATURE_EXPERIMENTAL_SPEC_SHAKING_V2").is_ok() {
        let env_name = "SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2";
        println!("cargo::rerun-if-env-changed={env_name}");
        if std::env::var(env_name).is_err()
            && std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm"
        {
            eprintln!(
                "\
\nerror: soroban-sdk feature 'experimental_spec_shaking_v2' requires stellar-cli v25.2.0+\
\n\
\nThe soroban-sdk 'experimental_spec_shaking_v2' feature requires building\
\nwith `stellar contract build` from stellar-cli v25.2.0 or newer.\
\n\
\nTo fix, either:\
\n  - Build with `stellar contract build` using stellar-cli v25.2.0+\
\n  - Disable the feature by removing 'experimental_spec_shaking_v2' from\
\n    the soroban-sdk import features list in Cargo.toml.\
"
            );
            std::process::exit(1);
        }
    }

    crate_git_revision::init();
}
