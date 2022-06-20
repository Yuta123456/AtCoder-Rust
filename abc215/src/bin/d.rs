use proconio::input;
use std::collections::*;
fn factorization(n: usize) -> Vec<usize> {
    let mut res = vec![];
    for i in 1..((n as f64).sqrt() + 1.0) as usize {
        if n % i == 0 {
            res.push(i); 
            res.push(n/i);
        }
    }
    let res: HashSet<usize> = res.into_iter().collect();
    let res: Vec<usize> = res.into_iter().collect();
    return res;
}
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    }
    let mut factors: Vec<usize> = vec![];
    for i in a {
        let f = factorization(i);
        f.iter().for_each(|x| factors.push(*x));
    }
    let factors: HashSet<usize> = factors.into_iter().collect();

    let mut output = vec![true; m+1];
    // println!("{:?}", factors);
    output[0] = false;
    for f in factors {
        if f == 1 {
            continue;
        }
        let mut false_index = f;
        while false_index <= m {
            output[false_index] = false;
            false_index += f;
        }
    }
    let mut ans = vec![];
    (0..m+1).for_each(|x| 
        if output[x] {
          ans.push(x);
    });
    println!("{}", ans.len());
    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }

}
