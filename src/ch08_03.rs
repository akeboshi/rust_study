/*
  ハッシュマップとベクタを使用して、
  ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
   例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。 
   それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。
*/
use std::collections::HashMap;
use regex::Regex;

fn yyy(text: &str) -> HashMap<String, String> {
    
    // 最初にマッチした箇所全体を取り出す例
    let re = Regex::new(r"Add (.+) to (.+)").unwrap();
    let caps = re.captures(text).unwrap();
    
    return xxx(caps.at(1).unwrap(), caps.at(2).unwrap())

}

fn xxx(dep: &str, man: &str) -> HashMap<String, String> {
    let mut hash = HashMap::new();
    hash.insert(String::from(dep), String::from(man));

    return hash;
}

#[test]
fn test_xxx() {
    println!("{:?}", yyy("Add AAA to Foo"))
}