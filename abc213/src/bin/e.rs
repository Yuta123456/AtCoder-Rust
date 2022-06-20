use proconio::input;
use std::collections::*;
fn bfs(cost: &mut Vec<Vec<usize>>, x: isize, y: isize, w: usize, h: usize, graph: Vec<Vec<String>>) {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
    queue.push_back((x, y));
    while queue.len() != 0 {
        let point = queue.pop_front().unwrap();
        let x: isize = point.0;
        let y: isize = point.1;
        if  0 <= x && x < w as isize && 0 <= y && y < h as isize {
            let c = cost[y as usize][x as usize];
            for d in [(0,1), (1,0), (-1, 0), (0, -1)] {
                let dx = d.0;
                let dy = d.1;
                // この移動は、コストが0の時じゃないと意味がない。
                if dx + x as isize > 0 && dy + y as isize > 0 {
                    let new_x = (dx + x) as usize; 
                    let new_y = (dy + y) as usize;
                    if graph[new_y][new_x] == "." {
                        cost[new_y][new_x] = min(c, cost[new_y][new_x]);
                    }  
                }
            }
        }
    }
}
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut graph = vec![];
    for _ in 0..h {
        input! {
            _v: [String; w]
        }
        graph.push(_v);
    }
    let mut cost = vec![vec![10000; w];h];
    cost[0][0] = 0;
    bfs(&mut cost, 0, 0, w, h);
}
