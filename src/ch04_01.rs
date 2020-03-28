fn move_string() -> String {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world", s1); value borrowed here after move
    // move したあとはs1は使えない
    return s2;
}

fn move_clone() -> (String, String) {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str("clone");

    return (s1, s2);
}

fn copy_primitive_type(x: i32) -> (i32, i32) {
    let y = x;
    return (x, y);
}

fn function_ownership() {
    let s = String::from("hello");

    takes_ownership(s);
    // println!("{}", s); 上の関数に引数としてmoveしてしまってるので、使えない

    let x = 5;
    makes_copy(x);
    println!("{}", x); // Copy trait なので、使える
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

#[test]
fn move_string_test() {
    assert_eq!("hello", move_string());
    assert_eq!((String::from("hello"), String::from("helloclone")), move_clone());
}

#[test]
fn copy_primitive_type_test() {
    assert_eq!((3, 3), copy_primitive_type(3));
}

#[test]
fn function_ownership_test() {
    // run only
    function_ownership();
}