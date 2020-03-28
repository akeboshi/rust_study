fn mutable_var() -> i32 {
    let mut x = 5;
    x = 20;
    return x;
}

const MAX_POINTS: u32 = 100_000;

#[test]
fn mutable_test() {
    assert_eq!(mutable_var(), 20);
}

#[test]
fn immutable_test() {
    let x = 3;
    assert_eq!(x, 3);
    let x = "foo";
    assert_eq!(x, "foo");
}

#[test]
fn const_test() {
    assert_eq!(100_000, MAX_POINTS);
}
