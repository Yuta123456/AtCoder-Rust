use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut vote = vec![];
    let mut sum_a = 0;
    for _i in 0..n {
        input! {
            a: usize, 
            b: usize,
        }
        sum_a += a;
        vote.push((a, b));
    }
    vote.sort_by_key(|a| 2 * a.0 +  a.1);
    vote.reverse();
    let mut takahashi_vote = 0;
    let mut ans = 0;
    while takahashi_vote <= sum_a {
        takahashi_vote += vote[ans].1 + vote[ans].0;
        sum_a -= vote[ans].0;
        ans += 1;
    }
    println!("{}", ans);
}
