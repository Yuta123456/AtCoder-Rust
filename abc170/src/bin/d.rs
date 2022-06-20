use proconio::input;
use std::collections::*;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    }
    a.sort();
    let inf = 1000003;
    let mut hash_map: HashMap<usize, usize> = HashMap::new();
    let mut flag = vec![true;inf];
    for i in 0..n {
        if hash_map.get(&a[i]) != None {
            hash_map.insert(a[i], hash_map.get(&a[i]).unwrap() + 1);
            continue;
        } else {
            hash_map.insert(a[i], 1);
        }
        if flag[a[i]] {
            let mut v = a[i];
            while v + a[i] < inf {
                v += a[i];
                flag[v] = false;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if *(hash_map.get(&a[i]).unwrap()) >= 2 {
            continue;
        } 
        if flag[a[i]] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
