use proconio::input;
use std::cmp::*;
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        p: usize,
        q: usize,
        r: usize,
        s: usize
    }
    let a: i32 = (a as i32 - p as i32) as i32;
    let b: i32 = (b as i32 - q as i32) as i32;
    let mut ans = vec![vec!["."; s-r]];
    let max_1 = max(1 - a as i32, 1 - b as i32);
    let max_2 = max(1 - a as i32, (b - n as i32 - q as i32) as i32);
    let min_1 = min((n as i32 - a as i32 - p as i32) as i32, (n as i32 - b as i32 - q as i32) as i32);
    let min_2 = min((n as i32 - a as i32- p as i32 ) as i32, (b - 1) as i32);
    
    let a: usize = a as usize;
    let b: usize = b as usize;
    for i in max_1..=min_1 {
        if i < 0 {
            continue;
        }
        let i = i as usize;
        ans[a+i][b+i] = "#";
    }

    for i in max_2..=min_2 {
        if i < 0 {
            continue;
        }
        let i = i as usize;
        ans[a+i][b-i] = "#";
    }

    for i in 0..s-r {
        for j in 0..q-p {
            print!("{}", ans[i][j]);
        }
        println!("");
    }
}
