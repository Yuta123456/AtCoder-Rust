use proconio::input;


fn main() {
    input!{
        n: f64,
    }
    let price = (n * 1.08) as u64;
    if price < 206 as u64 {
        println!("Yay!")
    } else if price > 206 as u64 {
        println!(":(");
    } else {
        println!("so-so");
    }
}
