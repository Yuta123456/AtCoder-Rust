use proconio::*;
use std::collections::*;
fn main() {
    input! {
        n: usize,
        prime_c: usize,
        info: [[usize;3];n],
    }

    let mut day_set: HashSet<usize> = HashSet::new();
    for i in 0..info.len() {
        let start = info[i][0];
        let end = info[i][1];
        day_set.insert(start);
        day_set.insert(end);
    }
    let mut days: Vec<usize> = day_set.into_iter().collect();
    days.sort();


    days.push(days[days.len()-1] + 1);
    
    let mut day2index: HashMap<usize, usize> = HashMap::new();
    for i in 0..days.len() {
        day2index.insert(days[i], i); 
    }
    let mut cost: Vec<i64> = vec![0;days.len()+1];
    for i in 0..info.len() {
        let start = info[i][0];
        let end = info[i][1];
        let c = info[i][2] as i64;
        let s_index = day2index[&start];
        let e_index = day2index[&end];
        cost[s_index] += c;
        cost[e_index+1] -= c;
    }
    for i in 0..days.len() {
        cost[i+1] += cost[i];
    }
    let mut pre_day = 0;
    let mut now_cost = 0;
    let mut ans = 0;
    for i in 0..days.len() {
        // pre_dayから、days[i](含まない)までのお金を計算。
        let span = days[i] - pre_day;
        if now_cost > prime_c {
            ans += prime_c * span;
        } else {
            ans += now_cost * span;
        }
        now_cost = cost[i] as usize;
        pre_day = days[i];
        // let index = day2index[&days[i]];
    }
    println!("{}", ans);
}
