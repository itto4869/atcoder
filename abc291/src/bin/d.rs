use proconio::{fastout, input};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n],
    }
    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..n {
        for j in 0..2 {
            for k in 0..2 {
                if ab[i][j] != ab[i - 1][k] {
                    dp[i][j] = (dp[i][j] + dp[i - 1][k]) % MOD;
                }
            }
        }
    }

    let ans = (dp[n - 1][0] + dp[n - 1][1]) % MOD;
    println!("{}", ans);
}
