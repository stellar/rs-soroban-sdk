//! A linker shim that shakes a contract's spec as part of linking it.
//!
//! `soroban-lld` runs the real linker, then rewrites the wasm it produced to
//! remove the spec entries for types and events the contract does not use.
//!
//! The SDK cannot do this itself. The spec entries it emits are
//! `#[link_section]` statics, and those are retained unconditionally: they are
//! not subject to dead code elimination, so an unused type's spec reaches the
//! wasm no matter what the compiler can prove about it. Whether a type is used
//! is also not knowable to a macro, because the type may be defined in one
//! crate and used, or not used, in another. It is only known once everything
//! has been linked together, which makes the linker the first point in the
//! build where the spec can be made accurate.
//!
//! Configure it in the contract's `.cargo/config.toml`:
//!
//! ```toml
//! [env]
//! SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2 = "1"
//!
//! [target.wasm32v1-none]
//! linker = "soroban-lld"
//! ```
//!
//! The `[env]` entry tells the SDK that the build system shakes the spec, which
//! it requires before it will build a contract at all. The `linker` entry is
//! what makes that true.

mod shake;
mod wasm;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The linker to run. rustc puts its bundled `rust-lld` on `PATH` when it
/// invokes a linker, so the bare name resolves to the toolchain's own copy.
const DEFAULT_LINKER: &str = "rust-lld";

/// Overrides the linker to run, for a build that does not use `rust-lld`.
const LINKER_ENV: &str = "SOROBAN_LLD_LINKER";

/// Set to report what was shaken. rustc surfaces linker output as a build
/// warning, so nothing is printed unless asked for.
const VERBOSE_ENV: &str = "SOROBAN_LLD_VERBOSE";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let linker = std::env::var(LINKER_ENV).unwrap_or_else(|_| DEFAULT_LINKER.to_string());
    let status = match Command::new(&linker).args(&args).status() {
        Ok(status) => status,
        Err(e) => return fail(format!("running linker `{linker}`: {e}")),
    };
    if !status.success() {
        // The linker already reported why. Pass its exit code through so the
        // build fails the same way it would without the shim.
        return ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1));
    }

    let Some(output) = output_path(&args) else {
        // Nothing identifiable to post-process. The link itself succeeded, so
        // the build should carry on.
        return ExitCode::SUCCESS;
    };
    if output.extension().is_none_or(|e| e != "wasm") {
        return ExitCode::SUCCESS;
    }

    match shake_file(&output) {
        Ok(Some(shake::Outcome::Shaken { before, after })) => {
            if std::env::var_os(VERBOSE_ENV).is_some() {
                eprintln!(
                    "soroban-lld: shook {} spec {} out of {}",
                    before - after,
                    if before - after == 1 {
                        "entry"
                    } else {
                        "entries"
                    },
                    output.display(),
                );
            }
            ExitCode::SUCCESS
        }
        Ok(_) => ExitCode::SUCCESS,
        // The wasm linked, so it is well formed, and the contract meta asked
        // for the spec to be shaken. Failing here rather than passing the wasm
        // through means a contract is never published with a spec that claims
        // types it does not have, which is the whole reason this shim exists.
        Err(e) => fail(format!("shaking {}: {e}", output.display())),
    }
}

fn shake_file(path: &Path) -> Result<Option<shake::Outcome>, Box<dyn std::error::Error>> {
    let wasm = std::fs::read(path)?;
    let Some((shaken, outcome)) = shake::shake(&wasm)? else {
        return Ok(None);
    };
    if matches!(outcome, shake::Outcome::Shaken { .. }) {
        // Remove before writing so that the new contents do not propagate
        // through the hard link cargo leaves between `deps/` and the profile
        // directory. See stellar/stellar-cli#1694.
        std::fs::remove_file(path)?;
        std::fs::write(path, shaken)?;
    }
    Ok(Some(outcome))
}

/// Returns the path the linker was told to write, in any of the spellings lld
/// accepts for it.
fn output_path(args: &[String]) -> Option<PathBuf> {
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let path = match arg.as_str() {
            "-o" | "--output" => args.next()?.as_str(),
            a if a.starts_with("--output=") => a.trim_start_matches("--output="),
            a if a.starts_with("-o") && a.len() > 2 => a.trim_start_matches("-o"),
            _ => continue,
        };
        return Some(PathBuf::from(path));
    }
    None
}

fn fail(message: String) -> ExitCode {
    eprintln!("soroban-lld: error: {message}");
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::output_path;
    use std::path::PathBuf;

    #[test]
    fn output_path_spellings() {
        let p = |s: &str| Some(PathBuf::from(s));
        let args = |s: &str| {
            s.split(' ')
                .map(ToString::to_string)
                .collect::<Vec<String>>()
        };
        assert_eq!(
            output_path(&args("-flavor wasm -o out.wasm")),
            p("out.wasm")
        );
        assert_eq!(output_path(&args("-oout.wasm")), p("out.wasm"));
        assert_eq!(output_path(&args("--output out.wasm")), p("out.wasm"));
        assert_eq!(output_path(&args("--output=out.wasm")), p("out.wasm"));
        // The first spelling wins, matching lld, and a flag that merely starts
        // the same way is not mistaken for it.
        assert_eq!(output_path(&args("-o a.wasm -o b.wasm")), p("a.wasm"));
        assert_eq!(output_path(&args("--no-entry --gc-sections")), None);
    }
}
