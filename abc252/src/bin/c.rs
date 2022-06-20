use proconio::*;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
fn main() {
    input!{
        n: usize,
        s: [Chars; n],
    }
    let mut ans = 10000000000;
    for target in 0..9 {
        let mut time = 0; 
        for _j in 0..n {
            let mut finish: HashSet<usize> = HashSet::new();
            // どの列を止めるか考える
            let mut stop = n+1;
            // 一番近いとこ
            let mut min_t = 11;
            for i in 0..n {
                // もしこの列が止まってたらスキップ
                if finish.contains(&i) {
                    continue;
                }
                let index = s[i].iter().position(|v| *v as usize - 48 == target).unwrap();
                let to_stop = (index + 10 - (time % 10)) % 10;
                if min_t > to_stop && (_j == 0 || to_stop != 0) {
                    min_t = to_stop;
                    stop = i;
                }
            }
            time += min_t;
            finish.insert(stop);
        }
        ans = min(time, ans);
    }
    println!("{}", ans);
}
