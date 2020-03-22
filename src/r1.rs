
pub fn return2() -> i64{
    return 2;
}

#[test]
fn return2_test() {
    assert_eq!(2, return2())
}
