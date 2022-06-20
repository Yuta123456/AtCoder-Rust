use proconio::*;

fn main() {
    input! {
        n: u32,
        a: [usize; 2i64.pow(n)]
    }
    let n: usize = n as usize;
    let max_a = a.iter().max().unwrap();
    let index = a.iter().position(|&x| x == *max_a).unwrap();
    let middle = n / 2;
    let ans;
    if index > middle {
        let max_half_a = &a[0..middle+1].iter().max().unwrap();
        // println!("{}", max_half_a);
        ans = a.iter().position(|&x| x == **max_half_a).unwrap();
    } else {
        let max_half_a = &a[middle+1..n].iter().max().unwrap();
        ans = a.iter().position(|&x| x == **max_half_a).unwrap();
    }
    println!("{}", ans+1);
}   
