use proconio::input;
use itertools::Itertools;
use std::collections::*;
fn main() {
    input! {
        s: String,
        k: usize
    }
    let mut word_list: Vec<String> = vec![];
    let s: &str = &s;
    let mut set = HashSet::new();
    for perm in (0..s.len()).permutations(s.len()) {
        let mut word: Vec<char> = vec![];
        // println!("{:?}", perm);
        perm.iter().for_each(|x| word.push(s.chars().nth(*x).unwrap()));
        if set.contains(&word.iter().collect::<String>()) {
            continue;
        }
        set.insert(word.iter().collect::<String>());
        word_list.push(word.iter().collect::<String>());
    }

    word_list.sort();
    // println!("{:?}", word_list);
    println!("{}", word_list[k-1]);
}
