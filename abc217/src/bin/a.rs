use proconio::input;

fn main() {
    input!{
        s: String, 
        t: String,
    }
    if s > t {
        println!("No");
    }else {
        println!("Yes");
    }
}
