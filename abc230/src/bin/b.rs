use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut original = "oxx".to_string();
    let add = "oxx";
    for _i in 0..20 {
        original += add;
    }
    for i in 0..30 {
        if s == original[i..i+s.len()] {
            println!("Yes");
            return;
        }
    }
    println!("No")
}
