use proconio::input;

fn main() {
    input!{
        n: usize,
        p: [usize;n],
    }
    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i]-1] = i + 1;
    }
    for i in 0..n {
        print!("{} ", q[i]);
    }
    println!();

}
