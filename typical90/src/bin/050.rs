use proconio::{fastout, input};

const MOD: u64 = 1_000_000_007;
#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
    }
    let mut dp = vec![0; 250000];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] = (dp[i + 1] + dp[i]) % MOD;
        dp[i + l] = (dp[i + l] + dp[i]) % MOD;
    }

    println!("{}", dp[n]);
}
