use proconio::input;
use std::collections::*;

fn main() {
    input! {
        n: usize,
    }
    let mut set: HashSet<String> = HashSet::new();
    for _i in 0..n {
        input! {
            s: String,
        }
        set.insert(s);
    }
    for i in &set {
        let key = "!".to_string() + &i;
        if set.contains(&key) {
            println!("{}", &i);
            return;
        }
    }
    println!("satisfiable");
}
