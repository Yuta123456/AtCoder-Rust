use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let num: i32 = n.parse().unwrap();
    if  num >= 42 {
        println!("AGC{0: >03}", num+1);
    } else {
        println!("AGC{0: >03}", num);
    }

}
