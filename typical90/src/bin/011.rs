use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }
    let &max_d = dcs.iter().map(|(d, _, _)| d).max().unwrap();
    dcs.sort_unstable();
    let mut dp = vec![vec![0; n + 1]; 10001];
    for i in 1..=max_d {
        for j in 1..=n {
            let (d, c, s) = dcs[j - 1];
            if i < c {
                dp[i][j] = dp[i][j - 1];
            } else if i <= d {
                dp[i][j] = dp[i][j - 1].max(dp[i - c][j - 1] + s);
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[d][j])
            }
        }
    }

    let ans = dp[max_d][n];
    println!("{}", ans);
}
