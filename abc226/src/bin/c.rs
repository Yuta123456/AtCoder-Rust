use proconio::input;
use std::cmp::Reverse;
use std::collections::*;
fn calc_time(technique: &Vec<Vec<usize>>, all_t: &Vec<usize>, all_k: &Vec<usize>, target: usize, fin: &mut HashSet<usize>) -> usize{
    let mut res = all_t[target];
    for t in &technique[target] {
        if fin.contains(t) {
            continue;
        }
        res += calc_time(technique, all_t, all_k, *t, fin);
        fin.insert(*t);
    }
    return res;
}
fn main() {
    let mut technique = vec![];
    input! { 
        n: usize,
    }
    let mut all_t = vec![];
    let mut all_k = vec![];
    for _ in 0..n {
        input! {
            t: usize,
            k: usize,
            mut a: [usize;k]
        }
        a.sort_by_key(|&x| Reverse(x));
        a = a.iter().map(|x| x-1).collect();
        technique.push(a);
        all_t.push(t);
        all_k.push(k);
    }
    let mut fin = HashSet::new();
    let ans = calc_time(&technique, &all_t, &all_k, n-1, &mut fin);
    println!("{}", ans);
}
