use proconio::input;
use std::collections::*;
fn kkkk(v: (f64, f64), deg: f64, center:(f64, f64)) -> (f64, f64){
    let x = v.0 - center.0;
    let y = v.1 - center.1;
    let mut new_x = x as f64 * deg.cos() - y as f64 * deg.sin() as f64;
    let mut new_y = x as f64 * deg.sin() + y as f64 * deg.cos() as f64;
    new_x += center.0;
    new_y += center.1;
    new_x *= 1000f64;
    new_y *= 1000f64;
    new_x = new_x.round() / 1000f64;
    new_y = new_y.round() / 1000f64;
    return (new_x , new_y);
} 
fn rotate(a: &Vec<(f64, f64)>, deg: f64, center: (f64, f64)) -> Vec<(f64, f64)>{
    return a.iter().map(|v| kkkk(*v, deg, center)).collect::<Vec<(f64,f64)>>();
}
fn check(a: &Vec<(f64, f64)>, b: &Vec<(f64, f64)>) -> bool {
    let mut set = HashSet::new();
    for i in 0..a.len() {
        set.insert((a[i].0 as isize, a[i].1 as isize));
    }
    for i in 0..a.len() {
        if !set.contains(&(b[i].0 as isize,b[i].1 as isize)) {
            return false;
        }
    }
    return true;
}
fn main() {
    input! {
        n:usize,
    }
    let mut s = vec![];
    for _ in 0..n {
        input! {
            a:f64,
            b:f64
        }
        s.push((a, b));
    }
    let mut t = vec![];
    for _ in 0..n {
        input! {
            a:f64,
            b:f64
        }
        t.push((a, b));
    }
    let t = t;
    let s = s;
    // 相対的な位置は変わらない。
    for i in 0..n {
        for j in 0..n {
            // jの位置のものを、iまで移動した後、色々回転してみて一致するかどうか確かめる。
            let sub = (s[i].0 - t[j].0, s[i].1 - t[j].1);
            let new_t = t.iter().map(|v| (v.0 + sub.0, v.1 + sub.1)).collect();
            let mut degs = vec![];
            for deg in degs {
                let c = rotate(&new_t, deg as f64, s[i]);
                if check(&s, &c) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
