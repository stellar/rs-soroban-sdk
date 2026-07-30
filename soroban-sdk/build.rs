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

    // On a wasm target, check for an env var from the build system (Stellar CLI) that indicates it
    // supports spec optimization using markers. Spec shaking is always on, and the contract's spec
    // is only correct once the build system has shaken it, so a build system that does not do so
    // is an error.
    let env_name = "SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2";
    println!("cargo::rerun-if-env-changed={env_name}");
    if std::env::var(env_name).is_err()
        && std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm"
    {
        eprintln!(
            "\
\nerror: soroban-sdk requires stellar-cli v25.2.0+ to build a contract\
\n\
\nThe soroban-sdk embeds spec shaking markers that the build system must use\
\nto shake the contract's spec, so contracts must be built with\
\n`stellar contract build` from stellar-cli v25.2.0 or newer.\
\n\
\nTo fix, either:\
\n  - Build with `stellar contract build` using stellar-cli v25.2.0+\
\n  - If you are using another build system that shakes the spec, set the\
\n    SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2 env var to signal it.\
"
        );
        std::process::exit(1);
    }

    crate_git_revision::init();
}
