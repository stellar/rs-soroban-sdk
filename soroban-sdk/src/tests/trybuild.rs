#[test]
fn fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/tests/trybuild/*.rs");
}
