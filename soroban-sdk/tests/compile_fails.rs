#[test]
fn compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fails/contracttrait_cfg_errors.rs");
}
