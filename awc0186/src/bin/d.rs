use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [usize; n],
        rc: [(usize, usize); m],
    }
    x.sort_unstable();
    let mut dp = vec![1usize << 60; n + 1];
    dp[0] = 0;
    for i in 0..n {
        for &(r, c) in &rc {
            let idx = x.partition_point(|&p| p <= (x[i] + 2 * r));
            dp[idx] = dp[idx].min(dp[i] + c);
        }
    }

    println!("{}", dp[n]);
}
