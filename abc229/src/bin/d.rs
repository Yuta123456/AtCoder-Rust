use proconio::input;
use std::cmp::*;
fn main() {
    input!{
        // Chars
        s: String,
        k: usize
    }
    let n = s.len();
    let mut dot_count = vec![0;n+1];
    if s.chars().nth(0).unwrap() == '.' as char {
        dot_count[1] = 1
    }
    let s_ch = s.chars().collect::<Vec<char>>();
    for i in 2..n+1 {
        dot_count[i] =  dot_count[i-1];
        if s_ch[i-1] == '.' as char {
            dot_count[i] += 1;
        }
    }
    let mut right = 0;
    let mut ans = 0;
    for left in 0..n {
        while right < n {
            let dot = dot_count[right+1] - dot_count[left];
            if dot <= k {
                ans = max(ans, right - left + 1);
            } else {
                break;
            }
            right += 1;
        }
    }
    println!("{}", ans);
}
