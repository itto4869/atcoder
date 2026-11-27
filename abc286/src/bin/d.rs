use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }
    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;
    for i in 1..=n {
        let (a, b) = ab[i - 1];
        for j in 0..=x {
            for k in 0..=b {
                if (k * a) > j {
                    break;
                }

                if dp[i - 1][j - (k * a)] {
                    dp[i][j] = true;
                    break;
                }
            }
        }
    }

    yes_no!(dp[n][x]);
}
