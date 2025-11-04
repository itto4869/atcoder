use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut wall: [[u64; 10]; 10],
        info: [[i64; w]; h],
    }
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                wall[i][j] = wall[i][j].min(wall[i][k] + wall[k][j]);
            }
        }
    }
    let mut ans = 0;
    for row in info {
        for &n in &row {
            if n == -1 {
                continue;
            } else {
                ans += wall[n as usize][1];
            }
        }
    }
    println!("{}", ans);
}