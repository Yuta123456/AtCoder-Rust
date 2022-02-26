use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    // let digit_sum_a: i32 = a.chars().collect::<Vec<char>>().iter().map(|ch| {*ch as i32 - 48}).sum();
    // let digit_sum_b: i32 = b.chars().collect::<Vec<char>>().iter().map(|ch| {*ch as i32 - 48}).sum();
    
    let digit_sum_a: i32 = a.chars().collect::<Vec<char>>().iter().map(|ch| {(*ch).to_string().parse::<i32>().unwrap()}).sum();
    let digit_sum_b: i32 = b.chars().collect::<Vec<char>>().iter().map(|ch| {*ch as i32 - 48}).sum();
    
    if digit_sum_a >= digit_sum_b {
        println!("{}", digit_sum_a);
    } else {
        println!("{}", digit_sum_b);
    }
}
