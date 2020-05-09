/*
  整数のリストが与えられ、
  ベクタを使ってmean(平均値)、
  median(ソートされた時に真ん中に来る値)、 
  mode(最も頻繁に出現する値; 
  ハッシュマップがここでは有効活用できるでしょう)
  を返してください。
*/

use std::collections::HashMap;

fn mean(l: &[i32]) -> i32 {
    let mut a: i32 = 0;
    for i in l {
        a += i;
    }

    return a/( l.len() as i32);
}
/*
fn mean_t<T: std::ops::AddAssign + std::ops::Div>(l: &[T]) -> T {
    let mut a: T;
    for i in l {
        a += *i;
    }

    return a / l.len();
}*/

fn medium(l: &[i32]) -> i32 {
    let mid = l.len() / 2;
    return l[mid];
}

fn mode(l:&[i32]) -> i32 {
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
    println!("{}", mean(&[1,2,3]));
    assert_eq!(mean(&[1,2,3]), 2);
    assert_eq!(medium(&[1,2,3,4,5]), 3);
    assert_eq!(mode(&[1,2,3,2,3,3,4,5,6,8]), 3);
}