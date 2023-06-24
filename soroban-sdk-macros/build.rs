pub fn main() {
    if let Ok(rustc_version) = rustc_version::version() {
        println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");
    }

    crate_git_revision::init();
}
