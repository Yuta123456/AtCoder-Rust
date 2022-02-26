use proconio::input;
fn main() {
    input!{
        s: String,
    }
    let a: Vec<&str> = s.split('.').collect();
    let x = a[0];
    let y: usize = a[1].to_string().parse().unwrap();
    if y <= 2 {
        println!("{}", format!("{}{}", x, "-"));
    } else if 3 <= y && y <= 6 {
        println!("{}", format!("{}", x));
    } else {
        println!("{}", format!("{}{}", x, "+"));
    }
}
