use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
    }
    let mut wv = Vec::new();
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }
        if b >= t {
            ans += a;
        } else {
            wv.push((c, a));
        }
    }

    let k = wv.len();
    let mut dp = vec![vec![0; m + 1]; k + 1];
    for i in 1..=k {
        let (w, v) = wv[i - 1];
        for j in 1..=m {
            if j < w {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - w] + v);
            }
        }
    }

    ans += dp[k][m];
    println!("{}", ans);
}
