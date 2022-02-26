use proconio::input;
use std::cmp::Ordering;
use std::cmp::min;
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i32;n],
        mut b: [i32;m],
    }
    a.sort();
    b.sort();
    let mut ans = 1000000000;
    for i in a {
        let index = b.upper_bound(&i);
        if index != 0 {
            ans = min(ans, (i - b[index-1]).abs());
        }
        if index != b.len() {
            ans = min(ans, (i - b[index]).abs())
        }
    }
    println!("{}", ans);
}

