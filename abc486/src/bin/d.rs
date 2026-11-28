use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        lp: [(usize, usize); n],
    }
    let mut dp = vec![vec![(1usize << 60, 1usize << 60); n + 2]; n + 2];
    dp[0][1].0 = 0;
    dp[0][1].1 = 0;
    for i in 1..=n {
        let (l, p) = lp[i - 1];
        for j in 1..=i {
            if l <= j {
                dp[i][j + 1].0 = (dp[i - 1][j].0 + p).min(dp[i - 1][j].1 + p);
                dp[i][j].1 = dp[i - 1][j].0.min(dp[i - 1][j].1);
            } else {
                dp[i][j].1 = dp[i - 1][j].0.min(dp[i - 1][j].1);
            }
        }
    }

    let mut ans = usize::MAX;
    for j in 0..=(n + 1) {
        ans = ans.min(dp[n][j].0);
    }

    if ans == 1usize << 60 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
