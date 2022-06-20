use proconio::input;
use std::cmp::Reverse;
fn main() {
    input!{
        n: usize, 
        w: usize
    }
    let mut cheese = vec![];
    for i in 0..n {
        input!{
            a: usize,
            b: usize
        }
        cheese.push((a, b));
    }
    cheese.sort_by_key(|&x| Reverse(x));
    let mut weight = 0;
    let mut cheese_index = 0;
    let mut value = 0;
    while  weight < w && cheese_index < n {
        let v = cheese[cheese_index].0;
        let rem = cheese[cheese_index].1;
        let unused = w - weight;
        if unused < rem {
            value += unused * v;
            weight += unused;
            break;
        }
        value += rem * v;
        weight += rem;
        cheese_index += 1;
    }
    println!("{}", value);
    
}
