
#![allow(unused_variables)]
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if item == &b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..];
    println!("word1: {}, word2: {}", hello, world);
}

#[test]
fn first_word_test() {
    assert_eq!(first_word(&String::from("foo bar")), 3);
    assert_eq!(first_word(&String::from("bar")), 3);
    assert_eq!(first_word(&String::from("")), 0);
}

#[test]
fn string_slice_test() {
    string_slice();
}