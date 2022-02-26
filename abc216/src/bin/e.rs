use proconio::input;

fn binary_search(left: usize, right: usize, k: usize, array: &Vec<usize>) -> usize {
    if right - left < 2 {
        return right;
    }
    let middle = (right + left) / 2; 
    if is_count_over_k(array, middle, k) {
        return binary_search(middle, right, k, array);
    } else {
        return binary_search(left, middle, k, array);
    }
}
/**
 * target以上の数字が、k個以下かどうかを判定する関数
 * もし、これがk個以下であれば、その数字以上のものをすべて取るのが最適
 * そのtargetの境界値を求めるのが今回の問題
 */
fn is_count_over_k(array: &Vec<usize>, target: usize, k: usize) -> bool {
    let mut cnt = 0;
    for i in 0..array.len() {
        if  target > array[i] {
            continue;
        }
        cnt += 1 + array[i] - target;
    }
    return cnt > k;
}
fn main() {
    input! {
        n: usize,
        mut k:  usize,
        a: [usize;n],
    }
    // どこまで取るかを二分探索で決定する。
    let ride_cnt = binary_search(0, 10f64.powf(10.0) as usize, k, &a);
    let mut ans = 0;
    for i in 0..n {
        // もしこの乗り物を乗る価値があれば
        if a[i] >= ride_cnt  {
            // ( a[i] - ridecnt ) ~ a[i] までの和を答えに足す
            ans += (a[i] - ride_cnt + 1) * (a[i] + ride_cnt) / 2;
            k -= a[i] - ride_cnt + 1;
        }
    }
    // 残りはride_cnt - 1をとる
    ans += (ride_cnt - 1) * k;
    println!("{}", ans);
}
