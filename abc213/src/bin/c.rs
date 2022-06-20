use proconio::input;
use std::collections::*;
fn main() {
    input!{
        _h: usize,
        _w: usize,
        n: usize,
    }
    let mut a = vec![];
    let mut b = vec![];
    for _i in 0..n {
        input!{
            _a: usize,
            _b: usize
        }
        a.push(_a);
        b.push(_b);
    }
    let mut sort_a = HashSet::new();
    let mut sort_b = HashSet::new();
    for i in 0..n {
        sort_a.insert(a[i]);
        sort_b.insert(b[i]);
    }
    let mut sort_a: Vec<&usize> = sort_a.iter().collect();
    let mut sort_b: Vec<&usize> = sort_b.iter().collect();
    sort_a.sort();
    sort_b.sort();
    for i in 0..n {
        // この数字が最終的にどの座標にあるのかは、
        // この数字の左側と上側にどれだけ数字があるかで決まる
        let x = sort_a.binary_search(&&a[i]).unwrap();
        let y = sort_b.binary_search(&&b[i]).unwrap();
        println!("{0} {1}", x+1, y+1);
    }
}
