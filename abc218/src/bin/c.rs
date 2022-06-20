use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut s = vec![];
    let mut t = vec![];
    for _i in 0..n {
        input! {
            _s: String,
        }
        s.push(_s);
    }

    for _i in 0..n {
        input! {
            _t: String,
        }
        t.push(_t);
    }
}
