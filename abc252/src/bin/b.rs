use proconio::*;
use std::collections::*;
// #![recursion_limit = "256"]
fn dislike(b: &Vec<usize>, i: usize) -> bool {
    for idx in 0..b.len() {
        if b[idx] - 1 == i {
            return true;
        }
    }
    return false;
}
fn main() {
    input!{
        n: usize,
        k: usize,
        mut a : [usize;n],
        b: [usize;k],
    }
    let max_val = a.iter().max().unwrap();
    for i in 0..n {
        if a[i] == *max_val && dislike(&b, i){
            println!("Yes");
            return;
        }
    }
    println!("No");
}
