fn mutable_var() -> i32 {
    let mut x = 5;
    x = 20;
    return x;
}

#[test]
fn immutable_test() {
    assert_eq!(mutable_var(), 20)
}
