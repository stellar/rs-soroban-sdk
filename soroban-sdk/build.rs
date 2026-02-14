pub fn main() {
    // Inform the compiler that the soroban_sdk_internal_no_rssdkver_meta cfg is valid.
    // The cfg is used when building the test vectors in this repository, to disable the embedding
    // of the rssdkver meta to increase the stability of the build wasms and therefore their wasm
    // hash.
    println!("cargo::rustc-check-cfg=cfg(soroban_sdk_internal_no_rssdkver_meta)");

    // Probe whether overflow checks are enabled by attempting an overflowing
    // operation at build-script run time. If the addition panics, overflow
    // checks are on; if it wraps silently, they are off.
    //
    // This probe runs in the build script, which is compiled for the host, not
    // the target. It reliably detects overflow-checks being disabled via:
    //   - [profile.*] sections (e.g. [profile.release]).
    //   - Per-package profile overrides ([profile.release.package.soroban-sdk]).
    //   - RUSTFLAGS=-Coverflow-checks=no.
    //
    // Limitations: It may not detect overflow-checks being disabled in cross-compilation setups
    // where the host and target profiles diverge or where the build-override profile is explicitly
    // set to overflow-checks = true when the main profile is set to false. These are advanced
    // configurations, and the check is primarily present to prevent developers using the default
    // release profile that has checks off.
    //
    // This build.rs check can be removed once cfg(overflow_checks) is stabilized.
    // Ref: https://github.com/rust-lang/rust/issues/111466
    println!("cargo::rustc-check-cfg=cfg(soroban_sdk_internal_overflow_checks_enabled)");
    let overflow_checks_enabled = std::panic::catch_unwind(|| {
        let x = std::hint::black_box(255u8);
        let _ = x + 1;
    })
    .is_err();
    if overflow_checks_enabled {
        println!("cargo::rustc-cfg=soroban_sdk_internal_overflow_checks_enabled");
    }

    #[cfg(all(target_family = "wasm", target_os = "unknown"))]
    if let Ok(version) = rustc_version::version() {
        if version.major == 1 && version.minor >= 82 {
            panic!("Rust compiler 1.82+ with target 'wasm32-unknown-unknown' is unsupported by the Soroban Environment, because the 'wasm32-unknown-unknown' target in Rust 1.82+ has features enabled that are not yet supported and not easily disabled: reference-types, multi-value. Use Rust 1.81 to build for the 'wasm32-unknown-unknown' target.");
        }
    }

    if let Ok(rustc_version) = rustc_version::version() {
        println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");
    }

    crate_git_revision::init();
}
