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

    println!("cargo::rerun-if-env-changed=SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2");
    println!("cargo::rerun-if-env-changed=SOROBAN_SDK_ALLOW_BUILD_WITHOUT_STELLAR_CLI");

    // When building for wasm and the build system (e.g. stellar-cli) does not support spec shaking
    // v2, we check some situations which will compile error to prevent developers from building
    // contracts that are broken.
    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm"
        && std::env::var("SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2").is_err()
    {
        // Check if spec shaking v2 is enabled, if it is, the build needs to be run with a version
        // of stellar-cli that supports it.
        if std::env::var("CARGO_FEATURE_EXPERIMENTAL_SPEC_SHAKING_V2").is_ok() {
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

        // Building contracts that use the soroban-sdk will soon require the stellar-cli and a
        // version that supports spec shaking v2. An env var
        // escape hatch is provided to temporarily disable the error, and will be removed in a future
        // version of soroban-sdk.
        if std::env::var("SOROBAN_SDK_ALLOW_BUILD_WITHOUT_STELLAR_CLI").is_err() {
            eprintln!(
                "\
\nerror: building contracts that use the soroban-sdk will soon require a recent version of the stellar-cli\
\n\
\nBuild with `stellar contract build` from stellar-cli v25.2.0 or newer.\
\n\
\nTo temporarily allow building without stellar-cli, set the env var:\
\n  SOROBAN_SDK_ALLOW_BUILD_WITHOUT_STELLAR_CLI=1\
\nThis escape hatch will be removed in a near future version of soroban-sdk.\
"
            );
            std::process::exit(1);
        }
    }

    crate_git_revision::init();
}
