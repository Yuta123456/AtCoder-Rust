use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut point = vec![];
    for _i in 0..n {
        input! {
            x: f64,
            y: f64,
        }
        point.push((x, y));
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            let inc = (point[i].1 - point[j].1) / (point[i].0 - point[j].0);
            if -1.0 <= inc && inc <= 1.0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
