use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }
    let mut dp = vec![0; n];
    dp[0] = 0;
    dp[1] = h[1].abs_diff(h[0]);
    for i in 2..n {
        dp[i] = (dp[i - 1] + h[i].abs_diff(h[i - 1])).min(dp[i - 2] + h[i].abs_diff(h[i - 2]));
    }
    println!("{}", dp[n - 1]);
}
