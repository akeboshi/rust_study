use std::panic;

fn return7() -> i32 {
    7
}

fn ver2_diverges() -> ! {
    panic!("panic だよ");
}

#[test]
fn immutable_test() {
    let f: fn() -> i32 = return7;
    assert_eq!(f(), 7)
}

#[test]
fn diverges_test() {
    let result = panic::catch_unwind(|| {
        ver2_diverges();
    });
    assert!(result.is_err());
}
