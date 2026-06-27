use ac_library::{Max, Monoid, Segtree};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }
    let mut v = Vec::new();
    let mut dp = vec![vec![0; w]; h];
    v.push('A');
    for i in 0..q {
        input! {
            r: Usize1, 
            c: Usize1,
            x: char,
        }
        v.push(x);
        dp[r][c] = i + 1;
    }

    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == (h - 1) {
                if j == (w - 1) {
                    continue;
                } else {
                    dp[i][j] = dp[i][j].max(dp[i][j + 1]);
                }
            } else {
                if j == (w - 1) {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j]);
                } else {
                    dp[i][j] = dp[i][j].max(dp[i][j + 1]).max(dp[i + 1][j]);
                }
            }
        }
    }

    let mut ans = vec![vec!['A'; w]; h];
    for i in 0..h {
        for j in 0..w {
            let idx = dp[i][j];
            ans[i][j] = v[idx];
        }
    }

    for ans_v in ans {
        println!("{}", ans_v.iter().format(""));
    }
}
