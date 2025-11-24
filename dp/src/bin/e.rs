use proconio::{fastout, input};

const MAX_V: usize = 100 * 1000;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: u64,
        wv: [(u64, usize); n],
    }
    let mut dp = vec![vec![1 << 60; MAX_V + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (w, v) = wv[i];
        for j in 0..=MAX_V {
            if j >= v {
                dp[i + 1][j] = dp[i][j].min(dp[i][j - v] + w);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..=MAX_V {
        if dp[n][i] <= w {
            ans = i;
        }
    }
    println!("{}", ans);
}
