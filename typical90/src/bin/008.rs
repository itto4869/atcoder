use proconio::{fastout, input, marker::Chars};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![vec![0usize; 8]; n + 1];
    let atcoder = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
    dp[0][0] = 1;
    for i in 1..=n {
        dp[i][0] = 1;
        for j in 1..=7 {
            if atcoder[j - 1] == s[i - 1] {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % MOD;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    let ans = dp[n][7];
    println!("{}", ans);
}
