use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }
    let n = s.len();
    let mut dp = vec![[1usize << 60; 2]; n];
    if s[0] == 'a' {
        dp[0][0] = x.min(z + y + z);
        dp[0][1] = (x + z).min(z + y);
    } else {
        dp[0][0] = y.min(z + x + z);
        dp[0][1] = (y + z).min(z + x);
    }

    for i in 1..n {
        if s[i] == 'a' {
            dp[i][0] = (dp[i - 1][0] + x).min(dp[i - 1][0] + z + y + z).min(dp[i - 1][1] + y + z).min(dp[i - 1][1] + z + x);
            dp[i][1] = (dp[i - 1][0] + x + z).min(dp[i - 1][0] + z + y).min(dp[i - 1][1] + y).min(dp[i - 1][1] + z + x + z);
        } else {
            dp[i][0] = (dp[i - 1][0] + y).min(dp[i - 1][0] + z + x + z).min(dp[i - 1][1] + x + z).min(dp[i - 1][1] + z + y);
            dp[i][1] = (dp[i - 1][0] + y + z).min(dp[i - 1][0] + z + x).min(dp[i - 1][1] + x).min(dp[i - 1][1] + z + y + z);
        }
    }

    let ans = dp[n - 1][0].min(dp[n - 1][1]);
    println!("{}", ans);
}
