fn control_for() {
    let a = [0, 1, 2, 3, 4, 5];

    for elem in a.iter() {
        println!("for: the value is {}", elem);
    }
}

fn control_loop() {
    let mut i = 0;
    loop {
        println!("loop: the value is {}", i);
        if i == 5 {
            break;
        }
        i += 1;
    }
}

fn control_while() {
    let mut i = 0;
    while i <= 5 {
        println!("while: the value is {}", i);
        i += 1;
    }
}

#[test]
fn control_test() {
    // run only
    control_for();
    control_loop();
    control_while();
}
