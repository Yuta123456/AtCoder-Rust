use proconio::input;
use std::collections::*;

fn main() {
    input! {
        n: usize,
    }
    let mut names = HashSet::new();
    let mut flag = false;
    for i in 0..n {
        input! {
            s: String, 
            t: String
        }
        let name = format!("{} {}", s, t);
        if names.contains(&name) {
            flag = true;
        }
        names.insert(name);
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
