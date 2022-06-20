use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [isize;n],
        b: [isize;n]
    }
    let mut sum = 0;
    for i in 0..n {
        sum += a[i] * b[i];
    }
    if sum == 0 {
        println!("Yes");
    } else {
        println!("No");
    }

}
