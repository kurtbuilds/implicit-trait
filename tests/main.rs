
#[test]
fn run_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic.rs");
}