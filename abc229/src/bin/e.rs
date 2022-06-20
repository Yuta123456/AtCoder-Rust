use proconio::*;
use petgraph::unionfind::UnionFind;
fn main() {
    input!{ 
        n: usize,
        m: usize
    }
    let mut edges = vec![];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize
        }
        edges.push((a, b));
    }
    let mut adjacent_list: Vec<Vec<usize>> = vec![vec![]; n+1];
    for (a, b) in edges {
        adjacent_list[a].push(b);
        adjacent_list[b].push(a);
    }
    let mut ans = vec![0;1];
    let mut tree :UnionFind<usize> = UnionFind::new(n+1);
    let mut count = 0;
    for i in (1..n+1).rev() {
        count += 1;
        for &next in adjacent_list[i].iter() {
            if next < i {
                continue;
            }
            if !tree.equiv(next, i) {
                count -= 1;
            }
            tree.union(i, next);
        }
        if i == 1 {
            break;
        }
        ans.push(count);
    }
    ans.reverse();
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
