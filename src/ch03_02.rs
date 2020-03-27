fn tuple() -> (i32, f64, u8) {
    return (500, 6.4, 1);
}

fn array() -> [i32; 6] {
    return [1, 2, 3, 5, 8, 13];
}

#[test]
fn tuple_test() {
    assert_eq!((500, 6.4, 1), tuple());

    let (x, _y, z) = tuple();
    assert_eq!(x, 500);
    assert_eq!(z, 1)
}

#[test]
fn array_test() {
    use std::panic;
    assert_eq!(5, array()[3]);
    let index = 6;
    let result = panic::catch_unwind(|| {
        array()[index];
    });
    assert!(result.is_err());
}