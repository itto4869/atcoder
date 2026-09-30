use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n],
    }
    let mut dp = vec![vec![1usize << 60; 2 * n + 1]; 2 * n + 1];
    for d in 1..=(2 * n) {
        for l in 1..=(2 * n) {
            let r = l + d;
            if r > (2 * n) {
                break;
            }

            if r == (l + 1) {
                dp[l + 1][r - 1] = 0;
            }
            
            dp[l][r] = dp[l][r].min(dp[l + 1][r - 1] + a[l - 1].abs_diff(a[r - 1]));
            for k in (l + 1)..r {
                dp[l][r] = dp[l][r].min(dp[l][k] + dp[k + 1][r]);
            }
        }
    }

    let ans = dp[1][2 * n];
    println!("{}", ans);
}
