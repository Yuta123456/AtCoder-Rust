use proconio::input;

fn main() {
    input! {
        s: String
    }
    let ans = "Hello,World!";
    if s == ans {
        println!("AC");
    } else {
        println!("WA");
    }
}
