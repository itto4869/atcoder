use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(i64, usize); n],
    }
    let mut dp = vec![vec![-1i64; m + 1]; n + 1];
    for i in 0..n {
        dp[i][ab[i].1] = ab[i].0;
    }

    for i in 0..n {
        for c in 0..=m {
            if dp[i][c] == -1 {
                continue;
            }

            for j in (i + 1)..n.min(i + k + 1) {
                let (aj, bj) = ab[j];
                let new_cost = c + bj;
                if new_cost <= m {
                    dp[j][new_cost] = dp[j][new_cost].max(dp[i][c] + aj);
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for c in 0..=m {
            if dp[i][c] != -1 {
                ans = ans.max(dp[i][c]);
            }
        }
    }

    println!("{}", ans);
}
