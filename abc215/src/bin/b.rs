use proconio::input;

fn main() {
    input! {
        n: f64
    }
    let mut k = 0.0;
    while (2.0 as f64).powf(k) <= n {
        k += 1.0;
    }
    if k == 0.0 {
        k += 1.0;
    }
    println!("{}", k-1.0);
}
