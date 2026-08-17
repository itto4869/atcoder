use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [[usize; 3]; n],
    }
    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = abc[0][0];
    dp[0][1] = abc[0][1];
    dp[0][2] = abc[0][2];

    for i in 1..n {
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }

                dp[i][j] = dp[i][j].max(dp[i - 1][k] + abc[i][j]);
            }
        }
    }

    let &ans = dp[n - 1].iter().max().unwrap();
    println!("{}", ans);
}
