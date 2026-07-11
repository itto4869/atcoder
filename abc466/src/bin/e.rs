use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![vec![0; 2]; k + 1]; n + 1];
    for i in 1..=n {
        for j in 0..=(i.min(k)) {
            let (a, b) = ab[i - 1];
            dp[i][j][0] = dp[i - 1][j][0].max(dp[i - 1][j][1]) + a;

            if j == 0 {
                continue;
            }
            dp[i][j][1] = dp[i - 1][j][1].max(dp[i - 1][j - 1][0]) + b;
        }
    }

    let mut ans = 0;
    for l in 0..=k {
        ans = ans.max(dp[n][l][0].max(dp[n][l][1]));
    }

    println!("{}", ans);
}
