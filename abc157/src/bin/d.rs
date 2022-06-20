use proconio::input;
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize
    }
    let mut uf = UnionFind::new(n+1);
    let mut friends = vec![vec![]; n+1];
    let mut not_friends = vec![vec![]; n+1];
    for _i in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        uf.unite(a, b);
        friends[a].push(b);
        friends[b].push(a);
    }
    for _i in 0..k {
        input! {
            a: usize,
            b: usize,
        }
        not_friends[a].push(b);
        not_friends[b].push(a);
    }
    for i in 1..n+1 {
        // 自分を引く
        let mut ans = uf.size(i) - 1;
        // 友達の数を引く
        ans -= friends[i].len();
        for j in not_friends[i].iter() {
            if uf.issame(i, *j) {
                ans -= 1;
            }
        }
        print!("{} ", ans);
    }
    
}
