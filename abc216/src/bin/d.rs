use proconio::input;
use std::collections::*;

fn has_cycle(
    graph: &Vec<Vec<usize>>,
    node: usize, 
    visited: &mut HashSet<usize>,
    stack: &mut HashSet<usize>
) -> bool { 
    for &next in graph[node].iter() {
        if stack.contains(&next) {
            return true;
        }
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);
        stack.insert(next);
        if has_cycle(graph, next, visited, stack) {
            return true;
        }
        stack.remove(&next);
    }
    return false;
}

fn main() {
    input! {
        n:usize,m:usize,
    }
    let mut graph = vec![vec![]; n];
    for _i in 0..m {
        input!{
            k:usize,
            a:[usize;k],
        }
        // グラフ構築
        for i in 0..(k-1) {
            let from = a[i] - 1;
            let to = a[i+1] - 1;
            graph[from].push(to);
        }
    }
    let mut visited = HashSet::new();
    for node in 0..n {
        if visited.contains(&node) {
            continue;
        }
        let mut stack = HashSet::new();
        visited.insert(node);
        stack.insert(node);
        if has_cycle(&graph, node, &mut visited, &mut stack) {
            println!("No");
            return;
        }
    }
    println!("Yes")
    
}
