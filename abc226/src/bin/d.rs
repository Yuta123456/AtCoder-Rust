use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let mut point = vec![];
    for _i in 0..n {
        input!{ 
            x: f64,
            y: f64
        }
        point.push((x, y));
    }
    let mut magic: Vec<(f64, f64)> = vec![];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let dx: f64 = point[i].0 - point[j].0;
            let dy: f64 = point[i].1 - point[j].1;
            let mut flag = true;
            for k in magic.iter() {
                let m_dx = k.0;
                let m_dy = k.1;
                let mult = m_dx / dx;
                if mult < 0. {
                    continue;
                }
                if dy * mult == m_dy {
                    flag = false;
                    break;
                }
            }
            for k in magic.iter() {
                if k.0 == dx && k.1 == dy {
                    flag = false;
                }
            }
            if flag {
                magic.push((dx as f64, dy as f64));
            }
        }
    }
    println!("{}", magic.len());
    // println!("{:?}", magic);
}
