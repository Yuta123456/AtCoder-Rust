use proconio::input;
use std::collections::*;

fn main() {
    input!{
        n:usize,
    }
    let mut point = vec![];
    let mut point_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for _i in 0..n {
        input! {
            x: usize,
            y: usize
        }
        point.push((x, y));
        if point_map.contains_key(&x) {
            point_map.get_mut(&x).unwrap().push(y);
        } else {
            point_map.insert(x, vec![y]);
        }
    }
}

