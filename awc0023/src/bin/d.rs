use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
    }
    let mut wv = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            p: usize,
            c: usize,
            w: usize,
        }
        if p > c {
            wv.push((w, p - c));
        }
    }

    let n = wv.len();
    let mut dp = vec![vec![vec![0; s + 1]; n + 1]; n + 1];
    for i in 1..=n {
        let (w, v) = wv[i - 1];
        for j in 1..=i {
            for k in 1..=s {
                if k < w {
                    dp[i][j][k] = dp[i - 1][j][k];
                } else {
                    dp[i][j][k] = dp[i - 1][j][k].max(dp[i - 1][j - 1][k - w] + v);
                }
            }
        }
    }

    let mut ans = -1;
    for j in 1..=n {
        if dp[n][j][s] >= t {
            ans = j as isize;
            break;
        }
    }

    println!("{}", ans);
}
