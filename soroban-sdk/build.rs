pub fn main() {
    // Inform the compiler that the soroban_sdk_internal_no_rssdkver_meta cfg is valid.
    // The cfg is used when building the test vectors in this repository, to disable the embedding
    // of the rssdkver meta to increase the stability of the build wasms and therefore their wasm
    // hash.
    println!("cargo::rustc-check-cfg=cfg(soroban_sdk_internal_no_rssdkver_meta)");

    // Check if we're building for wasm32-unknown-unknown target (cross-compilation safe)
    if std::env::var("CARGO_CFG_TARGET_FAMILY").as_deref() == Ok("wasm")
        && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("unknown")
    {
        if let Ok(version) = rustc_version::version() {
            if version.major == 1 && version.minor >= 82 {
                panic!("Rust compiler 1.82+ with target 'wasm32-unknown-unknown' is unsupported by the Soroban Environment, use 'wasm32v1-none' available with Rust 1.84+. The 'wasm32-unknown-unknown' target in Rust 1.82+ has features enabled that are not yet supported and not easily disabled: reference-types, multi-value. If you must build for the 'wasm32-unknown-unknown' use Rust 1.81 or earlier.");
            }
        }
    }

    if let Ok(rustc_version) = rustc_version::version() {
        println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");
    }

    // When the experimental_spec_shaking_v2 feature is enabled, check for an env var from the
    // build system (like Stellar CLI) that indicates it supports spec optimization using markers.
    // If the env var is set, enable spec_shaking_v2 cfg for the crate. If not, fall back to
    // spec shaking v1 behavior and emit a warning on wasm targets.
    println!("cargo::rustc-check-cfg=cfg(spec_shaking_v2)");
    if std::env::var("CARGO_FEATURE_EXPERIMENTAL_SPEC_SHAKING_V2").is_ok() {
        let env_name = "SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2";
        println!("cargo::rerun-if-env-changed={env_name}");
        if std::env::var(env_name).is_ok() {
            println!("cargo::rustc-cfg=spec_shaking_v2");
        } else if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm" {
            println!(
                "cargo::warning=soroban-sdk: feature 'experimental_spec_shaking_v2' was enabled but not used, \
                 because this build was not started by a tool that supports spec shaking v2. \
                 Falling back to spec shaking v1. To use v2, use a build tool that supports \
                 spec shaking v2 such as stellar-cli v25.2.0+ with `stellar contract build`. \
                 To manually use v2 without a supporting build tool, set the env var \
                 SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2 before building."
            );
        }
    }

    crate_git_revision::init();
}
