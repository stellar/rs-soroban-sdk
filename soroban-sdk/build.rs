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

    // The env var set by the build system (Stellar CLI) that indicates it supports spec
    // optimization using markers.
    let build_system_env_name = "SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2";
    println!("cargo::rerun-if-env-changed={build_system_env_name}");
    let build_system_supports_spec_shaking_v2 = std::env::var(build_system_env_name).is_ok();
    let target_family_is_wasm =
        std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm";

    if std::env::var("CARGO_FEATURE_EXPERIMENTAL_SPEC_SHAKING_V2").is_ok() {
        // When the experimental_spec_shaking_v2 feature is enabled on a wasm target, the build
        // system must support spec optimization using markers, otherwise the build cannot produce
        // a correct spec.
        if !build_system_supports_spec_shaking_v2 && target_family_is_wasm {
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
    } else if !build_system_supports_spec_shaking_v2 && target_family_is_wasm {
        // When the experimental_spec_shaking_v2 feature is not enabled, building a wasm without a
        // build system that supports spec optimization using markers still works today, but a
        // future SDK version will enable spec shaking v2 by default which requires a supported
        // build tool. Warn so users can migrate ahead of time.
        println!(
            "cargo::warning=\
soroban-sdk: building for wasm without a build tool that supports spec shaking v2. \
A future SDK version will enable spec shaking v2 by default, which requires building with a \
supported build tool. Migrate to `stellar contract build` from stellar-cli v25.2.0 or newer to \
ensure your contracts keep building."
        );
    }

    crate_git_revision::init();
}
