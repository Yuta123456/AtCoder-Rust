use proconio::input;
use std::collections::*;
fn main() {
    input!{
        s1: String, 
        s2: String,
        s3: String
    }
    // let mut set: HashSet<String> = HashSet::new();
    let mut set = HashSet::new();
    set.insert(String::from("ABC"));
    set.insert(String::from("ARC"));
    set.insert(String::from("AGC"));
    set.insert(String::from("AHC"));
    set.remove(&s1);
    set.remove(&s2);
    set.remove(&s3);
    for i in set {
        println!("{}", i);
    }
}
