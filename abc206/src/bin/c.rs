use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;
fn main() {
    input!{
        n: u64,
        a: [u64;n], 
    }
    let mut map = HashMap::new();
    for i in 0..n {
        let key  = a[i as usize].to_string();
        let mut value = 1;
        if map.contains_key(&key) {
            value = map[&key] + 1;
        }
        map.insert(key, value);
    }
    for i in 0..n {
        // let key = a[i as usize].to_string();
        // println!("{} {}",a[i as usize], map[&key]);
    }
    let mut ans = n * (n-1) / 2;
    let mut set  = HashSet::new();
    for i in 0..n {
        let key = a[i as usize].to_string();
        if map.contains_key(&key) && !set.contains(&key){
            let value = map[&key];
            ans -= value * (value-1) / 2;
        }
        set.insert(key);
    }
    println!("{}",ans);
}   
