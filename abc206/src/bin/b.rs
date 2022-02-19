use proconio::input;
use std::process::exit;
fn main() {
    input!{
        n: u64,
    }
    let mut sum = 0;
    for i in 0..(n+1) {
        sum += i;
        if sum >= n {
            println!("{}", i);
            exit(0);
        }
    }
}

