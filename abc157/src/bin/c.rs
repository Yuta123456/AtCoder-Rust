use proconio::input;
use std::collections::*;
fn main() {
    input! {
        n: usize,
        m: usize
    }
    let mut d = vec![];
    let mut ans = vec![0, 0, 0];
    let mut used = HashSet::new();
    for _i in 0..m {
        input! {
            s: usize,
            c: usize
        }
        d.push((s, c));
        if used.contains(&(s, 10)) && !used.contains(&(s, c)) {
            println!("-1");
            return;
        }
        used.insert((s, c));
        used.insert((s, 10));
        
        ans[s-1] = c;
    }
    for i in 0..1000 {
        let mut s = i.to_string();
        if s.len() != n {
            continue;
        }
        let mut flag = true;
        for j in d.iter() {
            if s.chars().nth(j.0-1).unwrap() != std::char::from_digit(j.1 as u32, 10).unwrap() {
                flag = false;
                break;
            }
        }
        if flag {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
    
}
