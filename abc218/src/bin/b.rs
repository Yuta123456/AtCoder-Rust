use proconio::input;

fn main() {
    input!{ 
        p: [usize;26],
    }
    let mut ans: Vec<char> = vec![];
    for i in 0..26 {
        ans.push((96 + p[i] as u8) as char);
    }
    for i in 0..26 {
        print!("{}", ans[i]);
    }
}
