use proconio::input;
use std::collections::*;
fn calc_rank( rank: usize,
              node: usize,
              finished: &mut HashSet<usize>,
              graph: &Vec<Vec<usize>>,
              ranks: &mut Vec<usize>) {
    ranks[node] = rank;
    finished.insert(node);
    for next in graph[node].iter() {
        if !finished.contains(&next) {
            calc_rank(rank+1, *next, finished, &graph, ranks);
        }
    }
}

fn main() {
    input!{
        n: usize,
    }
    let mut edge = vec![];
    let mut graph = vec![vec![]; n+1];
    let root = 1;
    for _i in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        edge.push((a, b));
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut rank = vec![n+1;n+1];
    let mut finished = HashSet::new();
    calc_rank(0, root, &mut finished, &graph, &mut rank);
    let mut sub_value = vec![0;n+1];
    let mut add_value = 0;
    println!("{:?}", rank);
    input! {
        q: usize
    }
    for _i in 0..q {
        input! {
            t: usize,
            e: usize,
            x: usize,
        }
        let mut parent = edge[e].0;
        let mut child = edge[e].1;
        if rank[parent] > rank[child] {
            let tmp = child;
            child = parent;
            parent = tmp;
        }
        add_value += x;
        if t == 1 {

        } else {

        }
    }


}
// 木を作成。まず根を決める -> 1
// クエリが与えられた際、根に近いほうがどっちか考える。
// 根が遠かったほうのノードに、-(x_i)を振っておく。かつ、すべてのノードにx_iを足す。
// すべてのクエリが終了後、根からすべてのノード深さ優先で進んでいくが、ノードに振ってある数字だけ持って置き、そのノードから引く。