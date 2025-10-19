use proconio::{fastout, input};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [[u64; w]; h],
    }
    let mut row_sum = vec![0; h];
    let mut column_sum = vec![0; w];
    for i in 0..h {
        row_sum[i] = grid[i].iter().sum();
    }

    for i in 0..w {
        let mut sum = 0;
        for j in 0..h {
            sum += grid[j][i];
        }
        column_sum[i] = sum;
    }

    let mut ans = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            ans[i][j] = row_sum[i] + column_sum[j] - grid[i][j];
        }
    }

    for i in 0..h {
        println!("{}", ans[i].iter().format(" "));
    }
}
