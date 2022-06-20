use proconio::input;
use std::collections::*;
fn main() {
    input! {
        n: usize,

    }
    let mut array = vec![];
    let mut l = vec![];
    for i in 0..n {
        input! {
            _l: usize,
            a : [usize;_l],
        }
        array.push(a);
        l.push(_l);
    }
    let mut set = HashSet::new();
    for i in 0..n {
        let mut string = "".to_string();
        for j in 0..l[i] {
            string += &array[i][j].to_string();
            string += " ";
        }
        set.insert(string);
    }
    println!("{}", set.len());
}
