use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [usize; n],
    }
    let mut dp = vec![1usize << 60; n];
    dp[0] = 0;
    for i in 0..(n - 1) {
        for j in 1..=(k.min(n - i - 1)) {
            dp[i + j] = dp[i + j].min(dp[i] + h[i].abs_diff(h[i + j]));
        }
    }

    let ans = dp[n - 1];
    println!("{}", ans);
}
