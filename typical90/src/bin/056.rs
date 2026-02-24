use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![(0u128, 0u128); 100000 + 1]; n + 1];
    dp[0][ab[0].0] = (1, 0);
    dp[0][ab[0].1] = (0, 1);
    for i in 1..n {
        let (a, b) = ab[i];
        for j in a.min(b)..=s {
            if j >= a {
                if dp[i - 1][j - a] != (0, 0) {
                    dp[i][j] = ((dp[i - 1][j - a].0 | (1 << i)), dp[i - 1][j - a].1);
                } else if j >= b {
                    if dp[i - 1][j - b] != (0, 0) {
                        dp[i][j] = (dp[i - 1][j - b].0, (dp[i - 1][j - b].1 | (1 << i)));
                    }
                }
            } else if j >= b {
                if dp[i - 1][j - b] != (0, 0) {
                    dp[i][j] = (dp[i - 1][j - b].0, (dp[i - 1][j - b].1 | (1 << i)));
                } else if j >= a {
                    if dp[i - 1][j - a] != (0, 0) {
                        dp[i][j] = ((dp[i - 1][j - a].0 | (1 << i)), dp[i - 1][j - a].1);
                    }
                }
            }
        }
    }
    let ans = dp[n - 1][s];
    if ans == (0, 0) {
        println!("Impossible");
    } else {
        let mut res = String::new();
        for i in 0..n {
            if ans.0 & (1 << i) != 0 {
                res.push('A');
            } else {
                res.push('B');
            }
        }

        println!("{}", res);
    }
}
