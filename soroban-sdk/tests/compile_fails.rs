#[test]
fn compile_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fails/contracttrait_cfg_errors.rs");
    t.compile_fail("tests/compile_fails/contracttrait_without_trait.rs");
    t.compile_fail("tests/compile_fails/contracttype_lib_removed.rs");
    t.compile_fail("tests/compile_fails/export_arg_errors.rs");
}
