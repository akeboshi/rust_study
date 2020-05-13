/*
  整数のリストが与えられ、
  ベクタを使ってmean(平均値)、
  median(ソートされた時に真ん中に来る値)、 
  mode(最も頻繁に出現する値; 
  ハッシュマップがここでは有効活用できるでしょう)
  を返してください。
*/

use std::collections::HashMap;

fn mean<T: std::ops::AddAssign + std::ops::Div + num::ToPrimitive>(l: &[T]) -> f64 {
    let mut a: f64 = 0.0;
    for i in l {
        a += i.to_f64().unwrap();
    }

    return a / l.len() as f64;
}

fn mediun(l: &Vec<i32>) -> i32 {
    let mut clone = l.clone();
    clone.sort();
    return clone[clone.len() / 2];
}

fn mode(l: &Vec<i32>) -> i32 {
    let mut num_count = HashMap::new();
    for i in l {
        let count = num_count.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    for (num, cnt) in num_count {
        if cnt > max_count {
            max_count = cnt;
            mode = *num;
        }
    }

    return mode;
}

#[test]
fn test_mean() {
    assert_eq!(mean(&vec![1,2,3]), 2.0);
    assert_eq!(mediun(&vec![1,4,3,5,5,5,2]), 4);
    assert_eq!(mode(&vec![1,2,3,2,3,3,4,5,6,8]), 3);
}