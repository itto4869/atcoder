use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        whb: [(u64, u64, u64); n],
    }
    let mut total_w = 0;
    let mut total_b = 0;
    for &(w, _, b) in &whb {
        total_w += w;
        total_b += b;
    }
    let cap = total_w / 2;

    let mut dp = vec![vec![0; (cap + 1) as usize]; n + 1];
    for i in 0..n {
        let (w, h, b) = whb[i];
        for c in 0..=cap as usize {
            if c >= w as usize {
                dp[i + 1][c] = dp[i][c].max(dp[i][c - w as usize] + h.saturating_sub(b));
            } else {
                dp[i + 1][c] = dp[i][c];
            }
        }
    }
    let diff = dp[n][cap as usize];
    let ans = total_b + diff;
    println!("{}", ans);
}
