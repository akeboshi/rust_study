
pub mod r1;
pub mod r4_2;
pub mod ch03_01;
pub mod ch03_02;
pub mod ch03_05;
pub mod ch03_traning;
pub mod ch04_01;
pub mod ch04_03;
mod ch08_01;
mod ch08_02;
mod ch08_03;

pub fn return5() -> i64{
    return 5;
}

#[test]
fn return5_test() {
    assert_eq!(5, return5())
}
