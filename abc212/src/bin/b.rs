use proconio::input;


fn main() {
    input! {
        pass: String,
    }

    let p: &str = &pass;
    if p.chars().nth(0).unwrap() == p.chars().nth(1).unwrap() 
    && p.chars().nth(1).unwrap() == p.chars().nth(2).unwrap() 
    && p.chars().nth(2).unwrap() == p.chars().nth(3).unwrap() {
        println!("Weak");
        return;
    }
    if (p.chars().nth(0).unwrap() as i32 - 45) % 10 == (p.chars().nth(1).unwrap() as i32 - 46) % 10 
    && (p.chars().nth(1).unwrap() as i32 - 46) % 10 == (p.chars().nth(2).unwrap() as i32 - 47) % 10
    && (p.chars().nth(2).unwrap() as i32 - 47) % 10 == (p.chars().nth(3).unwrap() as i32 - 48) % 10{
        println!("Weak");
        return;
    }
    println!("Strong");
}
