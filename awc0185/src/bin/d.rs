use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![vec![0; k + 1]; 3000 + 1]; n + 1];
    for l in 1..=k {
        for i in 1..=n {
            let (w, v) = wv[i - 1];
            for j in 0..=c {
                if j < w {
                    dp[i][j][l] = dp[i - 1][j][l];
                } else {
                    dp[i][j][l] = dp[i - 1][j][l].max(dp[i - 1][j - w][l - 1] + v);
                }
            }
        }
    }

    let mut ans = 0;
    for l in 1..=k {
        ans = ans.max(dp[n][c][l]);
    }
    println!("{}", ans);
}
