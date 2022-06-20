use proconio::input;


fn main() {
    input! {
        a:f64,
        b:f64,
        c:f64,
        d:f64
    }
    let k = a / (d * c - b);
    if k < 0.0 || d * c - b == 0.0 {
        println!("-1");
        return;
    }
    println!("{}", k.ceil());
}
