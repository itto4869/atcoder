use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for i in 1..=n {
        let (wi, vi) = wv[i - 1];
        for k in 0..=w {
            if k < wi {
                dp[i][k] = dp[i - 1][k];
            } else {
                dp[i][k] = dp[i - 1][k].max(dp[i - 1][k - wi] + vi);
            }
        }
    }

    let ans = dp[n][w];
    println!("{}", ans);
}
