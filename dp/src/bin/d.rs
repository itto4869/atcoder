use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        weight: usize,
        wv: [(u64, u64); n],
    }
    let mut dp = vec![vec![0; weight + 1]; n + 1];
    for i in 1..=n {
        let (w, v) = wv[i - 1];
        for j in 1..=weight {
            if w > j as u64 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - w as usize] + v);
            }
        }
    }
    println!("{}", dp[n][weight]);
}
