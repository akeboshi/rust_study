/*
  ハッシュマップとベクタを使用して、
  ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
   例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。 
   それからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。
*/
use std::collections::HashMap;
use regex::Regex;

struct Company {
    employee: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Self {
        Self {
            employee: HashMap::new()
        }
    }

    pub fn command(self, text: &str) -> Self {
        let add_re = Regex::new(r"Add (.+) to (.+)").unwrap();
        let list_re = Regex::new(r"List (.+)").unwrap();
        
        let add_caps = add_re.captures(text);

        if let Some(c) = add_caps {
            return self.add(c.at(2).unwrap(), c.at(1).unwrap());
        }

        let list_caps = list_re.captures(text);

        if let Some(c) = list_caps {
            self.list(c.at(1).unwrap());
            return self;
        }

        return self;
    }

    fn add(mut self, dep: &str, man: &str) -> Self {
        let mans = self.employee.get_mut(&String::from(dep));
        let mut values: Vec<String> = match mans {
            None => Vec::new(),
            Some(some) => some.to_vec(),
        };
        values.push(String::from(man));
        self.employee.insert(String::from(dep), values);
        return self;
    }

    fn list(&self, dep: &str) {
        let mans = self.employee.get(dep);
        match mans {
            None => println!("そこにはだれもいませんよ"),
            Some(some) => println!("---- {:?} ----", some),
        }
    }
}

#[test]
fn test_entry_employee() {
    let b: Company = Company::new();
    let c = b.command("Add AAA to Foo").command("Add Bar to Foo");
    c.command("List Foo");
}