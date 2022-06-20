use proconio::input;
use std::collections::*;
fn main() {
    input! {
        x: usize,
        n: usize,
    }
    let mut p: Vec<usize> = vec![];
    if n != 0 {
        input! {
            _p: [usize;n],
        }
        p = _p;
    }
    let mut hash = HashSet::new();
    p.iter().for_each(|&v| {
        hash.insert(v);
    });
    let max_p = p.iter().max().unwrap();
    let mut ans = 0;
    for i in 0..max_p+1 {
        if hash.contains(&i) {
            continue;
        }
        if (ans as i32 - x as i32).abs() > (i as i32 - x as i32).abs() {
            ans = i;
        }
    }
    println!("{}", ans);
}
