use std::u64;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut dp = vec![u64::MAX; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        dp[i] = dp[i].min(dp[i - 1] + 1);

        let mut power_of_six = 6;
        while power_of_six <= i {
            dp[i] = dp[i].min(dp[i - power_of_six] + 1);
            power_of_six *= 6;
        }

        let mut power_of_nine = 9;
        while power_of_nine <= i {
            dp[i] = dp[i].min(dp[i - power_of_nine] + 1);
            power_of_nine *= 9;
        }
    }

    println!("{}", dp[n]);
}