use proconio::*;

fn main() {
    input!{
        x: isize,
        y: isize
    }
    if (x - y).abs() >= 3 {
        println!("No");
    } else {
        println!("Yes");
    }
}
