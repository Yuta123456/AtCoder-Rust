use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,

    }
    // 操作
    let mut heap = BinaryHeap::new();
    let mut add = 0;
    for _i in 0..q {
        input! {
            kind: usize,
            // num: isize,
        }
        if kind == 1 {
            input! {
                num:isize
            }
            heap.push(-(num - add));
        } else if kind == 2 {
            input! {
                num:isize
            }
            add += num;
        } else {
            let mut a = heap.pop().unwrap() * -1;
            a += add;
            println!("{}", a);
        }
    } 
}
