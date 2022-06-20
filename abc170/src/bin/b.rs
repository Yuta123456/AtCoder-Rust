use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize
    }
    for turtle in 0..x+1 {
        if turtle * 2 + (x - turtle) * 4 == y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
