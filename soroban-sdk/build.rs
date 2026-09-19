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
    println!("cargo::rerun-if-env-changed={SHAKING_ENV}");
    println!("cargo::rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
    if let Some(key) = linker_env_key() {
        println!("cargo::rerun-if-env-changed={key}");
    }
    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default() == "wasm"
        && !build_system_shakes_spec()
    {
        eprintln!(
            "\
\nerror: soroban-sdk requires a build system that shakes the contract spec\
\n\
\nThe spec soroban-sdk emits names every type and event in the build, including\
\nthose the contract does not use. Which ones those are is only known once\
\neverything has been linked, so the build system has to remove them.\
\n\
\nTo fix, either build with `stellar contract build` using stellar-cli v25.2.0+,\
\nor link with soroban-lld. To use soroban-lld, install it with\
\n`cargo install soroban-lld` and add to .cargo/config.toml:\
\n\
\n    [target.wasm32v1-none]\
\n    rustflags = [\"-Clinker=soroban-lld\"]\
"
        );
        std::process::exit(1);
    }

    crate_git_revision::init();
}

/// The env var a build system sets to declare that it shakes the contract spec
/// itself, after cargo has run. The Stellar CLI sets it.
const SHAKING_ENV: &str = "SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2";

/// Returns whether the spec this build produces will be shaken.
///
/// Either the build system says so, or soroban-lld is configured as the linker,
/// which shakes the spec as part of linking. The latter is the same assertion
/// made by the configuration that carries it out, rather than by a promise set
/// alongside it, so it cannot be set without being true.
fn build_system_shakes_spec() -> bool {
    std::env::var_os(SHAKING_ENV).is_some() || linker_is_soroban_lld()
}

/// Returns the `CARGO_TARGET_<TRIPLE>_LINKER` env var name for this target.
fn linker_env_key() -> Option<String> {
    let target = std::env::var("TARGET").ok()?;
    Some(format!(
        "CARGO_TARGET_{}_LINKER",
        target.to_uppercase().replace('-', "_")
    ))
}

/// Returns whether soroban-lld is configured as the linker for this target.
///
/// Cargo reports `[target.<triple>] rustflags` to build scripts in
/// `CARGO_ENCODED_RUSTFLAGS`, and the env var form of `[target.<triple>] linker`
/// in the process environment. A `linker` key set in a config file is not
/// visible here, which is why the documented setup uses rustflags.
fn linker_is_soroban_lld() -> bool {
    if let Ok(flags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
        let mut flags = flags.split('\x1f');
        while let Some(flag) = flags.next() {
            let linker = if flag == "-C" || flag == "--codegen" {
                flags.next().and_then(|f| f.strip_prefix("linker="))
            } else {
                flag.strip_prefix("-Clinker=")
                    .or_else(|| flag.strip_prefix("--codegen=linker="))
            };
            if linker.is_some_and(is_soroban_lld) {
                return true;
            }
        }
    }
    linker_env_key()
        .and_then(|key| std::env::var(key).ok())
        .is_some_and(|path| is_soroban_lld(&path))
}

/// Returns whether `path` names the soroban-lld binary.
fn is_soroban_lld(path: &str) -> bool {
    std::path::Path::new(path)
        .file_stem()
        .is_some_and(|stem| stem == "soroban-lld")
}
