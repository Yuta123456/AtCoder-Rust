use proconio::*;
use std::collections::*;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        // if map.contains_key(&a[i]) {
        //     map.insert(a[i], map.get(&a[i]).unwrap()+1);
        // } else {
        //     map.insert(a[i], 1);
        // }
        *(map.entry(a[i]).or_insert(0))+=1;
    }


    let mut ans = (n * (n-1) * (n-2)) / 6;
    for key in map.keys() {
        let count = map[&key];
        if count >= 2 {
            let sub = (n - count) * (count * (count - 1)) / 2;
            ans -= sub;
            let add_sub = (count * (count-1) * (count-2)) / 6;
            ans -= add_sub;
        }
    }
    println!("{}", ans);
}
