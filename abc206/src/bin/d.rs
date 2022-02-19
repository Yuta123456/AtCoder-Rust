use petgraph::unionfind::UnionFind;
use proconio::input;


fn main() {
    input! {
        n: usize,
        mut a: [usize;n],
    }
    let mut ans = 0;
    let mut tree :UnionFind<usize> = UnionFind::new(200001);
    for i in 0..n {
        let former = a[i];
        let latter = a[n-i-1];
        if !tree.equiv(former, latter) {
            tree.union(former, latter);
            ans += 1
        }
    }
    println!("{}", ans);
}
