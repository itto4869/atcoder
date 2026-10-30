use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: i64,
        a: [i64; n],
    }
    let mut dp = vec![vec![0; 2]; n + 1];
    dp[1][1] = a[0];
    for i in 2..=n {
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1]);
        dp[i][1] = (dp[i - 1][0] + a[i - 1]).max(dp[i - 1][1] + a[i - 1] + c);
    }

    let ans = dp[n][0].max(dp[n][1]);
    println!("{}", ans);
}
