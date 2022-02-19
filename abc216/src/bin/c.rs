use proconio::input;
fn main() {
    input!{
        mut n: usize,
    };
    let mut ans = String::new();
    while n != 0 {
        if n % 2 == 0 {
            n = n / 2;
            ans = ans.to_string() + "B";
        } else {
            n -= 1;
            ans = ans.to_string() + "A";
        }
    }
    println!("{}", ans.chars().rev().collect::<String>());
}