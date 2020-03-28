fn convet_fahrenheit_to_centigrade(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        return 0
    }
    let mut ret = 1;
    let mut prev = 0;
    for _ in 1..num {
        let tmp = ret;
        ret += prev;
        prev = tmp;
    }
    return ret;
}

#[test]
fn convet_test() {
    assert_eq!(convet_fahrenheit_to_centigrade(32.0), 0.0);
    assert!((convet_fahrenheit_to_centigrade(-20.0) - (-28.8889)).abs() < 0.01);
    assert!((convet_fahrenheit_to_centigrade(1000.0) - 537.778).abs() < 0.01);
}

#[test]
fn fibonacci_test() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(10), 55);
}