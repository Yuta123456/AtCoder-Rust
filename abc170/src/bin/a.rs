use proconio::input;

fn main() {
    input! {
        a: [usize;5],
    }
    for i in 0..5 {
        if a[i] == 0 {
            println!("{}", i+1);
            return;
        }
    }
}

