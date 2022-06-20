use proconio::input;


fn main() {
    input! {
        n:usize
    }
    let mut s = vec![];
    for _ in 0..n {
        input! {
            t:usize,
            l:usize,
            r:usize
        }
        s.push((t, l, r));
    }
    s.sort_by_key(|a| a.1);
    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let r_i = s[i].2;
            let l_j = s[j].1;
            if r_i < l_j {
                continue;
            }
            if r_i == l_j {
                // どちらも境界値を含んでいる場合のみ+1
                let small_in = s[i].0 == 1 || s[i].0 == 3;
                let big_in = s[j].0 == 1 || s[j].0 == 2;
                if small_in && big_in {
                    ans += 1;
                }
                continue;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
