use proconio::input;
use std::collections::*;
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    let mut d = vec![];
    for _ in 0..n {
        input! {
            _a: usize,
            _b: usize
        }
        a.push(_a);
        b.push(_b);
    }
    for _ in 0..q {
        input! {
            _c: usize,
            _d: usize
        }
        c.push(_c-1);
        d.push(_d);
    }
    let infants_len = 2*100000 + 1;
    let mut maps: Vec<BTreeSet<(usize, usize)>> = vec![BTreeSet::new();infants_len];
    for i in 0..n {
        let rate = a[i];
        let infant = b[i];
        maps[infant].insert((rate, i));
    }
    let mut strong_infants :BTreeSet<(usize, usize)> = BTreeSet::new();
    // それぞれの幼稚園の最強を持ってくる。
    for i in 0..infants_len {
        if let Some(&max_rate) = maps[i].iter().rev().nth(0) {
            strong_infants.insert(max_rate);
        }
    }
    for _q in 0..q {
        let infant = c[_q];
        let to = d[_q];
        let infant_rate = a[infant];
        let from = b[infant];
        let value  = (infant_rate, infant);

        if let Some(from_strongest) = maps[from].iter().rev().nth(0) {
            strong_infants.remove(from_strongest);
        }
        if let Some(to_strongest) = maps[to].iter().rev().nth(0) {
            strong_infants.remove(to_strongest);
        }

        // 元の幼稚園から削除
        maps[from].remove(&value);
        // 移動先の幼稚園をメモしておく
        b[infant] = to;

        maps[to].insert(value);
        
        if let Some(&from_strongest) = maps[from].iter().rev().nth(0) {
            strong_infants.insert(from_strongest);
        }
        if let Some(&to_strongest) = maps[to].iter().rev().nth(0) {
            strong_infants.insert(to_strongest);
        }
        let ans = strong_infants.iter().nth(0).unwrap();
        println!("{}", ans.0);

    }
}
