use proconio::input;
use std::collections::*;

fn main() {
    input!{
        l: usize,
        q: usize,
    }
    let mut query = vec![];
    for _i in 0..q {
        input!{
            c: usize,
            x: usize,
        }
        query.push((c, x));
    }

    let mut tree: BTreeSet<usize> = BTreeSet::new();
    tree.insert(0);
    tree.insert(l);
    for q in query {
        if q.0 == 2 {
            // サイズを確かめる
            let c = q.1;
            let prev = tree.range(..c).next_back().unwrap();
            let next = tree.range(c..).next().unwrap();
            println!("{}", next-prev);
        } else {
            let c = q.1;
            tree.insert(c);
        }
    }
}
