use proconio::input;
// use std::collections::*;
fn main() {
    input! {
        s: usize, p:f64,
    }
    let mut div_p = Vec::new();
    for i in 1..(p.sqrt() + 1f64) as usize {
        if p as usize % i == 0 {
            div_p.push(i);
        }
    }

    for i in div_p {
        let m = s - i;
        if m * i == p as usize{
            println!("Yes");
            return;
        }
    }
    println!("No");
}
