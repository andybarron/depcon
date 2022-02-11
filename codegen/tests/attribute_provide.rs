use trybuild::TestCases;

#[test]
fn test_valid_attributes_pass() {
    let t = TestCases::new();
    t.pass("tests/attribute_provide/pass/*.rs");
}

#[test]
fn test_invalid_attributes_fail() {
    let t = TestCases::new();
    t.compile_fail("tests/attribute_provide/fail/*.rs");
}
