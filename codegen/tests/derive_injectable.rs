use trybuild::TestCases;

#[test]
fn test_valid_derives_pass() {
    let t = TestCases::new();
    t.pass("tests/derive_injectable/pass/*.rs");
}

#[test]
fn test_invalid_derives_fail() {
    let t = TestCases::new();
    t.compile_fail("tests/derive_injectable/fail/*.rs");
}
