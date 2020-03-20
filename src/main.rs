fn main() {
    let x = return5();
    println!("{}", x);
    println!("Hello, world!");
}

fn return5() -> i64{
    return 5;
}

#[test]
fn return5_test() {
    assert_eq!(4, return5())
}
