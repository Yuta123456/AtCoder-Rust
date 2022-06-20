use proconio::input;
use std::collections::*;

fn dfs(node: usize, finished: &mut HashSet<usize>, graph: &Vec<Vec<usize>>){
    print!("{} ", node);
    finished.insert(node);
    for next in graph[node].iter() {
        if !finished.contains(&next) {
            dfs(*next, finished, graph);
            print!("{} ", node);
        }
    }
}

fn main() {
    input! {
        n: usize
    }
    let mut edges = vec![];
    let mut graph: Vec<Vec<usize>> = vec![vec![];n+1];

    for _i in 0..n-1 {
        input! {
            a: usize,
            b: usize
        }
        edges.push((a, b));
        graph[a].push(b);
        graph[b].push(a);
    }
    for i in 0..n+1 {
        graph[i].sort();
    }
    let node = 1;
    let mut finished = HashSet::new();
    // println!("{:?}", graph);
    dfs(node, &mut finished, &graph);
    // loop {
    //     route.push(node);
    //     finished.insert(node);
    //     for next in graph[node] {
    //         if !finished.contains(&next) {
    //             node = next;
    //             pre_index = route.len();
    //             continue;
    //         }
    //     }
    //     if node == 1 {
    //         break;
    //     } else {
    //         node = route[pre_index-1];
    //         pre_index -= 1;
    //     }

    // }
}
