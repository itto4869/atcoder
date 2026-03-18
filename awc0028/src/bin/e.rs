use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
        p: [u64; k],
    }
    let mut dp = vec![vec![0; k + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 1;
    }
    for i in 1..=n {
        for j in 1..=k.min(i) {
            if a[i - 1] == p[j - 1] {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % MOD;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        ans = (ans + dp[i][k]) % MOD;
    }
    
    println!("{}", dp[n][k]);
}
