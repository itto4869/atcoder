use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut a_grid: [[i64; w]; h],
        b_grid: [[i64; w]; h],
    }
    let mut ans = 0;
    for i in 0..(h - 1) {
        for j in 0..(w - 1) {
            let d = b_grid[i][j] - a_grid[i][j];
            ans += d.abs();
            a_grid[i][j] += d;
            a_grid[i + 1][j] += d;
            a_grid[i][j + 1] += d;
            a_grid[i + 1][j + 1] += d;
        }
    }

    let mut ok = true;
    for i in 0..h {
        for j in 0..w {
            if a_grid[i][j] != b_grid[i][j] {
                ok = false;
            }
        }
    }

    if ok {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
