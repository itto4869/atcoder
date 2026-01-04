use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut dp = vec![0; n];
    dp[0] = a[0];
    for i in 1..n {
        dp[i] = dp[i - 1] + a[i];
    }

    let mut ans = 0;
    for i in 0..(n - 1) {
        ans += a[i] * (dp[n - 1] - dp[i]);
    }

    println!("{}", ans);
}
