use proconio::input;
#[warn(dead_code)]
const MOD: usize = 998244353;
fn main() {
    input! {
        n: usize, m: usize, k: usize,
    }
    // DPっぽい
    let mut dp = vec![vec![0;n+1];k+1];
    dp[0][1] = 1;
    let mut ini = vec![];
    for i in 1..(n+1) {
        ini.push(i);
    } 
    // let mut graph: Vec<HashSet<usize>> = vec![HashSet::from_iter(ini.iter().cloned());n+1];
    let mut lost_way: Vec<Vec<usize>> = vec![vec![];n+1];
    for _i in 0..m {
        input! {
            u: usize,
            v: usize,
        }
        // graph[u].remove(&v);
        // graph[v].remove(&u);
        lost_way[u].push(v);
        lost_way[v].push(u);
    }
    for i in 0..k {
        let sum: usize = dp[i].iter().sum();
        for j in 1..(n+1) {
            let mut sub = dp[i][j];
            for q in &lost_way[j] {
                sub += dp[i][*q];
                sub %= MOD;
            }
            dp[i+1][j] = (sum - sub) % MOD;
        }
    }
    // println!("{:?}", dp[k][1]);
    println!("{:?}", dp[k][1]);
}

// S'_j = {jとつながっていない町すべて。}
// dp[i+1][j] = 全部 - つながっていない町