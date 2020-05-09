#[derive(Debug)]
struct Parent(usize, Child, Child);

use std::ops::Drop; // このトレイトを実装するとDropをオーバライドできる

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let p1 = Parent(1, Child(11), Child(12));

    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("(a)  p1: {:?}, p2: {:?}", p1, p2);  // (a)の時点
    }

    println!("(b)  p1: {:?}", &p1);                   // (b)の時点
    let p3 = Parent(3, Child(31), Child(32));
    println!("(c)  p1: {:?}, p3: {:?}", p1, p3);      // (c)の時点
}