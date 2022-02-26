use proconio::input;
fn binary_search(left: usize, right: usize, array: &Vec<(usize, usize)>, target: usize) -> usize {
    if right - left < 2 {
        return left;
    }
    let middle = (right + left) / 2; 
    if array[middle].1 > target {
        return binary_search(left, middle, array, target);
    } else {
        return binary_search(middle, right, array, target);
    }
}
fn main() {
    input!{
        l: usize,
        q: usize,
    }
    let mut query = vec![];
    for _i in 0..q {
        input!{
            c: usize,
            x: usize,
        }
        query.push((c, x));
    }
    let mut cut = vec![];
    let mut ans = vec![];
    for i in 0..q {
        if query[i].0 == 1 {
            cut.push(query[i].1);
        }
    }
    cut.sort();
    // let mut to = 0;
    // (長さ, 位置)
    let mut tree = vec![];
    tree.push((0, 0));
    let mut pre = 0;
    for i in cut {
        tree.push((i - pre, i));
        pre = i;
    }
    tree.push((l-pre, l));
    query.reverse();
    for q in query {
        if q.0 == 2 {
            // サイズを確かめる
            // let c = q.1;
            let index = binary_search(0, tree.len(), &tree, q.1);
            ans.push(tree[index+1].0);
        } else {
            // つなぐ
            // let c = q.1;
            let index = binary_search(0, tree.len(), &tree, q.1);
            tree[index+1] = (tree[index].0 + tree[index+1].0, tree[index+1].1);
            tree.remove(index);
        }
    }
    ans.reverse();
    for i in ans {
        println!("{}", i);
    }
}
