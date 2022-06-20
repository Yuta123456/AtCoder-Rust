
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let mut data: Vec<(usize, usize)> = vec![];
    for _i in 0..n {
        input! {
            l: usize,
            r: usize
        }
        data.push((l, r));
    }
    data.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ans = 0;
    let mut last = 0;
    for i in 0..n {
        let l = data[i].0;
        let r = data[i].1;
        if l <= last {
            continue;
        }
        last = r+d-1;
        ans += 1;
    }
    println!("{}", ans);
}
