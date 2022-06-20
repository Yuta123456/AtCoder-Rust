use proconio::input;

fn check(v1: usize, v2: usize, v3: usize) -> bool{
    return v1 == 0 && v2 == 0 && v3 == 0;
}
fn main() {
    let mut a = vec![];
    for _i in 0..3 {
        input! {
            _a: [usize; 3],   
        }
        a.push(_a);
    }
    input! {
        n: usize,
    }
    for _ in 0..n {
        input! {
            b: usize
        }
        for i in 0..3 {
            for j in 0..3 {
                if a[i][j] == b {
                    a[i][j] = 0;
                }
            }
        }
    }

    let flag = check(a[0][0], a[0][1], a[0][2]) || 
    check(a[1][0], a[1][1], a[1][2]) || 
    check(a[2][0], a[2][1], a[2][2]) || 

    check(a[0][0], a[1][0], a[2][0]) || 
    check(a[0][1], a[1][1], a[2][1]) || 
    check(a[0][2], a[1][2], a[2][2]) || 

    check(a[0][0], a[1][1], a[2][2]) || 
    check(a[2][0], a[1][1], a[2][0]);
    if flag {
        println!("Yes");
    }else {
        println!("No");
    }
}
