#[test]
fn fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/macros_fails/*.rs");
}
