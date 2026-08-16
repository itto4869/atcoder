use proconio::{fastout, input, marker::Chars};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    let mut dp = vec![vec![0usize; w]; h];
    let mut row = vec![vec![0usize; w]; h];
    let mut col = vec![vec![0usize; w]; h];
    let mut diag = vec![vec![0usize; w]; h];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                continue;
            }

            if i == 0 && j == 0 {
                dp[i][j] = 1;
            } else {
                if j > 0 {
                    dp[i][j] += row[i][j - 1];
                }

                if i > 0 {
                    dp[i][j] += col[i - 1][j];
                }

                if i > 0 && j > 0 {
                    dp[i][j] += diag[i - 1][j - 1];
                }

                dp[i][j] %= MOD;
            }

            row[i][j] = dp[i][j];
            col[i][j] = dp[i][j];
            diag[i][j] = dp[i][j];

            if j > 0 {
                row[i][j] += row[i][j - 1];
                row[i][j] %= MOD;
            }

            if i > 0 {
                col[i][j] += col[i - 1][j];
                col[i][j] %= MOD;
            }

            if i > 0 && j > 0 {
                diag[i][j] += diag[i - 1][j - 1];
                diag[i][j] %= MOD;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}