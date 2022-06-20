use proconio::input;

fn main() {
    input! {
        array: [usize;3]
    }
    println!("{}", array.iter().sum::<usize>() - array.iter().min().unwrap());

}
