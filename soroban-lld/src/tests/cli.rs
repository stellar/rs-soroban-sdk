//! Tests for the linker command line handling.

use crate::output_path;
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
