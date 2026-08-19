use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let v = 100000;
    let mut dp = vec![vec![w + 1; v + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        let (wi, vi) = wv[i - 1];
        for k in 0..=v {
            if k < vi {
                dp[i][k] = dp[i - 1][k];
            } else {
                dp[i][k] = dp[i - 1][k].min(dp[i - 1][k - vi] + wi);
            }
        }
    }

    for i in (0..=v).rev() {
        if dp[n][i] <= w {
            println!("{}", i);
            return;
        }
    }
}
