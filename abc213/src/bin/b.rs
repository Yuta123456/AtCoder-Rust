use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize;n]
    }
    let mut b = vec![];
    a.iter().for_each(|x| b.push(x));
    b.sort();
    let booby_score = b[n-2];
    // println!("{:?}", b);
    for i in 0..n {
        if a[i] == *booby_score {
            println!("{}", i+1);
            return;
        }
    }
}
