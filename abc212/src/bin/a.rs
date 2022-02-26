use proconio::input;


fn main() {
    input! {
        a: usize,
        b:usize
    }
    if a > 0 && b == 0 {
        println!("Gold");
    } else if a == 0 && 0 < b {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
