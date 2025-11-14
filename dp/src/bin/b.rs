use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [u64; n],
    }
    let mut dp = vec![1 << 60; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=k.min(n - i - 1) {
            dp[i + j] = dp[i + j].min(dp[i] + h[i + j].abs_diff(h[i]));
        }
    }
    println!("{}", dp[n - 1]);
}
