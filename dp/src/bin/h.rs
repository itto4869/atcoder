use proconio::{fastout, input, marker::Bytes};

const MOD: u64 = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        grid: [Bytes; h],
    }
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;

    for i in 0..h {
        for j in 0..w {
            if (i + 1) < h && grid[i + 1][j] == b'.' {
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % MOD;
            }

            if (j + 1) < w && grid[i][j + 1] == b'.' {
                dp[i][j + 1] = (dp[i][j + 1] + dp[i][j]) % MOD;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
