use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [[u64; 3]; n],
    }
    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                } else {
                    dp[i + 1][k] = dp[i + 1][k].max(dp[i][j] + abc[i][k]);
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..3 {
        ans = ans.max(dp[n][i]);
    }
    println!("{}", ans);
}
